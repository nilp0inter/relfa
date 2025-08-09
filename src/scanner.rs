use anyhow::{Context, Result};
use chrono::{DateTime, Duration, Utc};
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::config::{Config, NotificationType};
use crate::state::NotificationState;

#[derive(Debug, Clone)]
pub struct StaleItem {
    pub path: PathBuf,
    pub name: String,
    pub last_modified: DateTime<Utc>,
    pub is_directory: bool,
    pub age_days: i64,
    pub notification_count: u32,
}

impl StaleItem {
    pub fn display(&self) -> String {
        let item_type = if self.is_directory { "folder" } else { "file" };
        format!(
            "📄 \"{}\" ({}) - last touched {} days ago ({})",
            self.name,
            item_type,
            self.age_days,
            self.last_modified.format("%Y-%m-%d")
        )
    }
}

pub struct Scanner {
    config: Config,
}

impl Scanner {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn scan_inbox(&self) -> Result<Vec<StaleItem>> {
        self.scan_inbox_with_state(false)
    }

    pub fn scan_inbox_with_notification_tracking(&self) -> Result<Vec<StaleItem>> {
        self.scan_inbox_with_state(true)
    }

    fn scan_inbox_with_state(&self, track_notifications: bool) -> Result<Vec<StaleItem>> {
        if !self.config.inbox.exists() {
            return Ok(vec![]);
        }

        let mut state = if track_notifications {
            Some(NotificationState::load().unwrap_or_default())
        } else {
            None
        };

        let mut stale_items = Vec::new();
        let mut currently_stale_files = std::collections::HashSet::new();
        let cutoff_date = Utc::now() - Duration::days(self.config.age_threshold_days as i64);

        for entry in fs::read_dir(&self.config.inbox).context("Failed to read inbox directory")? {
            let entry = entry.context("Failed to read directory entry")?;
            let path = entry.path();

            if let Some(last_modified) = self.get_last_modified_time(&path)? {
                if last_modified < cutoff_date {
                    let age_days = (Utc::now() - last_modified).num_days();
                    let name = path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("unknown")
                        .to_string();

                    // Track that this file is currently stale
                    currently_stale_files.insert(name.clone());

                    let notification_count = if let Some(ref mut state) = state {
                        // Get current count and increment it
                        let current_count = state.get_notification_count(&name);
                        state.increment_notification_count(&name);
                        current_count + 1 // Return what the count will be after this scan
                    } else {
                        // When not tracking, just load state to get current count
                        NotificationState::load()
                            .unwrap_or_default()
                            .get_notification_count(&name)
                    };

                    stale_items.push(StaleItem {
                        path: path.clone(),
                        name,
                        last_modified,
                        is_directory: path.is_dir(),
                        age_days,
                        notification_count,
                    });
                }
            }
        }

        // Save the updated state if we were tracking
        if let Some(mut state) = state {
            // Clean up entries for files that are no longer stale
            // This includes files that were deleted, modified, or are now younger than threshold
            state.retain_only_files(&currently_stale_files);
            state.save()?;
        }

        Ok(stale_items)
    }

    pub fn scan_auto_archive_eligible(&self) -> Result<Vec<StaleItem>> {
        if !self.config.inbox.exists() {
            return Ok(vec![]);
        }

        let state = NotificationState::load().unwrap_or_default();
        let mut auto_archive_items = Vec::new();
        let cutoff_date =
            Utc::now() - Duration::days(self.config.auto_archive_threshold_days as i64);

        for entry in fs::read_dir(&self.config.inbox).context("Failed to read inbox directory")? {
            let entry = entry.context("Failed to read directory entry")?;
            let path = entry.path();

            if let Some(last_modified) = self.get_last_modified_time(&path)? {
                if last_modified < cutoff_date {
                    let age_days = (Utc::now() - last_modified).num_days();
                    let name = path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("unknown")
                        .to_string();

                    let notification_count = state.get_notification_count(&name);

                    // Only eligible if it has been notified enough times
                    if notification_count >= self.config.auto_archive_min_scans {
                        auto_archive_items.push(StaleItem {
                            path: path.clone(),
                            name,
                            last_modified,
                            is_directory: path.is_dir(),
                            age_days,
                            notification_count,
                        });
                    }
                }
            }
        }

        Ok(auto_archive_items)
    }

    // New method to get items that will be eligible after more scans
    pub fn scan_pending_auto_archive(&self) -> Result<Vec<StaleItem>> {
        if !self.config.inbox.exists() {
            return Ok(vec![]);
        }

        let state = NotificationState::load().unwrap_or_default();
        let mut pending_items = Vec::new();
        let cutoff_date =
            Utc::now() - Duration::days(self.config.auto_archive_threshold_days as i64);

        for entry in fs::read_dir(&self.config.inbox).context("Failed to read inbox directory")? {
            let entry = entry.context("Failed to read directory entry")?;
            let path = entry.path();

            if let Some(last_modified) = self.get_last_modified_time(&path)? {
                if last_modified < cutoff_date {
                    let age_days = (Utc::now() - last_modified).num_days();
                    let name = path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("unknown")
                        .to_string();

                    let notification_count = state.get_notification_count(&name);

                    // Pending if old enough but not notified enough times yet
                    if notification_count < self.config.auto_archive_min_scans {
                        pending_items.push(StaleItem {
                            path: path.clone(),
                            name,
                            last_modified,
                            is_directory: path.is_dir(),
                            age_days,
                            notification_count,
                        });
                    }
                }
            }
        }

        Ok(pending_items)
    }

    fn get_last_modified_time(&self, path: &Path) -> Result<Option<DateTime<Utc>>> {
        if path.is_file() {
            let metadata = fs::metadata(path).context("Failed to get file metadata")?;
            let modified = metadata
                .modified()
                .context("Failed to get modification time")?;
            Ok(Some(DateTime::from(modified)))
        } else if path.is_dir() {
            // Start with the directory's own modification time
            let metadata = fs::metadata(path).context("Failed to get directory metadata")?;
            let dir_modified = metadata
                .modified()
                .context("Failed to get modification time")?;
            let mut latest_time = DateTime::from(dir_modified);

            // Then check all files and subdirectories recursively
            for entry in WalkDir::new(path)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| e.file_type().is_file() || e.file_type().is_dir())
            {
                // Skip the root directory itself to avoid double-counting
                if entry.path() == path {
                    continue;
                }

                if let Ok(metadata) = entry.metadata() {
                    if let Ok(modified) = metadata.modified() {
                        let modified_dt = DateTime::from(modified);
                        if modified_dt > latest_time {
                            latest_time = modified_dt;
                        }
                    }
                }
            }

            Ok(Some(latest_time))
        } else {
            Ok(None)
        }
    }

    pub fn display_scan_results(&self, stale_items: &[StaleItem]) {
        // Check for auto-archive eligible and pending items
        let auto_archive_items = self.scan_auto_archive_eligible().unwrap_or_default();
        let pending_items = self.scan_pending_auto_archive().unwrap_or_default();

        if stale_items.is_empty() && auto_archive_items.is_empty() && pending_items.is_empty() {
            println!("✨ No dusty items found in your Inbox! All clean and tidy.");
            self.send_notification(
                "Relfa Scan Complete",
                "No dusty items found in your Inbox! All clean and tidy. ✨",
            );
            return;
        }

        // Display stale items (regular threshold)
        if !stale_items.is_empty() {
            let plural = if stale_items.len() == 1 {
                "item"
            } else {
                "items"
            };
            let message = format!(
                "☠️  {} {} in ~/Inbox {} gathering dust:",
                stale_items.len(),
                plural,
                if stale_items.len() == 1 { "is" } else { "are" }
            );

            println!("{message}");

            for item in stale_items {
                let scan_info = if item.notification_count > 0 {
                    format!(" [seen {} times]", item.notification_count)
                } else {
                    String::new()
                };
                println!("   {}{}", item.display(), scan_info);
            }
        }

        // Display pending auto-archive items
        if !pending_items.is_empty() {
            if !stale_items.is_empty() {
                println!();
            }

            println!(
                "⏳ {} {} old enough for auto-archiving but {} more scans:",
                pending_items.len(),
                if pending_items.len() == 1 {
                    "item"
                } else {
                    "items"
                },
                if pending_items.len() == 1 {
                    "needs"
                } else {
                    "need"
                }
            );

            for item in &pending_items {
                let scans_needed = self.config.auto_archive_min_scans - item.notification_count;
                println!(
                    "   {} [needs {} more {}]",
                    item.display(),
                    scans_needed,
                    if scans_needed == 1 { "scan" } else { "scans" }
                );
            }
        }

        // Display auto-archive eligible items
        if !auto_archive_items.is_empty() {
            let auto_plural = if auto_archive_items.len() == 1 {
                "item"
            } else {
                "items"
            };

            if !stale_items.is_empty() || !pending_items.is_empty() {
                println!();
            }

            println!(
                "🤖 {} {} {} eligible for auto-archiving NOW:",
                auto_archive_items.len(),
                auto_plural,
                if auto_archive_items.len() == 1 {
                    "is"
                } else {
                    "are"
                }
            );

            for item in &auto_archive_items {
                println!(
                    "   {} [notified {} times]",
                    item.display(),
                    item.notification_count
                );
            }

            println!(
                "   ⚠️  These will be automatically archived if you run 'relfa archive' without arguments!"
            );
        }

        println!("\n💡 Run 'relfa review' to interactively deal with these items");
        if !stale_items.is_empty() && !auto_archive_items.is_empty() {
            println!(
                "   or 'relfa archive' to auto-archive old files (or 'relfa archive --all' for all)."
            );
        } else if !stale_items.is_empty() {
            println!("   or 'relfa archive --all' to archive them all to the Graveyard.");
        } else if !auto_archive_items.is_empty() {
            println!("   or 'relfa archive' to auto-archive old files.");
        }

        // Send desktop notification
        let total_items = stale_items.len() + auto_archive_items.len();
        let notification_text = if !auto_archive_items.is_empty() {
            format!(
                "{} items need attention in Inbox. {} are eligible for auto-archiving!",
                total_items,
                auto_archive_items.len()
            )
        } else {
            format!(
                "{} {} in Inbox {} gathering dust. Consider reviewing them!",
                stale_items.len(),
                if stale_items.len() == 1 {
                    "item"
                } else {
                    "items"
                },
                if stale_items.len() == 1 { "is" } else { "are" }
            )
        };
        self.send_notification("Digital Clutter Detected", &notification_text);
    }

    fn send_notification(&self, title: &str, body: &str) {
        if matches!(self.config.notification, NotificationType::Desktop) {
            #[cfg(not(target_os = "windows"))]
            {
                if let Err(e) = notify_rust::Notification::new()
                    .summary(title)
                    .body(body)
                    .icon("folder")
                    .timeout(notify_rust::Timeout::Milliseconds(5000))
                    .show()
                {
                    eprintln!("Failed to send desktop notification: {e}");
                }
            }

            #[cfg(target_os = "windows")]
            {
                if let Err(e) = notify_rust::Notification::new()
                    .summary(title)
                    .body(body)
                    .timeout(notify_rust::Timeout::Milliseconds(5000))
                    .show()
                {
                    eprintln!("Failed to send desktop notification: {}", e);
                }
            }
        }
    }
}
