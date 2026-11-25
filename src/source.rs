use crate::job::Job;
use anyhow::Result;

pub trait Source {
    fn fetch(&self) -> Result<Vec<Job>>;
}
