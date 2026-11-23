use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Profile {
    pub skills: Vec<String>,
}

impl Profile {
    pub fn data_dir() -> PathBuf {
        PathBuf::from(".data")
    }

    pub fn default_path() -> PathBuf {
        Self::data_dir().join("profile.json")
    }

    pub fn load(path: &Path) -> Result<Self> {
        if !path.exists() {
            return Ok(Self::default());
        }
        let raw = fs::read_to_string(path)
            .with_context(|| format!("failed to read profile at {}", path.display()))?;
        let profile: Self = serde_json::from_str(&raw)
            .with_context(|| format!("failed to parse profile at {}", path.display()))?;
        Ok(profile)
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("failed to create {}", parent.display()))?;
        }
        let raw = serde_json::to_string_pretty(self).context("failed to serialize profile")?;
        fs::write(path, raw)
            .with_context(|| format!("failed to write profile at {}", path.display()))?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn load_missing_returns_default() {
        let path = PathBuf::from(format!(
            "/tmp/sjt-profile-missing-{}.json",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("time")
                .as_nanos()
        ));
        let profile = Profile::load(&path).expect("load");
        assert!(profile.skills.is_empty());
    }

    #[test]
    fn save_round_trip() {
        let dir = PathBuf::from(format!(
            "/tmp/sjt-profile-{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("time")
                .as_nanos()
        ));
        let path = dir.join("profile.json");
        let profile = Profile {
            skills: vec!["Rust".into(), "TypeScript".into()],
        };
        profile.save(&path).expect("save");
        let loaded = Profile::load(&path).expect("load");
        assert_eq!(loaded, profile);
        let _ = fs::remove_dir_all(dir);
    }
}
