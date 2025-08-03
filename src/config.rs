use anyhow::{Context, Result};
use dirs::home_dir;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::fs;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub inbox: PathBuf,
    pub graveyard: PathBuf,
    pub age_threshold_days: u32,
    pub notification: NotificationType,
    pub path_format: PathFormatConfig,
    #[serde(default = "default_pager")]
    pub pager: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PathFormatConfig {
    pub created_subdir: SubdirConfig,
    pub modified_subdir: SubdirConfig,
    pub archived_subdir: SubdirConfig,
    pub date_format: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum SubdirConfig {
    #[serde(rename = "original")]
    Original { name: String },
    #[serde(rename = "symlink")]
    Symlink { name: String, target: String },
    #[serde(rename = "nothing")]
    Nothing,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum NotificationType {
    Cli,
    Desktop,
}

fn default_pager() -> String {
    std::env::var("PAGER").unwrap_or_else(|_| "less".to_string())
}

impl Default for Config {
    fn default() -> Self {
        let home = home_dir().unwrap_or_else(|| PathBuf::from("."));

        Self {
            inbox: home.join("Inbox"),
            graveyard: home.join("Graveyard"),
            age_threshold_days: 14,
            notification: NotificationType::Cli,
            path_format: PathFormatConfig::default(),
            pager: default_pager(),
        }
    }
}

impl Default for PathFormatConfig {
    fn default() -> Self {
        Self {
            created_subdir: SubdirConfig::Original { name: "created".to_string() },
            modified_subdir: SubdirConfig::Symlink {
                name: "modified".to_string(),
                target: "created".to_string(),
            },
            archived_subdir: SubdirConfig::Symlink {
                name: "archived".to_string(),
                target: "created".to_string(),
            },
            date_format: "{hostname}/{year}/{month:02}/{day:02}".to_string(),
        }
    }
}

impl SubdirConfig {
    pub fn display(&self) -> String {
        match self {
            SubdirConfig::Original { name } => format!("{} (original)", name),
            SubdirConfig::Symlink { name, target } => format!("{} (symlink -> {})", name, target),
            SubdirConfig::Nothing => "disabled".to_string(),
        }
    }

    pub fn is_enabled(&self) -> bool {
        !matches!(self, SubdirConfig::Nothing)
    }

    pub fn get_name(&self) -> Option<&str> {
        match self {
            SubdirConfig::Original { name } | SubdirConfig::Symlink { name, .. } => Some(name),
            SubdirConfig::Nothing => None,
        }
    }

    pub fn is_original(&self) -> bool {
        matches!(self, SubdirConfig::Original { .. })
    }

    pub fn get_target(&self) -> Option<&str> {
        match self {
            SubdirConfig::Symlink { target, .. } => Some(target),
            _ => None,
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_path = Self::config_path();
        
        if config_path.exists() {
            let content = fs::read_to_string(&config_path)
                .context("Failed to read config file")?;
            
            // Try to parse, but if it fails due to missing fields, use defaults and save
            match toml::from_str::<Self>(&content) {
                Ok(config) => Ok(config),
                Err(_) => {
                    // Config format has changed, use defaults and save new format
                    let config = Self::default();
                    config.save()?;
                    Ok(config)
                }
            }
        } else {
            let config = Self::default();
            config.save()?;
            Ok(config)
        }
    }

    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_path();
        
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)
                .context("Failed to create config directory")?;
        }

        let content = toml::to_string_pretty(self)
            .context("Failed to serialize config")?;
        
        fs::write(&config_path, content)
            .context("Failed to write config file")?;

        Ok(())
    }

    fn config_path() -> PathBuf {
        if let Some(config_dir) = dirs::config_dir() {
            config_dir.join("relfa").join("config.toml")
        } else {
            PathBuf::from(".relfa.toml")
        }
    }

    pub fn get_hostname(&self) -> String {
        gethostname::gethostname()
            .to_string_lossy()
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '-' || *c == '_')
            .collect()
    }

    pub fn format_date_path(&self, date: &chrono::DateTime<chrono::Utc>) -> String {
        use chrono::Datelike;
        
        self.path_format.date_format
            .replace("{hostname}", &self.get_hostname())
            .replace("{year}", &date.year().to_string())
            .replace("{month:02}", &format!("{:02}", date.month()))
            .replace("{day:02}", &format!("{:02}", date.day()))
            .replace("{month}", &date.month().to_string())
            .replace("{day}", &date.day().to_string())
    }

    pub fn display(&self) -> String {
        format!(
            "📂 Inbox: {}\n🪦 Graveyard: {}\n⏰ Age threshold: {} days\n🖥️  Hostname: {}\n🔔 Notifications: {:?}\n📄 Pager: {}\n📁 Path format:\n   Created: {}\n   Modified: {}\n   Archived: {}\n   Date format: {}",
            self.inbox.display(),
            self.graveyard.display(),
            self.age_threshold_days,
            self.get_hostname(),
            self.notification,
            self.pager,
            self.path_format.created_subdir.display(),
            self.path_format.modified_subdir.display(),
            self.path_format.archived_subdir.display(),
            self.path_format.date_format
        )
    }
}