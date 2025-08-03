use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "relfa",
    version,
    about = "Your gentle digital gravedigger 🪦",
    long_about = "Relfa helps you keep your computer's clutter under control by monitoring your Inbox folder, nudging you to review old files, and lovingly archiving them in a dust-covered, cobwebby digital Graveyard."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
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
