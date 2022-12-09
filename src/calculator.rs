//! A semantic tag
//!
//! ## Example
//!
//!
//! ## Panics
//!
//!

const FEATURE: &str = "feat";
const FIX: &str = "fix";

use crate::{ConventionalCommits, Error, Level, Semantic};
use clap::ValueEnum;
use git2::Repository;
use std::{collections::HashSet, ffi::OsString, fmt};

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

/// The options for choosing the level of a forced file requirement
///
/// The enum is used by the has_required method to define the level
/// at which the the required files are enforced.
///
#[derive(Debug, PartialEq, Eq, PartialOrd, Clone, ValueEnum)]
pub enum EnforceLevel {
    /// enforce requirements for breaking only
    Breaking = 4,
    /// enforce requirements for features and breaking
    Feature = 3,
    /// enforce requirements for fix, feature and breaking
    Fix = 2,
    /// enforce requirements for all types
    Other = 1,
}

impl std::str::FromStr for EnforceLevel {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "breaking" => Ok(EnforceLevel::Breaking),
            "feature" => Ok(EnforceLevel::Feature),
            "fix" => Ok(EnforceLevel::Fix),
            "other" => Ok(EnforceLevel::Other),
            _ => Err(Error::NoVersionTag),
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
    files: Option<HashSet<OsString>>,
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
    pub fn walk_commits(mut self) -> Result<Self, Error> {
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

        // Walk back through the commits
        let mut files = HashSet::new();
        for commit in revwalk.flatten() {
            // Get the summary for the conventional commits vec
            log::trace!("commit found: {}", &commit.summary().unwrap_or_default());
            conventional_commits.push(&commit);
            // Get the files for the files vec
            let tree = commit.tree()?;
            let diff = repo.diff_tree_to_workdir(Some(&tree), None).unwrap();

            diff.print(git2::DiffFormat::NameOnly, |delta, _hunk, _line| {
                let file = delta.new_file().path().unwrap().file_name().unwrap();
                log::trace!("file found: {:?}", file);
                files.insert(file.to_os_string());
                true
            })
            .unwrap();
        }

        self.conventional = Some(conventional_commits);
        log::debug!("Files found: {:#?}", &files);
        self.files = Some(files);

        Ok(self)
    }

    /// Calculate the next version and report the version number
    /// and level at which the change is made.
    pub fn next_version(&mut self) -> (Semantic, Level) {
        // check the conventional commits. No conventional commits; no change.
        #[cfg(let_else)]
        let Some(conventional) = self.conventional.clone() else {
            return (self.current_version.clone(), Level::None)
        };
        #[cfg(not(let_else))]
        let conventional = match self.conventional.clone() {
            Some(c) => c,
            None => return (self.current_version.clone(), Level::None),
        };

        let bump = if conventional.breaking() {
            // Breaking change found in commits
            log::debug!("breaking change found");
            Level::Major
        } else if 0 < conventional.commits_by_type("feat") {
            log::debug!(
                "{} feature commit(s) found requiring increment of minor number",
                &conventional.commits_by_type("feat")
            );
            Level::Minor
        } else if 0 < conventional.commits_all_types() {
            log::debug!(
                "{} conventional commit(s) found requiring increment of patch number",
                &conventional.commits_all_types()
            );
            Level::Patch
        } else {
            Level::None
        };

        let final_bump = if self.current_version.major() == 0 {
            log::info!("Not yet at a stable version");
            match bump {
                Level::Major => Level::Minor,
                Level::Minor => Level::Patch,
                _ => bump,
            }
        } else {
            bump
        };
        let next_version = next_version_calculator(self.current_version.clone(), &final_bump);

        (next_version, final_bump)
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
    pub fn has_required(
        &mut self,
        files_required: Vec<OsString>,
        level: EnforceLevel,
    ) -> Result<&mut Self, Error> {
        // How to use level to ensure that the rule is only applied
        // when required levels of commits are included

        let mut level_found = EnforceLevel::Other;
        if self.conventional.clone().unwrap().commits_by_type(FIX) > 0 {
            level_found = EnforceLevel::Fix
        };
        if self.conventional.clone().unwrap().commits_by_type(FEATURE) > 0 {
            level_found = EnforceLevel::Feature;
        };
        if self.breaking() {
            level_found = EnforceLevel::Feature
        };

        log::debug!(
            "{:?} is the highest level commit found. {:?} level is required to enforce required files.",
            &level_found,
            &level
        );
        if level_found >= level {
            let files = self.files.clone();
            if let Some(files) = files {
                let mut missing_files = vec![];

                for file in files_required {
                    if !files.contains(&file) {
                        missing_files.push(file.clone());
                    }
                }

                if !missing_files.is_empty() {
                    return Err(Error::MissingRequiredFile(missing_files));
                }
            } else {
                return Err(Error::NoFilesListed);
            }
        }

        Ok(self)
    }
}

fn next_version_calculator(mut version: Semantic, bump: &Level) -> Semantic {
    match *bump {
        Level::Major => version.increment_major().clone(),
        Level::Minor => version.increment_minor().clone(),
        Level::Patch => version.increment_patch().clone(),
        _ => version,
    }
}
