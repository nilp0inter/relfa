use anyhow::{Context, Result};
use std::path::Path;

use crate::config::Config;

pub fn view_file_with_pager(file_path: &Path, config: &Config) -> Result<()> {
    if !file_path.exists() {
        println!("❌ File not found: {}", file_path.display());
        return Ok(());
    }

    if file_path.is_dir() {
        println!(
            "📁 '{}' is a directory. Use 'o' to open with file manager.",
            file_path.display()
        );
        return Ok(());
    }

    let status = std::process::Command::new(&config.pager)
        .arg(file_path)
        .status()
        .context(format!("Failed to execute pager: {}", config.pager))?;

    if !status.success() {
        println!("⚠️  Pager exited with error code: {:?}", status.code());
    }

    Ok(())
}

pub fn open_file_with_default(file_path: &Path) -> Result<()> {
    if !file_path.exists() {
        println!("❌ File not found: {}", file_path.display());
        return Ok(());
    }

    #[cfg(target_os = "linux")]
    let command = "xdg-open";
    #[cfg(target_os = "macos")]
    let command = "open";
    #[cfg(target_os = "windows")]
    let command = "start";

    let status = std::process::Command::new(command)
        .arg(file_path)
        .status()
        .context(format!("Failed to execute {command}"))?;

    if status.success() {
        println!(
            "🚀 Opened '{}' with default application",
            file_path.display()
        );
    } else {
        println!(
            "⚠️  Failed to open file with default application (exit code: {:?})",
            status.code()
        );
    }

    Ok(())
}

pub fn delete_item(path: &Path) -> Result<()> {
    if path.is_file() {
        std::fs::remove_file(path).context(format!("Failed to delete file: {}", path.display()))?;
    } else if path.is_dir() {
        std::fs::remove_dir_all(path)
            .context(format!("Failed to delete directory: {}", path.display()))?;
    } else {
        return Err(anyhow::anyhow!(
            "Path does not exist or is not a file/directory: {}",
            path.display()
        ));
    }
    Ok(())
}

pub fn touch_item(path: &Path) -> Result<()> {
    use std::time::SystemTime;

    let now = SystemTime::now();

    if path.exists() {
        // Update both access and modification times to current time
        filetime::set_file_times(
            path,
            filetime::FileTime::from_system_time(now),
            filetime::FileTime::from_system_time(now),
        )
        .context(format!(
            "Failed to update timestamps for: {}",
            path.display()
        ))?;
    } else {
        return Err(anyhow::anyhow!("Path does not exist: {}", path.display()));
    }

    Ok(())
}

pub fn copy_file_or_dir(source: &Path, dest: &Path) -> Result<()> {
    if source.is_file() {
        if let Some(parent) = dest.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::copy(source, dest)?;
    } else if source.is_dir() {
        copy_dir_recursive(source, dest)?;
    }
    Ok(())
}

fn copy_dir_recursive(source: &Path, dest: &Path) -> Result<()> {
    std::fs::create_dir_all(dest)?;

    for entry in std::fs::read_dir(source)? {
        let entry = entry?;
        let source_path = entry.path();
        let dest_path = dest.join(entry.file_name());

        if source_path.is_file() {
            std::fs::copy(&source_path, &dest_path)?;
        } else if source_path.is_dir() {
            copy_dir_recursive(&source_path, &dest_path)?;
        }
    }

    Ok(())
}
