use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use std::fs;
use std::path::{Path, PathBuf};

use crate::config::Config;
use crate::scanner::StaleItem;

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
                fs::rename(&item.path, &target_path).context("Failed to move item to graveyard")?;
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

    fn ensure_directory_exists(&self, path: &Path) -> Result<()> {
        fs::create_dir_all(path).context("Failed to create directory")
    }

    fn create_symlink(&self, target: &Path, link: &Path) -> Result<()> {
        #[cfg(unix)]
        {
            std::os::unix::fs::symlink(target, link).context("Failed to create symlink")?;
        }

        #[cfg(windows)]
        {
            if target.is_dir() {
                std::os::windows::fs::symlink_dir(target, link)
                    .context("Failed to create directory symlink")?;
            } else {
                std::os::windows::fs::symlink_file(target, link)
                    .context("Failed to create file symlink")?;
            }
        }

        Ok(())
    }
}

fn copy_dir_all(src: &Path, dst: &Path) -> Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(&entry.path(), &dst.join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.join(entry.file_name()))?;
        }
    }
    Ok(())
}
