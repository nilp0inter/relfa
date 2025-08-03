use clap::{Parser, Subcommand};
use anyhow::{Context, Result};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use chrono::{DateTime, Utc};

mod config;
mod scanner;
mod archiver;

use config::Config;
use scanner::Scanner;
use archiver::Archiver;

#[derive(Parser)]
#[command(
    name = "relfa",
    version,
    about = "Your gentle digital gravedigger 🪦",
    long_about = "Relfa helps you keep your computer's clutter under control by monitoring your Inbox folder, nudging you to review old files, and lovingly archiving them in a dust-covered, cobwebby digital Graveyard."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Scan Inbox for stale files")]
    Scan,
    #[command(about = "Interactively review and archive files")]
    Review,
    #[command(about = "Archive files to the Graveyard")]
    Archive {
        #[arg(help = "Specific file or folder to archive")]
        item: Option<String>,
        #[arg(long, help = "Archive all eligible files")]
        all: bool,
        #[arg(long, help = "Add an epitaph (note) explaining why this was archived")]
        note: Option<String>,
    },
    #[command(about = "Resurrect files from the Graveyard back to Inbox (copies, doesn't remove)")]
    Resurrect {
        #[arg(help = "Pattern to search for in the graveyard")]
        pattern: String,
    },
    #[command(about = "Search for files in the Graveyard")]
    Search {
        #[arg(help = "Pattern to search for")]
        pattern: String,
    },
    #[command(about = "Show or edit configuration")]
    Config,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Scan => {
            println!("🕷️  Scanning Inbox for dusty files...");
            scan_inbox()?;
        }
        Commands::Review => {
            println!("🔍 Starting interactive review...");
            interactive_review()?;
        }
        Commands::Archive { item, all, note } => {
            if all {
                println!("🪦 Archiving all eligible files...");
                archive_all_with_note(note.as_deref())?;
            } else if let Some(item) = item {
                println!("🪦 Archiving {}...", item);
                archive_item_with_note(&item, note.as_deref())?;
            } else {
                println!("Please specify either --all or an item to archive");
            }
        }
        Commands::Resurrect { pattern } => {
            println!("🧟 Searching for '{}' in the Graveyard...", pattern);
            resurrect_files(&pattern)?;
        }
        Commands::Search { pattern } => {
            println!("🔍 Searching for '{}' in the Graveyard...", pattern);
            search_graveyard(&pattern)?;
        }
        Commands::Config => {
            println!("⚙️  Configuration:");
            show_config()?;
        }
    }

    Ok(())
}

fn scan_inbox() -> Result<()> {
    let config = Config::load()?;
    let scanner = Scanner::new(config);
    let stale_items = scanner.scan_inbox()?;
    scanner.display_scan_results(&stale_items);
    Ok(())
}

fn interactive_review() -> Result<()> {
    let config = Config::load()?;
    let scanner = Scanner::new(config.clone());
    let archiver = Archiver::new(config.clone());
    
    let stale_items = scanner.scan_inbox()?;
    
    if stale_items.is_empty() {
        println!("✨ No items to review - Inbox is clean!");
        return Ok(());
    }
    
    println!("Found {} stale items for review:\n", stale_items.len());
    
    let mut archived_count = 0;
    let mut skipped_count = 0;
    
    for (i, item) in stale_items.iter().enumerate() {
        println!("[{}/{}] {}", i + 1, stale_items.len(), item.display());
        
        loop {
            print!("Action: (a)rchive, (n)ote+archive, (d)elete, (v)iew, (o)pen, (s)kip, (q)uit? ");
            io::stdout().flush()?;
            
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let input = input.trim().to_lowercase();
            
            match input.as_str() {
                "a" | "archive" => {
                    archiver.archive_item_with_note(item, None)?;
                    archived_count += 1;
                    break;
                }
                "n" | "note" => {
                    print!("📝 Enter epitaph note (why archive this?): ");
                    io::stdout().flush()?;
                    
                    let mut note = String::new();
                    io::stdin().read_line(&mut note)?;
                    let note = note.trim();
                    
                    if note.is_empty() {
                        archiver.archive_item_with_note(item, None)?;
                    } else {
                        archiver.archive_item_with_note(item, Some(note))?;
                    }
                    archived_count += 1;
                    break;
                }
                "d" | "delete" => {
                    println!("⚠️  WARNING: This will permanently delete '{}' and cannot be undone!", item.name);
                    print!("🔒 To confirm, please type the exact name '{}': ", item.name);
                    io::stdout().flush()?;
                    
                    let mut confirmation = String::new();
                    io::stdin().read_line(&mut confirmation)?;
                    let confirmation = confirmation.trim();
                    
                    if confirmation == item.name {
                        delete_item(&item.path)?;
                        println!("🗑️  Permanently deleted '{}'", item.name);
                        archived_count += 1; // Count as "processed"
                        break;
                    } else {
                        println!("❌ Delete cancelled - name did not match exactly");
                        continue; // Show the prompt again
                    }
                }
                "v" | "view" => {
                    view_file_with_pager(&item.path, &config)?;
                    continue; // Continue the loop to show the prompt again
                }
                "o" | "open" => {
                    open_file_with_default(&item.path)?;
                    continue; // Continue the loop to show the prompt again
                }
                "s" | "skip" => {
                    println!("⏭️  Skipped '{}'", item.name);
                    skipped_count += 1;
                    break;
                }
                "q" | "quit" => {
                    println!("\n🛑 Review cancelled.");
                    println!("📊 Summary: {} processed, {} skipped", archived_count, skipped_count);
                    return Ok(());
                }
                _ => {
                    println!("Please enter 'a' for archive, 'n' for note+archive, 'd' for delete, 'v' for view, 'o' for open, 's' for skip, or 'q' to quit");
                    continue;
                }
            }
        }
        
        println!(); // Empty line for readability
    }
    
    println!("🎉 Review complete!");
    println!("📊 Summary: {} processed, {} skipped", archived_count, skipped_count);
    Ok(())
}

fn archive_all_with_note(note: Option<&str>) -> Result<()> {
    let config = Config::load()?;
    let scanner = Scanner::new(config.clone());
    let archiver = Archiver::new(config);
    
    let stale_items = scanner.scan_inbox()?;
    
    if stale_items.is_empty() {
        println!("✨ No items to archive - Inbox is clean!");
        return Ok(());
    }
    
    println!("Found {} stale items to archive:", stale_items.len());
    for item in &stale_items {
        archiver.archive_item_with_note(item, note)?;
    }
    
    println!("\n🎉 Successfully archived {} items to the Graveyard!", stale_items.len());
    Ok(())
}

fn archive_item_with_note(item_name: &str, note: Option<&str>) -> Result<()> {
    let config = Config::load()?;
    let scanner = Scanner::new(config.clone());
    let archiver = Archiver::new(config.clone());
    
    // First, check if the file exists in the Inbox at all
    let inbox_path = config.inbox.join(item_name);
    if !inbox_path.exists() {
        println!("❌ Item '{}' not found in Inbox", item_name);
        return Ok(());
    }
    
    // Check if it's among the stale items (for potential warning)
    let stale_items = scanner.scan_inbox()?;
    let is_stale = stale_items.iter().any(|i| i.name == item_name);
    
    // Create a StaleItem for this file regardless of age
    let metadata = std::fs::metadata(&inbox_path)?;
    let last_modified = if let Ok(modified) = metadata.modified() {
        DateTime::from(modified)
    } else {
        Utc::now()
    };
    
    let age_days = (Utc::now() - last_modified).num_days();
    let item = crate::scanner::StaleItem {
        path: inbox_path.clone(),
        name: item_name.to_string(),
        last_modified,
        is_directory: inbox_path.is_dir(),
        age_days,
    };
    
    // Show warning if file is not stale
    if !is_stale {
        println!("⚠️  Warning: '{}' is only {} days old (threshold: {} days)", 
                 item_name, age_days, config.age_threshold_days);
        println!("📦 Archiving anyway as explicitly requested...");
    }
    
    archiver.archive_item_with_note(&item, note)?;
    println!("🎉 Successfully archived '{}'!", item_name);
    
    Ok(())
}


fn resurrect_files(pattern: &str) -> Result<()> {
    let config = Config::load()?;
    let matches = find_in_graveyard(&config, pattern)?;
    
    if matches.is_empty() {
        println!("💀 No files found matching '{}' in the Graveyard", pattern);
        return Ok(());
    }
    
    println!("Found {} matches:", matches.len());
    for (i, path) in matches.iter().enumerate() {
        println!("  {}. {}", i + 1, path.display());
    }
    
    if matches.len() == 1 {
        // Auto-resurrect single match
        let source = &matches[0];
        let filename = source.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown");
        let dest = config.inbox.join(filename);
        
        copy_file_or_dir(source, &dest)?;
        println!("🧟‍♂️ Resurrected '{}' to Inbox!", filename);
    } else {
        // Interactive selection for multiple matches
        print!("\nWhich file to resurrect? (1-{}, or 'q' to quit): ", matches.len());
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();
        
        if input == "q" {
            println!("Resurrection cancelled.");
            return Ok(());
        }
        
        if let Ok(choice) = input.parse::<usize>() {
            if choice > 0 && choice <= matches.len() {
                let source = &matches[choice - 1];
                let filename = source.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown");
                let dest = config.inbox.join(filename);
                
                copy_file_or_dir(source, &dest)?;
                println!("🧟‍♂️ Resurrected '{}' to Inbox!", filename);
            } else {
                println!("Invalid choice.");
            }
        } else {
            println!("Invalid input.");
        }
    }
    
    Ok(())
}

fn search_graveyard(pattern: &str) -> Result<()> {
    let config = Config::load()?;
    let matches = find_in_graveyard(&config, pattern)?;
    
    if matches.is_empty() {
        println!("💀 No files found matching '{}' in the Graveyard", pattern);
        return Ok(());
    }
    
    println!("Found {} matches in the Graveyard:", matches.len());
    for path in matches {
        // Show relative path from graveyard root
        if let Ok(relative) = path.strip_prefix(&config.graveyard) {
            println!("  📄 {}", relative.display());
        } else {
            println!("  📄 {}", path.display());
        }
        
        // Check for epitaph and show if it contains the search pattern
        let epitaph_path = if path.is_file() {
            path.with_extension(format!("{}.epitaph", path.extension().and_then(|s| s.to_str()).unwrap_or("txt")))
        } else {
            path.join(".epitaph")
        };
        
        if epitaph_path.exists() {
            if let Ok(epitaph_content) = std::fs::read_to_string(&epitaph_path) {
                // Extract just the note part (after the header)
                if let Some(note_start) = epitaph_content.find("\n\n") {
                    let note = epitaph_content[note_start + 2..].trim();
                    if !note.is_empty() {
                        // Check if this match was found because of epitaph content
                        let filename_matches = path.file_name()
                            .and_then(|f| f.to_str())
                            .map(|f| f.contains(pattern))
                            .unwrap_or(false);
                        
                        if !filename_matches && epitaph_content.to_lowercase().contains(&pattern.to_lowercase()) {
                            println!("     💭 \"{}\" 🔍", note);
                        } else {
                            println!("     💭 \"{}\"", note);
                        }
                    }
                }
            }
        }
    }
    
    Ok(())
}

fn find_in_graveyard(config: &Config, pattern: &str) -> Result<Vec<PathBuf>> {
    let mut matches = Vec::new();
    
    if !config.graveyard.exists() {
        return Ok(matches);
    }
    
    for entry in walkdir::WalkDir::new(&config.graveyard)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file() || (e.file_type().is_dir() && e.depth() > 0))
    {
        let mut found_match = false;
        
        // Check filename match
        if let Some(filename) = entry.file_name().to_str() {
            if filename.contains(pattern) && !filename.ends_with(".epitaph") {
                found_match = true;
            }
        }
        
        // Check epitaph content match (only for non-epitaph files)
        if !found_match && !entry.file_name().to_str().unwrap_or("").ends_with(".epitaph") {
            let epitaph_path = if entry.file_type().is_file() {
                entry.path().with_extension(format!("{}.epitaph", 
                    entry.path().extension().and_then(|s| s.to_str()).unwrap_or("txt")))
            } else {
                entry.path().join(".epitaph")
            };
            
            if epitaph_path.exists() {
                if let Ok(epitaph_content) = std::fs::read_to_string(&epitaph_path) {
                    if epitaph_content.to_lowercase().contains(&pattern.to_lowercase()) {
                        found_match = true;
                    }
                }
            }
        }
        
        if found_match {
            matches.push(entry.path().to_path_buf());
        }
    }
    
    // Deduplicate - prefer files in the "created" subdir (originals over symlinks)
    matches.sort_by(|a, b| {
        let a_is_created = a.to_string_lossy().contains("/created/");
        let b_is_created = b.to_string_lossy().contains("/created/");
        match (a_is_created, b_is_created) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.cmp(b),
        }
    });
    
    // Remove duplicates (same filename)
    let mut deduped = Vec::new();
    let mut seen_names = std::collections::HashSet::new();
    
    for path in matches {
        if let Some(filename) = path.file_name() {
            if seen_names.insert(filename.to_os_string()) {
                deduped.push(path);
            }
        }
    }
    
    Ok(deduped)
}

fn copy_file_or_dir(source: &Path, dest: &Path) -> Result<()> {
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

fn view_file_with_pager(file_path: &Path, config: &Config) -> Result<()> {
    if !file_path.exists() {
        println!("❌ File not found: {}", file_path.display());
        return Ok(());
    }
    
    if file_path.is_dir() {
        println!("📁 '{}' is a directory. Use 'o' to open with file manager.", file_path.display());
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

fn open_file_with_default(file_path: &Path) -> Result<()> {
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
        .context(format!("Failed to execute {}", command))?;
    
    if status.success() {
        println!("🚀 Opened '{}' with default application", file_path.display());
    } else {
        println!("⚠️  Failed to open file with default application (exit code: {:?})", status.code());
    }
    
    Ok(())
}

fn delete_item(path: &Path) -> Result<()> {
    if path.is_file() {
        std::fs::remove_file(path)
            .context(format!("Failed to delete file: {}", path.display()))?;
    } else if path.is_dir() {
        std::fs::remove_dir_all(path)
            .context(format!("Failed to delete directory: {}", path.display()))?;
    } else {
        return Err(anyhow::anyhow!("Path does not exist or is not a file/directory: {}", path.display()));
    }
    Ok(())
}

fn show_config() -> Result<()> {
    let config = Config::load()?;
    println!("{}", config.display());
    println!("\nConfig file location: {}", 
        dirs::config_dir()
            .map(|d| d.join("relfa").join("config.toml").display().to_string())
            .unwrap_or_else(|| ".relfa.toml".to_string())
    );
    Ok(())
}
