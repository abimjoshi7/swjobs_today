use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::job::Job;
use crate::profile::Profile;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct JobStore {
    pub synced_at: Option<DateTime<Utc>>,
    pub jobs: Vec<Job>,
}

impl JobStore {
    pub fn default_path() -> PathBuf {
        Profile::data_dir().join("jobs.json")
    }

    pub fn load(path: &Path) -> Result<Self> {
        if !path.exists() {
            return Ok(Self::default());
        }
        let raw = fs::read_to_string(path)
            .with_context(|| format!("failed to read jobs at {}", path.display()))?;
        let store: Self = serde_json::from_str(&raw)
            .with_context(|| format!("failed to parse jobs at {}", path.display()))?;
        Ok(store)
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("failed to create {}", parent.display()))?;
        }
        let raw = serde_json::to_string_pretty(self).context("failed to serialize jobs")?;
        fs::write(path, raw)
            .with_context(|| format!("failed to write jobs at {}", path.display()))?;
        Ok(())
    }

    /// Drop jobs outside the freshness window, then persist.
    pub fn sweep_and_save(&mut self, path: &Path, now: DateTime<Utc>) -> Result<usize> {
        let before = self.jobs.len();
        self.jobs.retain(|job| job.is_fresh(now));
        let removed = before - self.jobs.len();
        self.save(path)?;
        Ok(removed)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use chrono::TimeDelta;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn tmp_path(name: &str) -> PathBuf {
        PathBuf::from(format!(
            "/tmp/sjt-jobs-{}-{}",
            name,
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("time")
                .as_nanos()
        ))
    }

    #[test]
    fn sweep_removes_stale_jobs() {
        let now = Utc::now();
        let path = tmp_path("sweep");
        let mut store = JobStore {
            synced_at: Some(now),
            jobs: vec![
                Job {
                    id: "fresh".into(),
                    source: "himalayas".into(),
                    title: "Software Engineer".into(),
                    company: "A".into(),
                    url: "https://example.com/a".into(),
                    posted_at: now - TimeDelta::hours(2),
                    skills: vec!["Rust".into()],
                },
                Job {
                    id: "stale".into(),
                    source: "himalayas".into(),
                    title: "Software Engineer".into(),
                    company: "B".into(),
                    url: "https://example.com/b".into(),
                    posted_at: now - TimeDelta::hours(48),
                    skills: vec!["Rust".into()],
                },
            ],
        };
        let removed = store.sweep_and_save(&path, now).expect("sweep");
        assert_eq!(removed, 1);
        let loaded = JobStore::load(&path).expect("load");
        assert_eq!(loaded.jobs.len(), 1);
        assert_eq!(loaded.jobs[0].id, "fresh");
        let _ = fs::remove_file(path);
    }
}
