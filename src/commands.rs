use anyhow::Result;
use chrono::{DateTime, Utc};
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::io::{self, Write};

use crate::archiver::Archiver;
use crate::config::Config;
use crate::graveyard::GraveyardManager;
use crate::scanner::{Scanner, StaleItem};
use crate::state::NotificationState;
use crate::utils::{delete_item, open_file_with_default, touch_item, view_file_with_pager};

fn get_single_keypress() -> Result<char> {
    enable_raw_mode()?;

    let key = loop {
        if let Event::Key(key_event) = event::read()? {
            let ch = match key_event.code {
                KeyCode::Char(c) => c,
                KeyCode::Enter => '\n',
                KeyCode::Esc => '\x1b',
                _ => continue,
            };
            break ch;
        }
    };

    disable_raw_mode()?;
    println!(); // Move to next line after keypress
    Ok(key)
}

pub fn scan_inbox() -> Result<()> {
    let config = Config::load_without_save()?;
    let scanner = Scanner::new(config);
    // Use the version that tracks notifications
    let stale_items = scanner.scan_inbox_with_notification_tracking()?;
    scanner.display_scan_results(&stale_items);
    Ok(())
}

pub fn interactive_review() -> Result<()> {
    let config = Config::load_without_save()?;
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

        // Show help automatically for the first file
        if i == 0 {
            println!("\n📚 Available actions:");
            println!("  a - Archive: Move the file to the graveyard");
            println!("  n - Note+archive: Archive with an epitaph (descriptive note)");
            println!(
                "  t - Touch: Update modification time to keep for another {} days",
                config.age_threshold_days
            );
            println!("  d - Delete: Permanently delete the file (requires confirmation)");
            println!(
                "  v - View: Preview file content using pager ({})",
                config.pager
            );
            println!("  o - Open: Open file with default application");
            println!("  s - Skip: Skip this file and move to the next");
            println!("  q - Quit: Exit the review session");
            println!("  ? - Help: Show this help message\n");
        }

        loop {
            print!("Action [a/n/t/d/v/o/s/q/?]: ");
            io::stdout().flush()?;

            let input = get_single_keypress()?.to_lowercase().next().unwrap_or('\0');

            match input {
                'a' => {
                    archiver.archive_item_with_note(item, None)?;
                    archived_count += 1;
                    break;
                }
                'n' => {
                    // Temporarily disable raw mode for multi-line input
                    disable_raw_mode()?;
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
                't' => {
                    touch_item(&item.path)?;
                    // Reset notification count since file was touched
                    let mut state = NotificationState::load().unwrap_or_default();
                    state.reset_notification_count(&item.name);
                    state.save()?;
                    println!("✨ Updated modification time for '{}' - file will be kept for another {} days", 
                             item.name, config.age_threshold_days);
                    skipped_count += 1; // Count as skipped since we're keeping it
                    break;
                }
                'd' => {
                    // Temporarily disable raw mode for confirmation input
                    disable_raw_mode()?;
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
                        // Reset notification count since file was deleted
                        let mut state = NotificationState::load().unwrap_or_default();
                        state.reset_notification_count(&item.name);
                        state.save()?;
                        println!("🗑️  Permanently deleted '{}'", item.name);
                        archived_count += 1; // Count as "processed"
                        break;
                    } else {
                        println!("❌ Delete cancelled - name did not match exactly");
                        continue; // Show the prompt again
                    }
                }
                'v' => {
                    // Temporarily disable raw mode for pager
                    disable_raw_mode()?;
                    view_file_with_pager(&item.path, &config)?;
                    continue; // Continue the loop to show the prompt again
                }
                'o' => {
                    open_file_with_default(&item.path)?;
                    continue; // Continue the loop to show the prompt again
                }
                's' => {
                    println!("⏭️  Skipped '{}'", item.name);
                    skipped_count += 1;
                    break;
                }
                'q' => {
                    println!("\n🛑 Review cancelled.");
                    println!("📊 Summary: {archived_count} processed, {skipped_count} skipped");
                    return Ok(());
                }
                '?' => {
                    println!("\n📚 Available actions:");
                    println!("  a - Archive: Move the file to the graveyard");
                    println!("  n - Note+archive: Archive with an epitaph (descriptive note)");
                    println!(
                        "  t - Touch: Update modification time to keep for another {} days",
                        config.age_threshold_days
                    );
                    println!("  d - Delete: Permanently delete the file (requires confirmation)");
                    println!(
                        "  v - View: Preview file content using pager ({})",
                        config.pager
                    );
                    println!("  o - Open: Open file with default application");
                    println!("  s - Skip: Skip this file and move to the next");
                    println!("  q - Quit: Exit the review session");
                    println!("  ? - Help: Show this help message\n");
                    continue;
                }
                _ => {
                    println!("Invalid key '{input}'. Press '?' for help.");
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
    let config = Config::load_without_save()?;
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
    let config = Config::load_without_save()?;
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

    // Get notification count for this item
    let state = NotificationState::load().unwrap_or_default();
    let notification_count = state.get_notification_count(item_name);

    let item = StaleItem {
        path: inbox_path.clone(),
        name: item_name.to_string(),
        last_modified,
        is_directory: inbox_path.is_dir(),
        age_days,
        notification_count,
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
    let config = Config::load_without_save()?;
    let graveyard = GraveyardManager::new(config);
    graveyard.resurrect_files(pattern)
}

pub fn search_graveyard(pattern: &str) -> Result<()> {
    let config = Config::load_without_save()?;
    let graveyard = GraveyardManager::new(config);
    graveyard.search_files(pattern)
}

pub fn auto_archive_eligible_files(note: Option<&str>) -> Result<()> {
    let config = Config::load_without_save()?;
    let scanner = Scanner::new(config.clone());
    let archiver = Archiver::new(config.clone());

    let auto_archive_items = scanner.scan_auto_archive_eligible()?;

    if auto_archive_items.is_empty() {
        println!(
            "✨ No files eligible for auto-archiving found - all files are within the auto-archive threshold of {} days!",
            config.auto_archive_threshold_days
        );
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
    let config_path = if let Some(config_dir) = dirs::config_dir() {
        config_dir.join("relfa").join("config.toml")
    } else {
        std::path::PathBuf::from(".relfa.toml")
    };

    let config = if config_path.exists() {
        // Config exists, try to load it and report any parsing errors
        match Config::load_without_save() {
            Ok(config) => config,
            Err(e) => {
                eprintln!("❌ Error parsing config file at {}:", config_path.display());
                eprintln!("   {e}");
                eprintln!("\n💡 The config file exists but has invalid format.");
                eprintln!("   Either fix the syntax or delete the file to regenerate defaults.");
                return Err(e);
            }
        }
    } else {
        // Config doesn't exist, create it
        Config::load()?
    };
    println!("{}", config.display());
    println!(
        "\nConfig file location: {}",
        dirs::config_dir()
            .map(|d| d.join("relfa").join("config.toml").display().to_string())
            .unwrap_or_else(|| ".relfa.toml".to_string())
    );
    Ok(())
}
