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

pub use error::Error;
use semantic::Semantic;

use git2::{Oid, Repository, Sort};

#[derive(Debug, PartialEq, PartialOrd, Eq, Clone, Ord)]
pub struct Tag {
    name: Semantic,
    id: Oid,
}

impl Tag {
    fn new(name: Semantic, id: Oid) -> Self {
        Tag { name, id }
    }
    pub fn name(&self) -> Semantic {
        self.name
    }
    pub fn id(&self) -> Oid {
        self.id
    }
}

/// Get a list of the version tags in the repo and identify the latest version
///
pub fn latest_version_tag() -> Result<Tag, Error> {
    let repo = Repository::open(".")?;
    let mut versions = vec![];
    repo.tag_foreach(|id, name| {
        if let Ok(name) = String::from_utf8(name.to_owned()) {
            if let Some(name) = name.strip_prefix("refs/tags/") {
                println!("id: {:?} name: {:?}", id, name);
                if name.starts_with('v') {
                    if let Ok(semantic_version) = Semantic::parse(name) {
                        versions.push(Tag::new(semantic_version, id));
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

pub fn conventional_commits_to_tag(last_release_tag: Semantic) -> Result<Vec<String>, Error> {
    let repo = Repository::open(".")?;

    let mut walk = repo.revwalk()?;
    walk.set_sorting(Sort::REVERSE)?;

    let mut ret = vec![];

    for c in walk.into_iter().flatten() {
        let commit = repo.find_commit(c)?;
        let message = commit.message();
        if let Some(m) = message {
            ret.push(m.to_string())
        }

        if let Ok(tag) = repo.find_tag(c) {
            if tag.name() == Some(&last_release_tag.to_string()) {
                break;
            }
        }
    }

    Ok(ret)
}
