use anyhow::{Context, Result};
use chrono::{DateTime, Duration, Utc};
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::config::{Config, NotificationType};

#[derive(Debug, Clone)]
pub struct StaleItem {
    pub path: PathBuf,
    pub name: String,
    pub last_modified: DateTime<Utc>,
    pub is_directory: bool,
    pub age_days: i64,
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
        if !self.config.inbox.exists() {
            return Ok(vec![]);
        }

        let mut stale_items = Vec::new();
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

                    stale_items.push(StaleItem {
                        path: path.clone(),
                        name,
                        last_modified,
                        is_directory: path.is_dir(),
                        age_days,
                    });
                }
            }
        }

        Ok(stale_items)
    }

    fn get_last_modified_time(&self, path: &Path) -> Result<Option<DateTime<Utc>>> {
        if path.is_file() {
            let metadata = fs::metadata(path).context("Failed to get file metadata")?;
            let modified = metadata
                .modified()
                .context("Failed to get modification time")?;
            Ok(Some(DateTime::from(modified)))
        } else if path.is_dir() {
            let mut latest_time: Option<DateTime<Utc>> = None;

            for entry in WalkDir::new(path)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| e.file_type().is_file())
            {
                if let Ok(metadata) = entry.metadata() {
                    if let Ok(modified) = metadata.modified() {
                        let modified_dt = DateTime::from(modified);
                        if latest_time.is_none() || Some(modified_dt) > latest_time {
                            latest_time = Some(modified_dt);
                        }
                    }
                }
            }

            if latest_time.is_none() {
                let metadata = fs::metadata(path).context("Failed to get directory metadata")?;
                let modified = metadata
                    .modified()
                    .context("Failed to get modification time")?;
                latest_time = Some(DateTime::from(modified));
            }

            Ok(latest_time)
        } else {
            Ok(None)
        }
    }

    pub fn display_scan_results(&self, stale_items: &[StaleItem]) {
        if stale_items.is_empty() {
            println!("✨ No dusty items found in your Inbox! All clean and tidy.");
            self.send_notification(
                "Relfa Scan Complete",
                "No dusty items found in your Inbox! All clean and tidy. ✨",
            );
            return;
        }

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
            println!("   {}", item.display());
        }

        println!("\n💡 Run 'relfa review' to interactively deal with these items,");
        println!("   or 'relfa archive --all' to archive them all to the Graveyard.");

        // Send desktop notification
        let notification_text = format!(
            "{} {} in Inbox {} gathering dust. Consider reviewing them!",
            stale_items.len(),
            plural,
            if stale_items.len() == 1 { "is" } else { "are" }
        );
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
