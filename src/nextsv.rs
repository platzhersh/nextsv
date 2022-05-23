//! Semantic Versioning Management
//!
//! Calculates the next semantic version number based on the current version
//! number and the conventional commits that have been made since the
//! last version has been released.
//!
//! ```rust
//!
//!     let current_version = Semantic::current();
//!     let new_version = current_version.bump();
//!
//! ```

use std::fmt;

/// The Semantic data structure represents a semantic version number.
#[derive(Debug, Default)]
struct Semantic {
    major: usize,
    minor: usize,
    patch: usize,
}

impl fmt::Display for Semantic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl Semantic {
    fn new(major: usize, minor: usize, patch: usize) -> Self {
        Semantic {
            major,
            minor,
            patch,
        }
    }
    /// Increment the patch component of the version number by 1
    fn increment_patch(self) -> Self {
        let mut patch = self.patch;
        patch += 1;
        Semantic::new(self.major, self.minor, patch)
    }
    /// Increment the minor component of the version number by 1
    fn increment_minor(self) -> Self {
        let mut minor = self.minor;
        minor += 1;
        Semantic::new(self.major, minor, self.patch)
    }
    /// Increment the major component of the version number by 1
    fn increment_major(self) -> Self {
        let mut major = self.major;
        major += 1;
        Semantic::new(major, self.minor, self.patch)
    }
}

#[cfg(test)]
mod tests {
    use crate::nextsv::*;

    #[test]
    fn display_semantic_version_number() {
        let version = Semantic::default();

        assert_eq!("0.0.0", &version.to_string());
    }

    #[test]
    fn bump_patch_version_number_by_one() {
        let version = Semantic::default();
        let updated_version = version.increment_patch();

        assert_eq!("0.0.1", &updated_version.to_string());
    }

    #[test]
    fn bump_minor_version_number_by_one() {
        let version = Semantic::default();
        let updated_version = version.increment_minor();

        assert_eq!("0.1.0", &updated_version.to_string());
    }

    #[test]
    fn bump_major_version_number_by_one() {
        let version = Semantic::default();
        let updated_version = version.increment_major();

        assert_eq!("1.0.0", &updated_version.to_string());
    }
}
