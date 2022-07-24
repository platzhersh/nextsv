//! Represents a vector of conventional commits
//!

#[derive(Default, Debug, PartialEq, PartialOrd, Eq, Clone, Ord)]
pub struct ConventionalCommits {
    commits: Vec<String>,
    feat_commits: u32,
    fix_commits: u32,
    docs_commits: u32,
    chore_commits: u32,
    refactor_commits: u32,
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
                if conventional.type_() == git_conventional::Type::FEAT {
                    self.feat_commits += 1;
                }

                if conventional.type_() == git_conventional::Type::FIX {
                    self.fix_commits += 1;
                }

                if conventional.type_() == git_conventional::Type::DOCS {
                    self.docs_commits += 1;
                }

                if conventional.type_() == git_conventional::Type::CHORE {
                    self.chore_commits += 1;
                }

                if conventional.type_() == git_conventional::Type::REFACTOR {
                    self.refactor_commits += 1;
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

    pub fn feat_commits(&self) -> u32 {
        self.feat_commits
    }

    pub fn fix_commits(&self) -> u32 {
        self.fix_commits
    }

    pub fn docs_commits(&self) -> u32 {
        self.docs_commits
    }

    pub fn chore_commits(&self) -> u32 {
        self.chore_commits
    }

    pub fn refactor_commits(&self) -> u32 {
        self.refactor_commits
    }

    pub fn total_commits(&self) -> u32 {
        self.feat_commits + self.fix_commits + self.docs_commits + self.refactor_commits
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
        self.feat_commits = 1;
        self
    }

    /// Set feat_commits count to one
    ///
    pub fn set_one_fix(&mut self) -> &mut Self {
        self.fix_commits = 1;
        self
    }
}
