//! Represents a vector of conventional commits
//!

use std::collections::HashMap;

#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub struct ConventionalCommits {
    commits: Vec<String>,
    counts: HashMap<String, u32>,
    feat_count: u32,
    fix_count: u32,
    docs_count: u32,
    chore_count: u32,
    refactor_count: u32,
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

                if conventional.type_() == git_conventional::Type::FEAT {
                    self.feat_count += 1;
                }

                if conventional.type_() == git_conventional::Type::FIX {
                    self.fix_count += 1;
                }

                if conventional.type_() == git_conventional::Type::DOCS {
                    self.docs_count += 1;
                }

                if conventional.type_() == git_conventional::Type::CHORE {
                    self.chore_count += 1;
                }

                if conventional.type_() == git_conventional::Type::REFACTOR {
                    self.refactor_count += 1;
                }
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

    fn increment_counts(&mut self, commit_type: git_conventional::Type) {
        let counter = self.counts.entry(commit_type.to_string()).or_insert(0);
        *counter += 1;
    }

    pub fn feat_commits(&self) -> u32 {
        self.feat_count
    }

    pub fn fix_commits(&self) -> u32 {
        self.fix_count
    }

    pub fn docs_commits(&self) -> u32 {
        self.docs_count
    }

    pub fn chore_commits(&self) -> u32 {
        self.chore_count
    }

    pub fn refactor_commits(&self) -> u32 {
        self.refactor_count
    }

    pub fn total_commits(&self) -> u32 {
        self.feat_count + self.fix_count + self.docs_count + self.refactor_count
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

    /// Set feat_commits count to one
    ///
    pub fn set_one_feat(&mut self) -> &mut Self {
        self.feat_count = 1;
        self
    }

    /// Set feat_commits count to one
    ///
    pub fn set_one_fix(&mut self) -> &mut Self {
        self.fix_count = 1;
        self
    }
}
