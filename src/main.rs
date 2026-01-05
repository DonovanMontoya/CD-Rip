use crate::cli::{Cli, Commands};
use clap::Parser;

mod cli;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Rip { path } => {
            println!("Rip: output path: {:?}", path);
        }
        Commands::View { media } => {
            println!("View: device: {:?}", media);
        }
        Commands::Makeflac { path } => {
            println!("MakeFlac: path: {:?}", path);
        }
    }
}
