use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "sjt", version, about = "Software jobs for today")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Today,
}
