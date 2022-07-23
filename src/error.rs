//! Error types for nextsv

use thiserror::Error;

/// The error type for nextsv.
#[non_exhaustive]
#[derive(Error, Debug)]
pub enum Error {
    #[error("Version tags must start with the letter 'v' but tag is {0}")]
    NotVersionTag(String),
    #[error("Version must have three components but at least {0} were found")]
    TooManyComponents(usize),
    #[error("Version must have three components but only {0} found")]
    NotEnoughComponents(usize),
    #[error("Version must be a number but found {0}")]
    MustBeNumber(String),
    #[error("No valid version tag found in the repository")]
    NoVersionTag,
    #[error("First production release already deployed. Current major version: {0}")]
    MajorAlreadyUsed(String),
    #[error("No conventional commits found")]
    NoConventionalCommits,
    #[error("No conventional commits requiringa version level change")]
    NoLevelChange,
    #[error("0:?")]
    Git2(#[from] git2::Error),
}
