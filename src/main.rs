mod cli;
mod job;
mod profile;

use crate::cli::{Cli, Commands};
use clap::Parser;

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Today => println!("No jobs yet."),
        // _ => println!("Something went wrong."),
    }
}
