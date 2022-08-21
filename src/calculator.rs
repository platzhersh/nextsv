//! A semantic tag
//!
//! ## Example
//!
//!
//! ## Panics
//!
//!

use crate::{ConventionalCommits, Error, Level, Semantic};
use git2::Repository;
use std::fmt;

/// The latest semantic version tag (vx.y.z)
///
pub fn latest(version_prefix: &str) -> Result<Semantic, Error> {
    let repo = Repository::open(".")?;
    log::debug!("repo opened to find latest");
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

/// The options for choosing the level of a forced change
///
/// The enum is used by the force method to define the level
/// at which the forced change is made.
///
#[derive(Debug)]
pub enum ForceLevel {
    /// force change to the major component of semver
    Major,
    /// force change to the minor component of semver
    Minor,
    /// force change to the patch component of semver
    Patch,
}

impl fmt::Display for ForceLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ForceLevel::Major => write!(f, "major"),
            ForceLevel::Minor => write!(f, "minor"),
            ForceLevel::Patch => write!(f, "patch"),
        }
    }
}

/// VersionCalculator
///
/// Builds up data about the current version to calculate the next version
/// number and change level
///
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct VersionCalculator {
    current_version: Semantic,
    conventional: Option<ConventionalCommits>,
    files: Option<Vec<String>>,
}

impl VersionCalculator {
    /// Create a new VersionCalculator struct
    ///
    /// ## Parameters
    ///
    ///  - version_prefix - identifies version tags
    ///
    pub fn new(version_prefix: &str) -> Result<VersionCalculator, Error> {
        let current_version = latest(version_prefix)?;
        Ok(VersionCalculator {
            current_version,
            conventional: None,
            files: None,
        })
    }

    /// Report the current_version
    ///
    pub fn name(&self) -> Semantic {
        self.current_version.clone()
    }

    /// The count of commits of a type in the conventional commits field
    ///
    /// ## Parameters
    ///
    /// - commit_type - identifies the type of commit e.g. "feat"
    ///
    /// ## Error handling
    ///
    /// If there are no conventional commits it returns 0.
    /// If conventional is None returns 0.
    ///
    pub fn count_commits_by_type(&self, commit_type: &str) -> u32 {
        match self.conventional.clone() {
            Some(conventional) => conventional
                .counts()
                .get(commit_type)
                .unwrap_or(&0_u32)
                .to_owned(),
            None => 0_u32,
        }
    }

    /// Report the status of the breaking flag in the conventional commits
    ///
    /// ## Error Handling
    ///
    /// If the conventional is None returns false
    ///
    pub fn breaking(&self) -> bool {
        match self.conventional.clone() {
            Some(conventional) => conventional.breaking(),
            None => false,
        }
    }

    /// Force update next_version to return a specific result
    ///
    /// Options are defined in `ForceLevel`
    ///
    pub fn force(&mut self, level: ForceLevel) -> Self {
        let mut conventional_commits = ConventionalCommits::new();
        log::debug!("forcing a change to {}", level);
        match level {
            ForceLevel::Major => {
                conventional_commits.set_breaking(true);
            }
            ForceLevel::Minor => {
                conventional_commits.increment_counts(git_conventional::Type::FEAT);
            }
            ForceLevel::Patch => {
                conventional_commits.increment_counts(git_conventional::Type::FIX);
            }
        }

        self.conventional = Some(conventional_commits);
        self.clone()
    }

    /// Get the conventional commits created since the tag was created
    ///
    /// Uses `git2` to open the repository and walk back to the
    /// latest version tag collecting the conventional commits.
    ///
    /// ## Error Handling
    ///
    /// Errors from 'git2' are returned.
    ///
    pub fn commits(mut self) -> Result<Self, Error> {
        let repo = git2::Repository::open(".")?;
        log::debug!("repo opened to find conventional commits");
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
            // TODO: Better handling of this error as the first error
            // encountered will abandon the entire function - is this necessary?
            let commit = commit?;
            log::trace!("commit found: {}", &commit.summary().unwrap_or_default());
            conventional_commits.push(&commit);
        }

        self.conventional = Some(conventional_commits);

        Ok(self)
    }

    /// Calculate the next version and report the version number
    /// and level at which the change is made.
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
                    log::warn!("Not yet at a stable version");
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

    /// Report version 1.0.0 and update level major
    ///
    /// ## Error
    ///
    /// Report error if major version number is greater than 0
    pub fn promote_first(&mut self) -> Result<(Semantic, Level), Error> {
        if 0 < self.current_version.major() {
            Err(Error::MajorAlreadyUsed(
                self.current_version.major().to_string(),
            ))
        } else {
            Ok(self.force(ForceLevel::Major).next_version())
        }
    }

    /// Check for required files
    ///
    /// ## Parameters
    ///
    /// - files - a list of the required files or None
    ///
    /// ## Error
    ///
    /// Report error if one of the files are not found.
    /// Exits on the first failure.
    pub fn has_required(&mut self, files_required: Vec<String>) -> Result<&mut Self, Error> {
        let files = self.files.clone();
        if let Some(files) = files {
            let mut missing_files = vec![];

            for required_file in files_required {
                if !files.contains(&required_file) {
                    missing_files.push(required_file.clone());
                }
            }

            if !missing_files.is_empty() {
                return Err(Error::MissingRequiredFile(missing_files));
            }
        } else {
            return Err(Error::NoFilesListed);
        }

        Ok(self)
    }
}
