use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "sjt", version, about = "Software jobs for today")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Show today's ranked tech jobs from the local store (run sync first)
    Today {
        /// Max rows to print
        #[arg(short, long, default_value_t = 10)]
        limit: usize,
    },
    /// Pull jobs from Himalayas into .data/jobs.json
    Sync,
    /// View or update the local skills profile
    Profile {
        #[command(subcommand)]
        action: ProfileCmd,
    },
}

#[derive(Subcommand)]
pub enum ProfileCmd {
    /// Print the saved profile
    Show,
    /// Replace profile skills (comma-separated)
    Set {
        #[arg(long, value_delimiter = ',', num_args = 1..)]
        skills: Vec<String>,
    },
}
