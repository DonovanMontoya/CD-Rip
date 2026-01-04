use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "cdrip")]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Locate and Rip aiff files from CD
    Rip {
        /// Output Path( /Desktop/CDName )
        #[arg(short, long)]
        path: PathBuf,
    },

    /// Shows contents of CD
    View {
        /// Show contents for the specific cd Dir
        #[arg(short, long)]
        media: Option<String>,
    },
}
