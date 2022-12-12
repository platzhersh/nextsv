//! Represents a vector of conventional commits
//!

use std::collections::HashMap;

use clap::ValueEnum;

use crate::Error;

/// TypeHierarchy maps the types identified by git_conventional to a hierarchy of levels
///
/// The enum provides an ordered list to identify the highest level type found in a set
/// of conventional commits.
///
/// Types are mapped as follows:
/// - FEAT: Feature
/// - FIX: Fix
/// - REVERT: Fix
/// - DOCS: Other
/// - STYLE: Other
/// - REFACTOR: Other
/// - PERF: Other
/// - TEST: Other
/// - CHORE: Other
///
/// If a breaking change is found it sets breaking hierarchy.
///
#[derive(Default, Debug, PartialEq, Eq, PartialOrd, Ord, Clone, ValueEnum)]
pub enum TypeHierarchy {
    /// enforce requirements for all types
    #[default]
    Other = 1,
    /// enforce requirements for fix, feature and breaking
    Fix = 2,
    /// enforce requirements for features and breaking
    Feature = 3,
    /// enforce requirements for breaking only
    Breaking = 4,
}

impl TypeHierarchy {
    /// Parse a string into a TypeHierarchy mapping the types or "breaking"
    ///
    pub fn parse(s: &str) -> Result<TypeHierarchy, Error> {
        let out;

        match s.to_lowercase().as_str() {
            "feat" => out = TypeHierarchy::Feature,
            "fix" => out = TypeHierarchy::Fix,
            "revert" => out = TypeHierarchy::Fix,
            "docs" => out = TypeHierarchy::Other,
            "style" => out = TypeHierarchy::Other,
            "refactor" => out = TypeHierarchy::Other,
            "perf" => out = TypeHierarchy::Other,
            "test" => out = TypeHierarchy::Other,
            "chore" => out = TypeHierarchy::Other,
            "breaking" => out = TypeHierarchy::Breaking,
            _ => return Err(Error::NotTypeHierachyName(s.to_string())),
        }

        Ok(out)
    }
}
#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub struct ConventionalCommits {
    commits: Vec<String>,
    counts: HashMap<String, u32>,
    breaking: bool,
    top_type: Option<TypeHierarchy>,
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
                    if conventional.breaking() {
                        self.breaking = conventional.breaking();
                        self.set_top_type_if_higher("breaking");
                    } else {
                        self.set_top_type_if_higher(conventional.type_().as_str());
                    }
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

    fn set_top_type_if_higher(&mut self, type_: &str) -> &mut Self {
        let th = TypeHierarchy::parse(type_);

        if th.is_ok() {
            let th = th.unwrap();
            if self.top_type.is_none() {
                self.top_type = Some(th)
            } else if self.top_type.as_ref().unwrap() < &th {
                self.top_type = Some(th)
            }
        }
        self
    }

    pub fn top_type(&self) -> Option<TypeHierarchy> {
        self.top_type.clone()
    }
}
