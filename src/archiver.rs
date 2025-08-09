use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use std::fs;
use std::path::{Path, PathBuf};

use crate::config::Config;
use crate::scanner::StaleItem;
use crate::state::NotificationState;

pub struct Archiver {
    config: Config,
}

#[derive(Debug)]
pub struct ArchivedItem {}

impl Archiver {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn archive_item_with_note(
        &self,
        item: &StaleItem,
        note: Option<&str>,
    ) -> Result<ArchivedItem> {
        let now = Utc::now();

        let created_time = self.get_creation_time(&item.path)?;
        let modified_time = item.last_modified;
        let archived_time = now;

        // Find all subdirs that need original files
        let mut original_subdirs = self.find_original_subdirs()?;

        // Set proper times for each subdir
        for (subdir_name, time) in original_subdirs.iter_mut() {
            if self.config.path_format.created_subdir.get_name() == Some(subdir_name) {
                *time = created_time;
            } else if self.config.path_format.modified_subdir.get_name() == Some(subdir_name) {
                *time = modified_time;
            } else if self.config.path_format.archived_subdir.get_name() == Some(subdir_name) {
                *time = archived_time;
            }
        }

        let mut primary_path = None;
        let mut created_paths = std::collections::HashMap::new();

        // Create original files in all required subdirs
        for (i, (subdir_name, time)) in original_subdirs.iter().enumerate() {
            let target_path = self.create_path_for_subdir(subdir_name, &item.name, *time)?;
            self.ensure_directory_exists(target_path.parent().unwrap())?;

            if i == 0 {
                // Move the original file to the first location
                self.move_item(&item.path, &target_path)
                    .context("Failed to move item to graveyard")?;
                primary_path = Some(target_path.clone());
            } else {
                // Copy to additional locations
                if item.path.is_dir() {
                    copy_dir_all(primary_path.as_ref().unwrap(), &target_path)?;
                } else {
                    fs::copy(primary_path.as_ref().unwrap(), &target_path)
                        .context("Failed to copy item to additional location")?;
                }
            }

            created_paths.insert(subdir_name.clone(), target_path.clone());
            println!("🪦 Stored '{}' in: {}", item.name, target_path.display());
        }

        // Create symlinks for any remaining enabled subdirs
        self.create_remaining_symlinks(
            &item.name,
            &created_paths,
            created_time,
            modified_time,
            archived_time,
        )?;

        // Save epitaphs if provided - create them in all relevant subdirs following same logic as files
        if let Some(note_text) = note {
            self.save_epitaphs_with_logic(
                &item.name,
                &created_paths,
                note_text,
                &created_time,
                &modified_time,
                &archived_time,
            )?;
        }

        // Reset notification count since file was archived
        let mut state = NotificationState::load().unwrap_or_default();
        state.reset_notification_count(&item.name);
        state.save()?;

        println!(
            "✅ Archived '{}' to {} locations",
            item.name,
            original_subdirs.len()
        );
        if note.is_some() {
            println!("📝 Epitaph saved with the archived item");
        }

        Ok(ArchivedItem {})
    }

    fn get_creation_time(&self, path: &Path) -> Result<DateTime<Utc>> {
        let metadata = fs::metadata(path).context("Failed to get metadata")?;

        if let Ok(created) = metadata.created() {
            Ok(DateTime::from(created))
        } else {
            Ok(DateTime::from(metadata.modified()?))
        }
    }

    fn find_original_subdirs(&self) -> Result<Vec<(String, DateTime<Utc>)>> {
        let mut originals = Vec::new();
        let now = Utc::now();

        if self.config.path_format.created_subdir.is_original() {
            originals.push((
                self.config
                    .path_format
                    .created_subdir
                    .get_name()
                    .unwrap()
                    .to_string(),
                // Use creation time for created subdir - we'll set this properly later
                now,
            ));
        }
        if self.config.path_format.modified_subdir.is_original() {
            originals.push((
                self.config
                    .path_format
                    .modified_subdir
                    .get_name()
                    .unwrap()
                    .to_string(),
                // Use modification time for modified subdir - we'll set this properly later
                now,
            ));
        }
        if self.config.path_format.archived_subdir.is_original() {
            originals.push((
                self.config
                    .path_format
                    .archived_subdir
                    .get_name()
                    .unwrap()
                    .to_string(),
                now, // archived time
            ));
        }

        if originals.is_empty() {
            return Err(anyhow::anyhow!(
                "No subdir configured to store original files"
            ));
        }

        Ok(originals)
    }

    fn create_path_for_subdir(
        &self,
        subdir_name: &str,
        name: &str,
        time: DateTime<Utc>,
    ) -> Result<PathBuf> {
        let date_path = self.config.format_date_path(&time);

        let mut path = self
            .config
            .graveyard
            .join(subdir_name)
            .join(date_path)
            .join(name);

        path = self.ensure_unique_name(path)?;
        Ok(path)
    }

    fn get_path_for_subdir(&self, subdir_name: &str, name: &str, time: DateTime<Utc>) -> PathBuf {
        let date_path = self.config.format_date_path(&time);

        self.config
            .graveyard
            .join(subdir_name)
            .join(date_path)
            .join(name)
    }

    fn create_remaining_symlinks(
        &self,
        name: &str,
        _created_paths: &std::collections::HashMap<String, PathBuf>,
        created_time: DateTime<Utc>,
        modified_time: DateTime<Utc>,
        archived_time: DateTime<Utc>,
    ) -> Result<()> {
        let subdirs = [
            (
                "created",
                &self.config.path_format.created_subdir,
                created_time,
            ),
            (
                "modified",
                &self.config.path_format.modified_subdir,
                modified_time,
            ),
            (
                "archived",
                &self.config.path_format.archived_subdir,
                archived_time,
            ),
        ];

        for (_subdir_type, subdir_config, time) in subdirs {
            if !subdir_config.is_enabled() || subdir_config.is_original() {
                continue; // Skip disabled subdirs and those that already have originals
            }

            let subdir_name = subdir_config.get_name().unwrap();

            if let Some(target_subdir) = subdir_config.get_target() {
                // Always create the path for the immediate target, not resolved chain
                let target_time = if self.config.path_format.created_subdir.get_name()
                    == Some(target_subdir)
                {
                    created_time
                } else if self.config.path_format.modified_subdir.get_name() == Some(target_subdir)
                {
                    modified_time
                } else if self.config.path_format.archived_subdir.get_name() == Some(target_subdir)
                {
                    archived_time
                } else {
                    time
                };

                let target_path = self.get_path_for_subdir(target_subdir, name, target_time);
                let link_path = self.create_path_for_subdir(subdir_name, name, time)?;

                self.ensure_directory_exists(link_path.parent().unwrap())?;
                self.create_symlink(&target_path, &link_path)?;
                println!(
                    "🔗 Created symlink '{}' -> {}",
                    link_path.display(),
                    target_path.display()
                );
            }
        }

        Ok(())
    }

    fn ensure_unique_name(&self, mut path: PathBuf) -> Result<PathBuf> {
        if !path.exists() {
            return Ok(path);
        }

        let original_name = path
            .file_name()
            .context("Invalid path")?
            .to_string_lossy()
            .to_string();

        let mut counter = 1;
        loop {
            let new_name = if let Some(dot_pos) = original_name.rfind('.') {
                format!(
                    "{}_{}{}",
                    &original_name[..dot_pos],
                    counter,
                    &original_name[dot_pos..]
                )
            } else {
                format!("{original_name}_{counter}")
            };

            path.set_file_name(new_name);

            if !path.exists() {
                break;
            }

            counter += 1;
        }

        Ok(path)
    }

    fn save_epitaphs_with_logic(
        &self,
        name: &str,
        created_paths: &std::collections::HashMap<String, PathBuf>,
        note: &str,
        created_time: &DateTime<Utc>,
        modified_time: &DateTime<Utc>,
        archived_time: &DateTime<Utc>,
    ) -> Result<()> {
        // Create epitaph content once
        let epitaph_content = format!(
            "# Epitaph for {}\n\
            # Archived: {}\n\
            # Created: {}\n\
            # Modified: {}\n\
            # Hostname: {}\n\
            \n\
            {}",
            name,
            archived_time.format("%Y-%m-%d %H:%M:%S UTC"),
            created_time.format("%Y-%m-%d %H:%M:%S UTC"),
            modified_time.format("%Y-%m-%d %H:%M:%S UTC"),
            self.config.get_hostname(),
            note
        );

        let epitaph_filename = format!("{name}.epitaph");
        let mut primary_epitaph_path = None;
        let mut created_epitaph_paths = std::collections::HashMap::new();

        // First, create epitaphs for all original file locations
        for (subdir_name, file_path) in created_paths {
            let epitaph_path = file_path.parent().unwrap().join(&epitaph_filename);

            if primary_epitaph_path.is_none() {
                // Write the first epitaph
                fs::write(&epitaph_path, &epitaph_content)
                    .context("Failed to write epitaph file")?;
                primary_epitaph_path = Some(epitaph_path.clone());
                println!("📄 Epitaph written to: {}", epitaph_path.display());
            } else {
                // Copy epitaph to additional original locations
                fs::copy(primary_epitaph_path.as_ref().unwrap(), &epitaph_path)
                    .context("Failed to copy epitaph file")?;
                println!("📄 Epitaph copied to: {}", epitaph_path.display());
            }

            created_epitaph_paths.insert(subdir_name.clone(), epitaph_path);
        }

        // Now create epitaph symlinks for symlink subdirs, following same logic as files
        let subdirs = [
            (
                "created",
                &self.config.path_format.created_subdir,
                *created_time,
            ),
            (
                "modified",
                &self.config.path_format.modified_subdir,
                *modified_time,
            ),
            (
                "archived",
                &self.config.path_format.archived_subdir,
                *archived_time,
            ),
        ];

        for (_subdir_type, subdir_config, time) in subdirs {
            if !subdir_config.is_enabled() || subdir_config.is_original() {
                continue; // Skip disabled subdirs and those that already have originals
            }

            let subdir_name = subdir_config.get_name().unwrap();

            if let Some(target_subdir) = subdir_config.get_target() {
                // Get the target time for path resolution
                let target_time = if self.config.path_format.created_subdir.get_name()
                    == Some(target_subdir)
                {
                    *created_time
                } else if self.config.path_format.modified_subdir.get_name() == Some(target_subdir)
                {
                    *modified_time
                } else if self.config.path_format.archived_subdir.get_name() == Some(target_subdir)
                {
                    *archived_time
                } else {
                    time
                };

                let target_epitaph_path =
                    self.get_path_for_subdir(target_subdir, &epitaph_filename, target_time);
                let link_epitaph_path =
                    self.create_path_for_subdir(subdir_name, &epitaph_filename, time)?;

                self.ensure_directory_exists(link_epitaph_path.parent().unwrap())?;
                self.create_symlink(&target_epitaph_path, &link_epitaph_path)?;
                println!(
                    "🔗 Created epitaph symlink '{}' -> {}",
                    link_epitaph_path.display(),
                    target_epitaph_path.display()
                );
            }
        }

        Ok(())
    }

    fn move_item(&self, src: &Path, dst: &Path) -> Result<()> {
        // Try rename first (faster for same filesystem)
        match fs::rename(src, dst) {
            Ok(()) => Ok(()),
            Err(e) if e.raw_os_error() == Some(18) => {
                // Error 18 is "Invalid cross-device link" - use copy + remove instead
                if src.is_dir() {
                    copy_dir_all(src, dst)?;
                    self.remove_dir_with_permissions(src)
                        .context("Failed to remove source directory after copy")?;
                } else {
                    fs::copy(src, dst).context("Failed to copy file across devices")?;
                    fs::remove_file(src).context("Failed to remove source file after copy")?;
                }
                Ok(())
            }
            Err(e) => Err(e.into()),
        }
    }

    fn remove_dir_with_permissions(&self, path: &Path) -> Result<()> {
        // First try normal removal
        match fs::remove_dir_all(path) {
            Ok(()) => Ok(()),
            Err(_) => {
                // If that fails, try to fix permissions recursively and then remove
                Self::fix_permissions_recursive(path)?;
                fs::remove_dir_all(path)
                    .context("Failed to remove directory even after fixing permissions")
            }
        }
    }

    fn fix_permissions_recursive(path: &Path) -> Result<()> {
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            // Make the directory writable
            if path.is_dir() {
                let mut perms = fs::metadata(path)?.permissions();
                perms.set_mode(perms.mode() | 0o700); // Add owner write/execute permissions
                fs::set_permissions(path, perms)?;

                // Recursively fix permissions for contents
                if let Ok(entries) = fs::read_dir(path) {
                    for entry in entries.flatten() {
                        let entry_path = entry.path();
                        if entry_path.is_dir() {
                            Self::fix_permissions_recursive(&entry_path)?;
                        } else {
                            // Make files writable
                            if let Ok(metadata) = fs::metadata(&entry_path) {
                                let mut perms = metadata.permissions();
                                perms.set_mode(perms.mode() | 0o600); // Add owner read/write permissions
                                let _ = fs::set_permissions(&entry_path, perms);
                                // Ignore errors for broken symlinks
                            }
                        }
                    }
                }
            }
        }
        #[cfg(windows)]
        {
            // On Windows, try to remove read-only attribute
            let _ = fs::metadata(path).and_then(|metadata| {
                let mut perms = metadata.permissions();
                perms.set_readonly(false);
                fs::set_permissions(path, perms)
            });
        }
        Ok(())
    }

    fn ensure_directory_exists(&self, path: &Path) -> Result<()> {
        fs::create_dir_all(path).context("Failed to create directory")
    }

    fn create_symlink(&self, target: &Path, link: &Path) -> Result<()> {
        // Calculate relative path from link to target to avoid cross-device issues
        let relative_target = self.calculate_relative_path(link.parent().unwrap(), target)?;

        #[cfg(unix)]
        {
            std::os::unix::fs::symlink(&relative_target, link)
                .context("Failed to create symlink")?;
        }

        #[cfg(windows)]
        {
            if target.is_dir() {
                std::os::windows::fs::symlink_dir(&relative_target, link)
                    .context("Failed to create directory symlink")?;
            } else {
                std::os::windows::fs::symlink_file(&relative_target, link)
                    .context("Failed to create file symlink")?;
            }
        }

        Ok(())
    }

    fn calculate_relative_path(&self, from_dir: &Path, to_path: &Path) -> Result<PathBuf> {
        let from_abs = from_dir
            .canonicalize()
            .unwrap_or_else(|_| from_dir.to_path_buf());
        let to_abs = to_path
            .canonicalize()
            .unwrap_or_else(|_| to_path.to_path_buf());

        // Find common ancestor
        let from_components: Vec<_> = from_abs.components().collect();
        let to_components: Vec<_> = to_abs.components().collect();

        let common_len = from_components
            .iter()
            .zip(to_components.iter())
            .take_while(|(a, b)| a == b)
            .count();

        // Build relative path
        let mut relative_path = PathBuf::new();

        // Add ".." for each directory we need to go up from the common ancestor
        for _ in common_len..from_components.len() {
            relative_path.push("..");
        }

        // Add the path components from common ancestor to target
        for component in &to_components[common_len..] {
            relative_path.push(component.as_os_str());
        }

        // Handle the case where we're in the same directory
        if relative_path.as_os_str().is_empty() {
            relative_path.push(to_path.file_name().unwrap());
        }

        Ok(relative_path)
    }
}

fn copy_dir_all(src: &Path, dst: &Path) -> Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if ty.is_dir() {
            copy_dir_all(&src_path, &dst_path)?;
        } else if ty.is_symlink() {
            // Handle symlinks by copying the symlink itself, not following it
            #[cfg(unix)]
            {
                if let Ok(target) = fs::read_link(&src_path) {
                    std::os::unix::fs::symlink(&target, &dst_path)
                        .context("Failed to copy symlink")?;
                }
            }
            #[cfg(windows)]
            {
                // On Windows, try to copy as file/dir symlink based on target
                if let Ok(target) = fs::read_link(&src_path) {
                    if target.is_dir() {
                        std::os::windows::fs::symlink_dir(&target, &dst_path)
                            .context("Failed to copy directory symlink")?;
                    } else {
                        std::os::windows::fs::symlink_file(&target, &dst_path)
                            .context("Failed to copy file symlink")?;
                    }
                }
            }
        } else {
            fs::copy(&src_path, &dst_path).context("Failed to copy file")?;
        }
    }
    Ok(())
}
