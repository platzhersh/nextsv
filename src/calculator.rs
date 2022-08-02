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
    log::debug!("opened repo");
    let mut versions = vec![];
    repo.tag_foreach(|_id, name| {
        if let Ok(name) = String::from_utf8(name.to_owned()) {
            if let Some(name) = name.strip_prefix("refs/tags/") {
                if name.starts_with(version_prefix) {
                    if let Ok(semantic_version) = Semantic::parse(name, version_prefix) {
                        log::trace!("found qualifying tag {}", &semantic_version);
                        versions.push(semantic_version);
                    }
                }
            }
        }
        true
    })?;

    versions.sort();
    log::debug!("versions sorted");

    match versions.last().cloned() {
        Some(v) => {
            log::trace!("latest version found is {}", &v);
            Ok(v)
        }
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

    /// The count of commits of a type in the conventional commits field
    /// If the conventional commits field has not been set returns 0
    pub fn types(&self, commit_type: &str) -> u32 {
        if let Some(conventional) = self.conventional.clone() {
            conventional
                .counts()
                .get(commit_type)
                .unwrap_or(&0_u32)
                .to_owned()
        } else {
            0_u32
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

    /// Get the conventional commits created since the tag was created
    ///
    pub fn commits(mut self) -> Result<Self, Error> {
        let repo = git2::Repository::open(".")?;
        log::debug!("repo opened");
        let mut revwalk = repo.revwalk()?;
        revwalk.set_sorting(git2::Sort::NONE)?;
        revwalk.push_head()?;
        log::debug!("starting the walk from the HEAD");
        let glob = format!("refs/tags/{}", &self.current_version);
        revwalk.hide_ref(&glob)?;
        log::debug!("hide commits from {}", &self.current_version);

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
            log::trace!("commit found {}", &commit.summary().unwrap_or_default());
            conventional_commits.push(&commit);
        }

        self.conventional = Some(conventional_commits);

        Ok(self)
    }

    #[cfg(feature = "version")]
    pub fn next_version(&mut self) -> (Semantic, Level) {
        // clone the current version to mutate for the next version
        let mut next_version = self.current_version.clone();
        let mut bump = Level::None;

        // check the conventional commits. No conventional commits; no change.
        if let Some(conventional) = self.conventional.clone() {
            // Breaking change found in commits
            if conventional.breaking() {
                log::debug!("breaking change found");
                if next_version.major() == 0 {
                    log::debug!("Not yet at a stable version");
                    next_version.increment_minor();
                    bump = Level::Minor;
                } else {
                    next_version.increment_major();
                    bump = Level::Major;
                }
            } else if 0 < conventional.commits_by_type("feat") {
                log::debug!(
                    "{} feature commit(s) found requiring increment of minor  number",
                    &conventional.commits_by_type("feat")
                );
                next_version.increment_minor();
                bump = Level::Minor;
            } else if 0 < conventional.commits_all_types() {
                log::debug!(
                    "{} conventional commit(s) found requiring increment of patch number",
                    &conventional.commits_all_types()
                );
                next_version.increment_patch();
                bump = Level::Patch;
            } else {
                bump = Level::None;
            }
        }

        (next_version, bump)
    }

    #[cfg(feature = "level")]
    pub fn next_level(&mut self) -> Result<Level, Error> {
        // check the conventional commits. No conventional commits; no change.
        if let Some(conventional) = self.conventional.clone() {
            if conventional.breaking() {
                log::debug!("breaking change found");
                if self.current_version.major() == 0 {
                    log::debug!("Not yet at a stable version");
                    Ok(Level::Minor)
                } else {
                    Ok(Level::Major)
                }
            } else if 0 < conventional.commits_by_type("feat") {
                log::debug!(
                    "{} feature commit(s) found requiring increment of minor  number",
                    &conventional.commits_by_type("feat")
                );
                Ok(Level::Minor)
            } else if 0 < conventional.commits_all_types() {
                log::debug!(
                    "{} conventional commit(s) found requiring increment of patch number",
                    &conventional.commits_all_types()
                );
                Ok(Level::Patch)
            } else {
                // Ok(Level::None)
                Err(Error::NoLevelChange)
            }
        } else {
            Err(Error::NoConventionalCommits)
        }
    }

    // pub fn bump_level(&self) -> Option<Level> {
    //     self.bump_level.clone()
    // }

    pub fn promote_first(&mut self) -> Result<(Semantic, Level), Error> {
        Ok((
            self.current_version.first_production()?.clone(),
            Level::Major,
        ))
    }
}
