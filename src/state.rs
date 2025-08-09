use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NotificationState {
    // Map from file path (relative to inbox) to notification count
    #[serde(default)]
    pub notification_counts: HashMap<String, u32>,
}

impl NotificationState {
    pub fn load() -> Result<Self> {
        let state_path = Self::state_path();

        if state_path.exists() {
            let content = fs::read_to_string(&state_path)
                .context("Failed to read notification state file")?;

            toml::from_str(&content).context("Failed to parse notification state file")
        } else {
            Ok(Self::default())
        }
    }

    pub fn save(&self) -> Result<()> {
        let state_path = Self::state_path();

        if let Some(parent) = state_path.parent() {
            fs::create_dir_all(parent).context("Failed to create state directory")?;
        }

        let content =
            toml::to_string_pretty(self).context("Failed to serialize notification state")?;

        fs::write(&state_path, content).context("Failed to write notification state file")?;

        Ok(())
    }

    fn state_path() -> PathBuf {
        // Try XDG_STATE_HOME first (defaults to ~/.local/state)
        if let Ok(state_home) = std::env::var("XDG_STATE_HOME") {
            PathBuf::from(state_home)
                .join("relfa")
                .join("notification_state.toml")
        } else if let Some(home_dir) = dirs::home_dir() {
            // Use the XDG default location
            home_dir
                .join(".local")
                .join("state")
                .join("relfa")
                .join("notification_state.toml")
        } else {
            // Fallback to same directory as config file
            dirs::config_dir()
                .map(|d| d.join("relfa").join("notification_state.toml"))
                .unwrap_or_else(|| PathBuf::from("relfa").join("notification_state.toml"))
        }
    }

    pub fn get_notification_count(&self, file_name: &str) -> u32 {
        self.notification_counts
            .get(file_name)
            .copied()
            .unwrap_or(0)
    }

    pub fn increment_notification_count(&mut self, file_name: &str) {
        let count = self
            .notification_counts
            .entry(file_name.to_string())
            .or_insert(0);
        *count += 1;
    }

    pub fn reset_notification_count(&mut self, file_name: &str) {
        self.notification_counts.remove(file_name);
    }

    pub fn retain_only_files(&mut self, current_files: &std::collections::HashSet<String>) {
        // Keep only the files that are in the current_files set
        // This removes entries for files that are no longer stale
        self.notification_counts
            .retain(|file_name, _| current_files.contains(file_name));
    }
}
