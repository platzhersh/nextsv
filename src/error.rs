//! Error types for nextsv

use thiserror::Error;

/// The error type for nextsv.
#[non_exhaustive]
#[derive(Error, Debug)]
pub enum Error {
    /// The tag provided is not a version tag as it does not
    /// start with the provided prefix string.
    #[error("Version tags must start with \"{0}\" but tag is {1}")]
    NotVersionTag(String, String),
    /// Too many components found.
    #[error("Version must have three components but at least {0} were found")]
    TooManyComponents(usize),
    /// Too few components found.
    #[error("Version must have three components but only {0} found")]
    TooFewComponents(usize),
    /// The component must be a digit
    #[error("Version must be a number but found {0}")]
    MustBeNumber(String),
    /// No valid version tag was found in the repository
    #[error("No valid version tag found in the repository")]
    NoVersionTag,
    /// The first production release (1.0.0) has already been made
    #[error("First production release already deployed. Current major version: {0}")]
    MajorAlreadyUsed(String),
    /// No conventional commits in the VersionCalculator struct
    #[error("No conventional commits have been loaded into the VersionCalculator struct. May have been called before `commits`.")]
    NoConventionalCommits,
    /// Missing required file found.
    #[error("Missing the required file(s): {0:?}.")]
    MissingRequiredFile(Vec<String>),
    /// List of files has not been generated yet (or there are no commits). Call `commits` to generate the list by walking back to the current version tag.
    #[error("No files have been listed. May have been called before `commits`.")]
    NoFilesListed,
    /// Error passed up from git2
    #[error("0:?")]
    Git2(#[from] git2::Error),
}
