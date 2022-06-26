//! A semantic tag
//!
//! # Example
//!
//!
//! # Panics
//!
//!

use crate::{ConventionalCommits, Error, Semantic};
use git2::{Oid, Repository};

/// Describes a tag
#[derive(Debug, PartialEq, PartialOrd, Eq, Clone, Ord)]
pub struct VersionTag {
    name: Semantic,
    id: Oid,
    conventional: Option<ConventionalCommits>,
}

impl VersionTag {
    fn new(name: Semantic, id: Oid) -> Self {
        VersionTag {
            name,
            id,
            conventional: None,
        }
    }

    /// The  name field
    pub fn name(&self) -> Semantic {
        self.name.clone()
    }

    /// The id field
    pub fn id(&self) -> Oid {
        self.id
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

    /// The latest semantic version tag (vx.y.z)
    ///
    pub fn latest(version_prefix: &str) -> Result<Self, Error> {
        let repo = Repository::open(".")?;
        let mut versions = vec![];
        repo.tag_foreach(|id, name| {
            if let Ok(name) = String::from_utf8(name.to_owned()) {
                if let Some(name) = name.strip_prefix("refs/tags/") {
                    if name.starts_with('v') {
                        if let Ok(semantic_version) = Semantic::parse(name, version_prefix) {
                            versions.push(VersionTag::new(semantic_version, id));
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

    /// The number of conventional commits created since the tag was created
    ///
    pub fn commits(mut self) -> Result<Self, Error> {
        let repo = git2::Repository::open(".")?;
        let mut revwalk = repo.revwalk()?;
        revwalk.set_sorting(git2::Sort::NONE)?;
        revwalk.push_head()?;
        let glob = dbg!(format!("refs/tags/{}", self.name));
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

    pub fn next(&self) -> Semantic {
        // clone the current version to mutate for the next version
        let mut next_version = self.name.clone();

        // check the conventional commits. No conventional commits; no change.
        if let Some(conventional) = self.conventional.clone() {
            let other_commits = conventional.docs_commits()
                + conventional.chore_commits()
                + conventional.refactor_commits();

            // Breaking change found in commits
            if conventional.breaking() {
                next_version.breaking_increment();
            }

            if next_version.major() == 0 {
                if conventional.feat_commits() > 0 {
                    next_version.increment_minor();
                } else if other_commits > 0 {
                    next_version.increment_patch();
                }
            } else if conventional.feat_commits() > 0 {
                next_version.increment_major();
            } else if conventional.fix_commits() > 0 {
                next_version.increment_minor();
            } else if other_commits > 0 {
                next_version.increment_patch();
            }
        }

        next_version
    }
}
