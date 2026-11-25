mod cli;
mod job;
mod profile;
mod source;
mod sources;
mod store;

use anyhow::{Context, Result};
use chrono::Utc;
use clap::Parser;

use crate::cli::{Cli, Commands, ProfileCmd};
use crate::job::rank_today;
use crate::profile::Profile;
use crate::source::Source;
use crate::sources::himalayas::Himalayas;
use crate::store::JobStore;

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Today { limit } => run_today(limit)?,
        Commands::Sync => run_sync()?,
        Commands::Profile { action } => run_profile(action)?,
    }
    Ok(())
}

fn run_sync() -> Result<()> {
    let path = JobStore::default_path();
    let now = Utc::now();
    println!("syncing from Himalayas…");
    let jobs = Himalayas.fetch().context("Himalayas fetch failed")?;
    let fetched = jobs.len();
    let mut store = JobStore {
        synced_at: Some(now),
        jobs,
    };
    let removed = store.sweep_and_save(&path, now)?;
    let kept = store.jobs.len();
    println!(
        "fetched {fetched}, kept {kept} within 24h (swept {removed}), saved {}",
        path.display()
    );
    if kept == 0 {
        println!(
            "note: Himalayas listings may be older than 24h due to API cache; try again later or widen freshness in a future change"
        );
    }
    Ok(())
}

fn run_today(limit: usize) -> Result<()> {
    let profile = Profile::load(&Profile::default_path())?;
    let path = JobStore::default_path();
    let mut store = JobStore::load(&path)?;
    let now = Utc::now();

    if store.jobs.is_empty() {
        println!("No stored jobs. Run `sjt sync` first.");
        return Ok(());
    }

    let removed = store.sweep_and_save(&path, now)?;
    if removed > 0 {
        println!("swept {removed} expired job(s)");
    }

    let ranked = rank_today(&store.jobs, &profile, now);
    if ranked.is_empty() {
        println!("No matching tech jobs in the last 24h.");
        if profile.skills.is_empty() {
            println!("Hint: set skills with `sjt profile set --skills rust,python`");
        }
        return Ok(());
    }

    println!(
        "{:<6} {:<8} {:<28} {:<16} {}",
        "ID", "SCORE", "TITLE", "COMPANY", "URL"
    );
    for (job, score) in ranked.into_iter().take(limit) {
        println!(
            "{:<6} {:<8.2} {:<28} {:<16} {}",
            truncate(&short_id(&job.id), 6),
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

fn short_id(id: &str) -> String {
    id.rsplit('/').next().unwrap_or(id).to_owned()
}

fn truncate(value: &str, max: usize) -> String {
    if value.chars().count() <= max {
        return value.to_owned();
    }
    let mut out: String = value.chars().take(max.saturating_sub(1)).collect();
    out.push('…');
    out
}
