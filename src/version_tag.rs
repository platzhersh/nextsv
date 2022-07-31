//! A semantic tag
//!
//! # Example
//!
//!
//! # Panics
//!
//!

use crate::{ConventionalCommits, Error, Level, Semantic};
use git2::Repository;

/// The latest semantic version tag (vx.y.z)
///
pub fn latest(version_prefix: &str) -> Result<Semantic, Error> {
    let repo = Repository::open(".")?;
    let mut versions = vec![];
    repo.tag_foreach(|_id, name| {
        if let Ok(name) = String::from_utf8(name.to_owned()) {
            if let Some(name) = name.strip_prefix("refs/tags/") {
                if name.starts_with(version_prefix) {
                    if let Ok(semantic_version) = Semantic::parse(name, version_prefix) {
                        versions.push(semantic_version);
                    }
                }
            }
        }
        true
    })?;

    versions.sort();
    let last_version = versions.last().cloned();

    match last_version {
        Some(v) => Ok(v),
        None => Err(Error::NoVersionTag),
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct VersionCalculator {
    current_version: Semantic,
    conventional: Option<ConventionalCommits>,
    bump_level: Option<Level>,
}

impl VersionCalculator {
    pub fn new(version_prefix: &str) -> Result<VersionCalculator, Error> {
        let current_version = latest(version_prefix)?;
        Ok(VersionCalculator {
            current_version,
            conventional: None,
            bump_level: None,
        })
    }

    /// The the name of the current version
    /// If the conventional commits field has not been set returns 0
    pub fn name(&self) -> Semantic {
        self.current_version.clone()
    }

    /// The count of feature commits in the conventional commits field
    /// If the conventional commits field has not been set returns 0
    pub fn feat_commits(&self) -> u32 {
        if self.conventional.is_some() {
            self.conventional.as_ref().unwrap().feat_commits()
        } else {
            0
        }
    }

    /// The count of fix commits in the conventional commits field
    /// If the conventional commits field has not been set returns 0
    pub fn fix_commits(&self) -> u32 {
        if self.conventional.is_some() {
            self.conventional.as_ref().unwrap().fix_commits()
        } else {
            0
        }
    }

    /// The count of docs commits in the conventional commits field
    /// If the conventional commits field has not been set returns 0
    pub fn docs_commits(&self) -> u32 {
        if self.conventional.is_some() {
            self.conventional.as_ref().unwrap().docs_commits()
        } else {
            0
        }
    }

    /// The count of chore commits in the conventional commits field
    /// If the conventional commits field has not been set returns 0
    pub fn chore_commits(&self) -> u32 {
        if self.conventional.is_some() {
            self.conventional.as_ref().unwrap().chore_commits()
        } else {
            0
        }
    }

    /// The count of refactor commits in the conventional commits field
    /// If the conventional commits field has not been set returns 0
    pub fn refactor_commits(&self) -> u32 {
        if self.conventional.is_some() {
            self.conventional.as_ref().unwrap().refactor_commits()
        } else {
            0
        }
    }

    /// The breaking flag in the conventional commits field
    /// If the conventional commits field has not been set returns false
    pub fn breaking(&self) -> bool {
        if self.conventional.is_some() {
            self.conventional.as_ref().unwrap().breaking()
        } else {
            false
        }
    }

    /// Construct conventional commits that forces Major update
    ///
    pub fn force_major(&mut self) -> Self {
        let mut conventional_commits = ConventionalCommits::new();
        conventional_commits.set_breaking(true);
        self.conventional = Some(conventional_commits);
        self.clone()
    }

    /// Construct conventional commits that forces Minor update
    ///
    pub fn force_minor(&mut self) -> Self {
        let mut conventional_commits = ConventionalCommits::new();
        conventional_commits.set_one_feat();
        self.conventional = Some(conventional_commits);
        self.clone()
    }

    /// Construct conventional commits that forces Patch update
    ///
    pub fn force_patch(&mut self) -> Self {
        let mut conventional_commits = ConventionalCommits::new();
        conventional_commits.set_one_fix();
        self.conventional = Some(conventional_commits);
        self.clone()
    }

    /// The number of conventional commits created since the tag was created
    ///
    pub fn commits(mut self) -> Result<Self, Error> {
        let repo = git2::Repository::open(".")?;
        let mut revwalk = repo.revwalk()?;
        revwalk.set_sorting(git2::Sort::NONE)?;
        revwalk.push_head()?;
        let glob = format!("refs/tags/{}", self.current_version);
        revwalk.hide_ref(&glob)?;

        macro_rules! filter_try {
            ($e:expr) => {
                match $e {
                    Ok(t) => t,
                    Err(e) => return Some(Err(e)),
                }
            };
        }

        #[allow(clippy::unnecessary_filter_map)]
        let revwalk = revwalk.filter_map(|id| {
            let id = filter_try!(id);
            let commit = repo.find_commit(id);
            let commit = filter_try!(commit);
            Some(Ok(commit))
        });

        let mut conventional_commits = ConventionalCommits::new();

        for commit in revwalk {
            let commit = commit?;
            conventional_commits.push(&commit);
        }

        self.conventional = Some(conventional_commits);

        Ok(self)
    }

    #[cfg(feature = "version")]
    pub fn next_version(&mut self) -> Semantic {
        // clone the current version to mutate for the next version
        let mut next_version = self.current_version.clone();

        // check the conventional commits. No conventional commits; no change.
        if let Some(conventional) = self.conventional.clone() {
            // Breaking change found in commits
            if conventional.breaking() {
                if next_version.major() == 0 {
                    next_version.increment_minor();
                    self.bump_level = Some(Level::Minor);
                } else {
                    next_version.increment_major();
                    self.bump_level = Some(Level::Major);
                }
            } else if conventional.feat_commits() > 0 {
                next_version.increment_minor();
                self.bump_level = Some(Level::Minor);
            } else if conventional.total_commits() > 0 {
                next_version.increment_patch();
                self.bump_level = Some(Level::Patch);
            } else {
                self.bump_level = Some(Level::None);
            }
        }

        next_version
    }

    #[cfg(feature = "level")]
    pub fn next_level(&mut self) -> Result<Level, Error> {
        // check the conventional commits. No conventional commits; no change.
        if let Some(conventional) = self.conventional.clone() {
            // Breaking change found in commits
            // println!("Conventional: {:#?}", conventional);
            if conventional.breaking() {
                if self.current_version.major() == 0 {
                    Ok(Level::Minor)
                } else {
                    Ok(Level::Major)
                }
            } else if conventional.feat_commits() > 0 {
                Ok(Level::Minor)
            } else if conventional.total_commits() > 0 {
                Ok(Level::Patch)
            } else {
                // Ok(Level::None)
                Err(Error::NoLevelChange)
            }
        } else {
            Err(Error::NoConventionalCommits)
        }
    }

    pub fn bump_level(&self) -> Option<Level> {
        self.bump_level.clone()
    }

    pub fn promote_first(&mut self) -> Result<Semantic, Error> {
        Ok(self.current_version.first_production()?.clone())
    }
}
