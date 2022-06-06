//! Semantic Versioning Management
//!
//! Calculates the next semantic version number based on the current version
//! number and the conventional commits that have been made since the
//! last version has been released.
//!
//! ```rust
//!
//!     let current_version = get_latest_version_tag();
//!     let new_version = current_version.bump();
//!
//! ```

mod error;
mod semantic;

use error::Error;
use semantic::Semantic;

use git2::Repository;

/// Get a list of the version tags in the repo and identify the latest version
///
pub fn get_latest_version_tag() -> Result<Semantic, Error> {
    let repo = Repository::open(".")?;

    let tags = repo.tag_names(Some("v*")).unwrap();

    let mut versions = vec![];

    for tag in tags.iter().flatten() {
        if let Ok(s) = Semantic::parse(tag) {
            versions.push(s)
        };
    }
    versions.sort();
    match versions.last() {
        Some(last_version) => Ok(last_version.to_owned()),
        None => Err(Error::NoVersionTag),
    }
}
