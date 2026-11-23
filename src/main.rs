mod cli;
mod job;
mod profile;

use anyhow::{Context, Result};
use chrono::Utc;
use clap::Parser;

use crate::cli::{Cli, Commands, ProfileCmd};
use crate::job::{load_fixture_jobs, rank_today};
use crate::profile::Profile;

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Today { limit } => run_today(limit)?,
        Commands::Profile { action } => run_profile(action)?,
    }
    Ok(())
}

fn run_today(limit: usize) -> Result<()> {
    let path = Profile::default_path();
    let profile = Profile::load(&path)?;
    let now = Utc::now();
    let jobs = load_fixture_jobs(now).context("failed to load job fixtures")?;
    let ranked = rank_today(&jobs, &profile, now);

    if ranked.is_empty() {
        println!("No matching jobs for today.");
        if profile.skills.is_empty() {
            println!("Hint: set skills with `sjt profile set --skills rust,python`");
        }
        return Ok(());
    }

    println!(
        "{:<4} {:<8} {:<28} {:<16} {}",
        "ID", "SCORE", "TITLE", "COMPANY", "URL"
    );
    for (job, score) in ranked.into_iter().take(limit) {
        println!(
            "{:<4} {:<8.2} {:<28} {:<16} {}",
            job.id,
            score,
            truncate(&job.title, 28),
            truncate(&job.company, 16),
            job.url
        );
    }
    Ok(())
}

fn run_profile(action: ProfileCmd) -> Result<()> {
    let path = Profile::default_path();
    match action {
        ProfileCmd::Show => {
            let profile = Profile::load(&path)?;
            if profile.skills.is_empty() {
                println!("(empty profile — no skills set)");
                println!("path: {}", path.display());
            } else {
                println!("skills: {}", profile.skills.join(", "));
                println!("path: {}", path.display());
            }
        }
        ProfileCmd::Set { skills } => {
            let skills: Vec<String> = skills
                .into_iter()
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
            let profile = Profile { skills };
            profile.save(&path)?;
            println!("saved skills: {}", profile.skills.join(", "));
            println!("path: {}", path.display());
        }
    }
    Ok(())
}

fn truncate(value: &str, max: usize) -> String {
    if value.chars().count() <= max {
        return value.to_owned();
    }
    let mut out: String = value.chars().take(max.saturating_sub(1)).collect();
    out.push('…');
    out
}
