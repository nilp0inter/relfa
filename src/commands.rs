use anyhow::Result;
use chrono::{DateTime, Utc};
use std::io::{self, Write};

use crate::archiver::Archiver;
use crate::config::Config;
use crate::graveyard::GraveyardManager;
use crate::scanner::{Scanner, StaleItem};
use crate::utils::{delete_item, open_file_with_default, view_file_with_pager};

pub fn scan_inbox() -> Result<()> {
    let config = Config::load()?;
    let scanner = Scanner::new(config);
    let stale_items = scanner.scan_inbox()?;
    scanner.display_scan_results(&stale_items);
    Ok(())
}

pub fn interactive_review() -> Result<()> {
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
                    println!(
                        "⚠️  WARNING: This will permanently delete '{}' and cannot be undone!",
                        item.name
                    );
                    print!(
                        "🔒 To confirm, please type the exact name '{}': ",
                        item.name
                    );
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
                    println!("📊 Summary: {archived_count} processed, {skipped_count} skipped");
                    return Ok(());
                }
                _ => {
                    println!(
                        "Please enter 'a' for archive, 'n' for note+archive, 'd' for delete, 'v' for view, 'o' for open, 's' for skip, or 'q' to quit"
                    );
                    continue;
                }
            }
        }

        println!(); // Empty line for readability
    }

    println!("🎉 Review complete!");
    println!("📊 Summary: {archived_count} processed, {skipped_count} skipped");
    Ok(())
}

pub fn archive_all_with_note(note: Option<&str>) -> Result<()> {
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

    println!(
        "\n🎉 Successfully archived {} items to the Graveyard!",
        stale_items.len()
    );
    Ok(())
}

pub fn archive_item_with_note(item_name: &str, note: Option<&str>) -> Result<()> {
    let config = Config::load()?;
    let scanner = Scanner::new(config.clone());
    let archiver = Archiver::new(config.clone());

    // First, check if the file exists in the Inbox at all
    let inbox_path = config.inbox.join(item_name);
    if !inbox_path.exists() {
        println!("❌ Item '{item_name}' not found in Inbox");
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
    let item = StaleItem {
        path: inbox_path.clone(),
        name: item_name.to_string(),
        last_modified,
        is_directory: inbox_path.is_dir(),
        age_days,
    };

    // Show warning if file is not stale
    if !is_stale {
        println!(
            "⚠️  Warning: '{}' is only {} days old (threshold: {} days)",
            item_name, age_days, config.age_threshold_days
        );
        println!("📦 Archiving anyway as explicitly requested...");
    }

    archiver.archive_item_with_note(&item, note)?;
    println!("🎉 Successfully archived '{item_name}'!");

    Ok(())
}

pub fn resurrect_files(pattern: &str) -> Result<()> {
    let config = Config::load()?;
    let graveyard = GraveyardManager::new(config);
    graveyard.resurrect_files(pattern)
}

pub fn search_graveyard(pattern: &str) -> Result<()> {
    let config = Config::load()?;
    let graveyard = GraveyardManager::new(config);
    graveyard.search_files(pattern)
}

pub fn auto_archive_eligible_files(note: Option<&str>) -> Result<()> {
    let config = Config::load()?;
    let scanner = Scanner::new(config.clone());
    let archiver = Archiver::new(config.clone());

    let auto_archive_items = scanner.scan_auto_archive_eligible()?;

    if auto_archive_items.is_empty() {
        println!("✨ No files eligible for auto-archiving found - all files are within the auto-archive threshold of {} days!", config.auto_archive_threshold_days);
        return Ok(());
    }

    println!(
        "Found {} {} exceeding the auto-archive threshold of {} days:",
        auto_archive_items.len(),
        if auto_archive_items.len() == 1 {
            "file"
        } else {
            "files"
        },
        config.auto_archive_threshold_days
    );

    for item in &auto_archive_items {
        println!("   {}", item.display());
    }

    println!("\n🤖 Auto-archiving these files...");
    for item in &auto_archive_items {
        archiver.archive_item_with_note(item, note)?;
    }

    println!(
        "\n🎉 Successfully auto-archived {} {} to the Graveyard!",
        auto_archive_items.len(),
        if auto_archive_items.len() == 1 {
            "file"
        } else {
            "files"
        }
    );
    Ok(())
}

pub fn show_config() -> Result<()> {
    let config = Config::load()?;
    println!("{}", config.display());
    println!(
        "\nConfig file location: {}",
        dirs::config_dir()
            .map(|d| d.join("relfa").join("config.toml").display().to_string())
            .unwrap_or_else(|| ".relfa.toml".to_string())
    );
    Ok(())
}
