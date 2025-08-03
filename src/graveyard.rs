use anyhow::Result;
use std::io::{self, Write};
use std::path::PathBuf;
use walkdir::WalkDir;

use crate::config::Config;
use crate::utils::copy_file_or_dir;

pub struct GraveyardManager {
    config: Config,
}

impl GraveyardManager {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn resurrect_files(&self, pattern: &str) -> Result<()> {
        let matches = self.find_in_graveyard(pattern)?;
        
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
            let dest = self.config.inbox.join(filename);
            
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
                    let dest = self.config.inbox.join(filename);
                    
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

    pub fn search_files(&self, pattern: &str) -> Result<()> {
        let matches = self.find_in_graveyard(pattern)?;
        
        if matches.is_empty() {
            println!("💀 No files found matching '{}' in the Graveyard", pattern);
            return Ok(());
        }
        
        println!("Found {} matches in the Graveyard:", matches.len());
        for path in matches {
            // Show relative path from graveyard root
            if let Ok(relative) = path.strip_prefix(&self.config.graveyard) {
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

    fn find_in_graveyard(&self, pattern: &str) -> Result<Vec<PathBuf>> {
        let mut matches = Vec::new();
        
        if !self.config.graveyard.exists() {
            return Ok(matches);
        }
        
        for entry in WalkDir::new(&self.config.graveyard)
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
}