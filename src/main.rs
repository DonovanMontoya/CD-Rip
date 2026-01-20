use crate::cli::{Cli, Commands};
use clap::Parser;

mod cli;
mod makeflac;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Rip { path } => {
            println!("Rip: output path: {:?}", path);
        }
        Commands::View { media } => {
            println!("View: device: {:?}", media);
        }
        Commands::Makeflac {
            path,
            output,
            delete,
        } => {
            println!("MakeFlac: path: {:?}, delete: {:?}", path, delete);
        }
    }
}
