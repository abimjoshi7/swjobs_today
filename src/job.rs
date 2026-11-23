use std::collections::HashSet;

use chrono::{DateTime, TimeDelta, Utc};
use serde::Deserialize;

use crate::profile::Profile;

const TECH_TITLE_KEYWORDS: &[&str] = &[
    "software",
    "developer",
    "engineer",
    "devops",
    "backend",
    "frontend",
    "full-stack",
    "fullstack",
    "sre",
    "programmer",
    "swe",
    "platform",
];

/// On-disk fixture row. `age_hours` is relative to a provided `now` so fixtures stay fresh.
#[derive(Debug, Clone, Deserialize)]
struct FixtureJob {
    id: u16,
    source: String,
    title: String,
    company: String,
    url: String,
    age_hours: i64,
    skills: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct FixtureFile {
    jobs: Vec<FixtureJob>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Job {
    pub id: u16,
    pub source: String,
    pub title: String,
    pub company: String,
    pub url: String,
    pub posted_at: DateTime<Utc>,
    pub skills: Vec<String>,
}

impl Job {
    pub fn is_tech(&self) -> bool {
        let title = self.title.to_lowercase();
        TECH_TITLE_KEYWORDS
            .iter()
            .any(|keyword| title.contains(keyword))
    }

    pub fn is_fresh(&self, now: DateTime<Utc>) -> bool {
        let age = now - self.posted_at;
        age >= TimeDelta::zero() && age <= TimeDelta::hours(24)
    }

    pub fn score(&self, profile: &Profile) -> f64 {
        let profile_set: HashSet<String> = profile
            .skills
            .iter()
            .map(|skill| skill.to_lowercase())
            .collect();

        self.skills
            .iter()
            .filter(|skill| profile_set.contains(&skill.to_lowercase()))
            .count() as f64
    }
}

/// Filter to tech + fresh, score, sort highest first.
pub fn rank_today(jobs: &[Job], profile: &Profile, now: DateTime<Utc>) -> Vec<(Job, f64)> {
    let mut ranked: Vec<(Job, f64)> = jobs
        .iter()
        .filter(|job| job.is_tech() && job.is_fresh(now))
        .map(|job| (job.clone(), job.score(profile)))
        .collect();

    ranked.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    ranked
}

/// Load bundled fixtures; `posted_at = now - age_hours`.
pub fn load_fixture_jobs(now: DateTime<Utc>) -> Result<Vec<Job>, serde_json::Error> {
    let file: FixtureFile = serde_json::from_str(include_str!("fixtures/jobs.json"))?;
    Ok(file
        .jobs
        .into_iter()
        .map(|row| Job {
            id: row.id,
            source: row.source,
            title: row.title,
            company: row.company,
            url: row.url,
            posted_at: now - TimeDelta::hours(row.age_hours),
            skills: row.skills,
        })
        .collect())
}

#[cfg(test)]
mod test {
    use chrono::TimeZone;

    use super::*;

    fn sample_job(title: &str, skills: &[&str], posted_at: DateTime<Utc>) -> Job {
        Job {
            id: 1,
            source: "himalayas".to_owned(),
            title: title.to_owned(),
            company: "Example".to_owned(),
            url: "https://example.com/jobs/1".to_owned(),
            posted_at,
            skills: skills.iter().map(|s| (*s).to_owned()).collect(),
        }
    }

    // Intentionally red / exploratory cases — keep ignored while edge tests cover behavior.
    #[test]
    #[ignore = "intentionally failing exploratory test"]
    fn is_tech_test() {
        let result = Job {
            id: 1,
            source: "Himalayas".to_owned(),
            title: "Staff Software Engineer".to_owned(),
            company: "Netflix".to_owned(),
            posted_at: Utc::now(),
            skills: vec!["Rust".to_owned(), "Database".to_owned()],
            url: "https://www.netflix.com/jobs".to_owned(),
        }
        .is_tech();
        assert!(result);
    }

    #[test]
    #[ignore = "intentionally failing exploratory test"]
    fn is_fresh_test() {
        let result = Job {
            id: 1,
            source: "Himalayas".to_owned(),
            title: "Staff Software Engineer".to_owned(),
            company: "Netflix".to_owned(),
            posted_at: Utc.with_ymd_and_hms(2026, 9, 30, 12, 10, 23).unwrap(),
            skills: vec!["Rust".to_owned(), "Database".to_owned()],
            url: "https://www.netflix.com/jobs".to_owned(),
        }
        .is_fresh(Utc::now());
        assert!(result);
    }

    #[test]
    fn score_test() {
        let profile = Profile {
            skills: vec!["Rust".to_owned(), "Engineer".to_owned()],
        };
        let result = sample_job("Staff Software Engineer", &["Rust", "Database"], Utc::now())
            .score(&profile);
        assert_eq!(result, 1.0);
    }

    #[test]
    fn is_tech_accepts_software_titles() {
        let job = sample_job("Staff Software Engineer", &["Rust"], Utc::now());
        assert!(job.is_tech());
    }

    #[test]
    fn is_tech_rejects_non_tech_titles() {
        let job = sample_job("Registered Nurse", &["EMR"], Utc::now());
        assert!(!job.is_tech());
    }

    #[test]
    fn is_fresh_within_24h() {
        let now = Utc.with_ymd_and_hms(2026, 11, 23, 12, 0, 0).unwrap();
        let job = sample_job(
            "Software Engineer",
            &["Rust"],
            now - TimeDelta::hours(1),
        );
        assert!(job.is_fresh(now));
    }

    #[test]
    fn is_fresh_rejects_stale_and_future() {
        let now = Utc.with_ymd_and_hms(2026, 11, 23, 12, 0, 0).unwrap();
        let stale = sample_job(
            "Software Engineer",
            &["Rust"],
            now - TimeDelta::hours(48),
        );
        let future = sample_job(
            "Software Engineer",
            &["Rust"],
            now + TimeDelta::hours(3),
        );
        assert!(!stale.is_fresh(now));
        assert!(!future.is_fresh(now));
    }

    #[test]
    fn score_is_case_insensitive() {
        let profile = Profile {
            skills: vec!["rust".to_owned()],
        };
        let job = sample_job("Software Engineer", &["RUST", "Go"], Utc::now());
        assert_eq!(job.score(&profile), 1.0);
    }

    #[test]
    fn rank_today_filters_and_orders_by_score() {
        let now = Utc.with_ymd_and_hms(2026, 11, 23, 12, 0, 0).unwrap();
        let profile = Profile {
            skills: vec!["Rust".to_owned(), "PostgreSQL".to_owned()],
        };
        let jobs = load_fixture_jobs(now).expect("fixtures parse");
        let ranked = rank_today(&jobs, &profile, now);

        // Non-tech (#3, #5), stale tech (#4), future (#6) dropped; #1 and #2 remain.
        assert_eq!(ranked.len(), 2);
        assert_eq!(ranked[0].0.id, 1); // Rust + PostgreSQL overlap
        assert!(ranked[0].1 > ranked[1].1);
        assert_eq!(ranked[1].0.id, 2); // Go/K8s — zero overlap with profile
    }

    #[test]
    fn fixtures_include_edge_cases() {
        let now = Utc.with_ymd_and_hms(2026, 11, 23, 12, 0, 0).unwrap();
        let jobs = load_fixture_jobs(now).expect("fixtures parse");
        assert!(jobs.iter().any(|j| !j.is_tech()));
        assert!(jobs.iter().any(|j| j.is_tech() && !j.is_fresh(now)));
        assert!(jobs.iter().any(|j| j.posted_at > now));
        assert!(jobs.iter().any(|j| j.is_tech() && j.is_fresh(now)));
    }
}
