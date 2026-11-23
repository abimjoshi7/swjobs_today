use std::collections::HashSet;

use chrono::{DateTime, TimeDelta, Utc};

use crate::profile::Profile;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Job {
    id: u16,
    source: String,
    title: String,
    company: String,
    url: String,
    posted_at: DateTime<Utc>,
    skills: Vec<String>,
}

impl Job {
    fn is_tech(&self) -> bool {
        // TODO: Add robust solution for this later
        self.title.contains("software, developer, engineer")
            || self
                .skills
                .contains(&"software, developer, engineer".to_owned())
    }

    fn is_fresh(&self, now: DateTime<Utc>) -> bool {
        let age = now - self.posted_at;
        age >= TimeDelta::zero() && age <= TimeDelta::hours(24)
    }

    fn score(&self, profile: &Profile) -> f64 {
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

#[cfg(test)]
mod test {
    use chrono::TimeZone;

    use super::*;

    #[test]
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
        let result = Job {
            id: 1,
            source: "Himalayas".to_owned(),
            title: "Staff Software Engineer".to_owned(),
            company: "Netflix".to_owned(),
            posted_at: Utc::now(),
            skills: vec!["Rust".to_owned(), "Database".to_owned()],
            url: "https://www.netflix.com/jobs".to_owned(),
        }
        .score(&profile);
        assert_eq!(result, 1.0);
    }
}
