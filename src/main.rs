use anyhow::Result;
use clap::Parser;

mod archiver;
mod cli;
mod commands;
mod config;
mod graveyard;
mod scanner;
mod utils;

use cli::{Cli, Commands};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Scan => {
            println!("🕷️  Scanning Inbox for dusty files...");
            commands::scan_inbox()?;
        }
        Commands::Review => {
            println!("🔍 Starting interactive review...");
            commands::interactive_review()?;
        }
        Commands::Archive { item, all, note } => {
            if all {
                println!("🪦 Archiving all eligible files...");
                commands::archive_all_with_note(note.as_deref())?;
            } else if let Some(item) = item {
                println!("🪦 Archiving {item}...");
                commands::archive_item_with_note(&item, note.as_deref())?;
            } else {
                println!("Please specify either --all or an item to archive");
            }
        }
        Commands::Resurrect { pattern } => {
            println!("🧟 Searching for '{pattern}' in the Graveyard...");
            commands::resurrect_files(&pattern)?;
        }
        Commands::Search { pattern } => {
            println!("🔍 Searching for '{pattern}' in the Graveyard...");
            commands::search_graveyard(&pattern)?;
        }
        Commands::Config => {
            println!("⚙️  Configuration:");
            commands::show_config()?;
        }
    }

    Ok(())
}
