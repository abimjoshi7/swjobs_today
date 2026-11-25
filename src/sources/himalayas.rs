use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::job::Job;
use crate::source::Source;

const API_URL: &str = "https://himalayas.app/jobs/api";
const USER_AGENT: &str = "sjt/0.1 (personal learning project; +https://github.com/abimjoshi7/swjobs_today)";
/// Keep sync light: one page, respect Himalayas caching (~daily).
const PAGE_LIMIT: u32 = 20;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Himalayas;

impl Source for Himalayas {
    fn fetch(&self) -> Result<Vec<Job>> {
        let client = reqwest::blocking::Client::builder()
            .user_agent(USER_AGENT)
            .build()
            .context("failed to build HTTP client")?;

        let response = client
            .get(API_URL)
            .query(&[("limit", PAGE_LIMIT.to_string())])
            .send()
            .context("Himalayas request failed")?
            .error_for_status()
            .context("Himalayas returned an error status")?;

        let body = response.text().context("failed to read Himalayas body")?;
        parse_jobs(&body)
    }
}

#[derive(Debug, Deserialize)]
struct HimalayasResponse {
    jobs: Vec<HimalayasJob>,
}

#[derive(Debug, Deserialize)]
struct HimalayasJob {
    title: String,
    #[serde(rename = "companyName")]
    company_name: String,
    #[serde(rename = "applicationLink")]
    application_link: String,
    guid: String,
    #[serde(rename = "pubDate")]
    pub_date: i64,
    #[serde(default)]
    categories: Vec<String>,
    #[serde(default)]
    seniority: Vec<String>,
    #[serde(default, rename = "parentCategories")]
    parent_categories: Vec<String>,
}

impl HimalayasJob {
    fn to_job(&self) -> Result<Job> {
        let posted_at = DateTime::<Utc>::from_timestamp(self.pub_date, 0).with_context(|| {
            format!("invalid pubDate {} for job {}", self.pub_date, self.guid)
        })?;

        let mut skills = self.categories.clone();
        skills.extend(self.seniority.iter().cloned());
        skills.extend(self.parent_categories.iter().cloned());
        skills.sort();
        skills.dedup();

        Ok(Job {
            id: self.guid.clone(),
            source: "himalayas".to_owned(),
            title: self.title.clone(),
            company: self.company_name.clone(),
            url: self.application_link.clone(),
            posted_at,
            skills,
        })
    }
}

/// Parse a Himalayas API JSON body into domain jobs (no network).
pub fn parse_jobs(json: &str) -> Result<Vec<Job>> {
    let response: HimalayasResponse =
        serde_json::from_str(json).context("failed to deserialize Himalayas JSON")?;
    response
        .jobs
        .iter()
        .map(HimalayasJob::to_job)
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_min_fixture() {
        let jobs = parse_jobs(include_str!("../fixtures/himalayas_min.json")).expect("parse");
        assert_eq!(jobs.len(), 2);
        assert_eq!(jobs[0].source, "himalayas");
        assert_eq!(jobs[0].title, "Senior Software Engineer");
        assert_eq!(jobs[0].company, "Example Labs");
        assert!(jobs[0].url.contains("utm_source=api"));
        assert!(jobs[0].skills.iter().any(|s| s == "Rust"));
        assert!(jobs[0].is_tech());
        assert!(!jobs[1].is_tech());
    }

    #[test]
    fn parse_live_sample_fixture() {
        let jobs = parse_jobs(include_str!("../fixtures/himalayas_sample.json")).expect("parse");
        assert_eq!(jobs.len(), 5);
        assert!(jobs.iter().all(|j| j.source == "himalayas"));
        assert!(jobs.iter().all(|j| !j.id.is_empty()));
    }
}
