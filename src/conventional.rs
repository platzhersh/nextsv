//! Represents a vector of conventional commits
//!

use std::collections::HashMap;

#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub struct ConventionalCommits {
    commits: Vec<String>,
    counts: HashMap<String, u32>,
    breaking: bool,
}

impl ConventionalCommits {
    pub fn new() -> ConventionalCommits {
        ConventionalCommits::default()
    }

    pub fn push(&mut self, commit: &git2::Commit) -> &Self {
        if commit.summary().take().unwrap_or("No") != "No" {
            if let Ok(conventional) = git_conventional::Commit::parse(
                commit.summary().take().unwrap_or("NotConventional"),
            ) {
                self.increment_counts(conventional.type_());

                if !self.breaking {
                    self.breaking = conventional.breaking();
                }
            }
            self.commits.push(
                commit
                    .summary()
                    .take()
                    .unwrap_or("NotConventional")
                    .to_string(),
            );
        }
        self
    }

    pub fn increment_counts(&mut self, commit_type: git_conventional::Type) {
        let counter = self.counts.entry(commit_type.to_string()).or_insert(0);
        *counter += 1;
    }

    pub fn counts(&self) -> HashMap<String, u32> {
        self.counts.clone()
    }

    pub fn commits_by_type(&self, commit_type: &str) -> u32 {
        self.counts.get(commit_type).unwrap_or(&0_u32).to_owned()
    }

    pub fn commits_all_types(&self) -> u32 {
        self.counts.values().sum()
    }

    pub fn breaking(&self) -> bool {
        self.breaking
    }

    /// Set the breaking flag value
    ///
    pub fn set_breaking(&mut self, flag: bool) -> &mut Self {
        self.breaking = flag;
        self
    }
}
