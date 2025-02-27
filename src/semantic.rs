//! Semantic Versioning Struct
//!
//! Holds a semantic version number as defined by
//! the [Semantic Version Specification v 2.0.0](https://semver.org/spec/v2.0.0.html)
//!
//! ## Notes
//!
//! Initial implementation does not include support
//! for pre-release suffixes.
//!

use std::fmt;

use crate::Error;

/// Level at which the next increment will be made
///
#[derive(Debug, PartialOrd, PartialEq, Eq, Ord, Clone)]
pub enum Level {
    /// When no update has been detected the level is set to none
    None,
    /// Update will be made at the patch level
    Patch,
    /// Update will be made at the private level
    Minor,
    /// Update will be made at the major level
    Major,
    /// Update is a release removing any pre-release suffixes (for future use)
    Release,
    /// Update is to an alpha pre-release suffix (for future use)
    Alpha,
    /// Update is to an beta pre-release suffix (for future use)
    Beta,
    /// Update is to an rc pre-release suffix (for future use)
    Rc,
}

impl Default for Level {
    fn default() -> Self {
        Level::None
    }
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Level::None => write!(f, "none"),
            Level::Patch => write!(f, "patch"),
            Level::Minor => write!(f, "minor"),
            Level::Major => write!(f, "major"),
            Level::Release => write!(f, "release"),
            Level::Alpha => write!(f, "alpha"),
            Level::Beta => write!(f, "beta"),
            Level::Rc => write!(f, "rc"),
        }
    }
}

/// The Semantic data structure represents a semantic version number.
///
/// TODO: Implement support for pre-release and build
///
#[derive(Debug, Default, PartialEq, PartialOrd, Eq, Ord, Clone)]
pub struct Semantic {
    version_prefix: String,
    major: usize,
    minor: usize,
    patch: usize,
}

impl fmt::Display for Semantic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}.{}.{}",
            self.version_prefix, self.major, self.minor, self.patch
        )
    }
}

impl Semantic {
    // Create a new struct specifying each of the semantic version components.
    fn new(version_prefix: String, major: usize, minor: usize, patch: usize) -> Self {
        Semantic {
            version_prefix,
            major,
            minor,
            patch,
        }
    }
    /// Parse a tag and return a struct
    /// String format expect: <version_prefix>x.y.z
    ///
    /// # Fields
    ///
    /// tag - the tag proposed as a semantic version tag
    /// version_prefix - any string before the semantic version number
    ///
    /// # Example
    ///
    /// Parse a tag into a semantic version number where "v" is used to identify
    /// tags representing semantic version numbers.
    ///
    /// ```rust
    /// # fn main() -> Result<(), nextsv::Error> {
    /// use nextsv::Semantic;
    ///
    /// let tag = "v0.2.3";
    /// let semantic_version = Semantic::parse(tag, "v")?;
    ///
    /// assert_eq!(0, semantic_version.major());
    /// assert_eq!(2, semantic_version.minor());
    /// assert_eq!(3, semantic_version.patch());
    ///
    /// # Ok(())
    /// # }
    /// ```
    /// to identify tags with semantic version numbers
    /// the tag name can be parsed
    pub fn parse(tag: &str, version_prefix: &str) -> Result<Self, Error> {
        // the tag string must start with the version_prefix
        if !tag.starts_with(version_prefix) {
            return Err(Error::NotVersionTag(
                version_prefix.to_string(),
                tag.to_string(),
            ));
        }

        let version = tag.trim_start_matches(version_prefix);
        let components: Vec<&str> = version.split('.').collect();

        let mut count_numbers = 0;
        let mut numbers = vec![];

        for item in components {
            count_numbers += 1;
            if count_numbers > 3 {
                return Err(Error::TooManyComponents(count_numbers));
            }
            numbers.push(match item.parse::<usize>() {
                Ok(n) => n,
                Err(_) => return Err(Error::MustBeNumber(item.to_string())),
            });
        }

        if count_numbers < 3 {
            return Err(Error::TooFewComponents(count_numbers));
        }

        Ok(Semantic::new(
            version_prefix.to_string(),
            numbers[0],
            numbers[1],
            numbers[2],
        ))
    }

    /// Increment the version based on a breaking change
    /// When the major number is 0 increment the minor
    /// number else increment the major number
    ///
    pub fn breaking_increment(&mut self) -> &mut Self {
        if self.major == 0 {
            self.minor += 1;
            self.patch = 0;
        } else {
            self.major += 1;
            self.minor = 0;
            self.patch = 0;
        }
        self
    }

    /// Increment the patch component of the version number by 1
    ///
    pub fn increment_patch(&mut self) -> &mut Self {
        self.patch += 1;
        self
    }

    /// Increment the minor component of the version number by 1
    ///
    pub fn increment_minor(&mut self) -> &mut Self {
        self.minor += 1;
        self.patch = 0;
        self
    }

    /// Increment the major component of the version number by 1
    ///
    pub fn increment_major(&mut self) -> &mut Self {
        self.major += 1;
        self.minor = 0;
        self.patch = 0;
        self
    }

    /// Set the first production release version
    ///
    pub fn first_production(&mut self) -> Result<&mut Self, Error> {
        if 0 < self.major {
            return Err(Error::MajorAlreadyUsed(self.major.to_string()));
        } else {
            self.major = 1;
            self.minor = 0;
            self.patch = 0;
        }
        Ok(self)
    }

    /// Report the major version number
    ///
    pub fn major(&self) -> usize {
        self.major
    }
    /// Report the minor version number
    pub fn minor(&self) -> usize {
        self.minor
    }

    /// Report the patch version number
    ///
    pub fn patch(&self) -> usize {
        self.patch
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_semantic_version_number() {
        let version = Semantic::default();

        assert_eq!("0.0.0", &version.to_string());
    }

    #[test]
    fn bump_patch_version_number_by_one() {
        let mut version = Semantic::default();
        let updated_version = version.increment_patch();

        assert_eq!("0.0.1", &updated_version.to_string());
    }

    #[test]
    fn bump_minor_version_number_by_one() {
        let mut version = Semantic::default();
        let updated_version = version.increment_minor();

        assert_eq!("0.1.0", &updated_version.to_string());
    }

    #[test]
    fn bump_major_version_number_by_one() {
        let mut version = Semantic::default();
        let updated_version = version.increment_major();

        assert_eq!("1.0.0", &updated_version.to_string());
    }

    #[test]
    fn parse_valid_version_tag_to_new_semantic_struct() {
        let tag = "v0.3.90";
        let version_prefix = "v";
        let semantic = Semantic::parse(tag, version_prefix);

        claims::assert_ok!(&semantic);
        let semantic = match semantic {
            Ok(s) => s.to_string(),
            Err(e) => e.to_string(),
        };
        assert_eq!(tag, semantic);
    }

    #[test]
    fn parse_long_valid_version_tag_to_new_semantic_struct() {
        let tag = "Release Version 0.3.90";
        let version_prefix = "Release Version ";
        let semantic = Semantic::parse(tag, version_prefix);

        claims::assert_ok!(&semantic);
        let semantic = match semantic {
            Ok(s) => s.to_string(),
            Err(e) => e.to_string(),
        };
        assert_eq!(tag, semantic);
    }

    #[test]
    fn parse_error_failed_not_version_tag() {
        let tag = "0.3.90";
        let version_prefix = "v";
        let semantic = Semantic::parse(tag, version_prefix);

        claims::assert_err!(&semantic);
        let semantic = match semantic {
            Ok(s) => s.to_string(),
            Err(e) => e.to_string(),
        };
        assert_eq!(
            r#"Version tags must start with "v" but tag is 0.3.90"#,
            semantic
        );
    }

    #[test]
    fn parse_error_too_many_components() {
        let tag = "v0.3.90.8";
        let version_prefix = "v";
        let semantic = Semantic::parse(tag, version_prefix);

        claims::assert_err!(&semantic);
        let semantic = match semantic {
            Ok(s) => s.to_string(),
            Err(e) => e.to_string(),
        };
        assert_eq!(
            "Version must have three components but at least 4 were found",
            semantic
        );
    }

    #[test]
    fn parse_error_not_enough_components() {
        let tag = "v0.3";
        let version_prefix = "v";
        let semantic = Semantic::parse(tag, version_prefix);

        claims::assert_err!(&semantic);
        let semantic = match semantic {
            Ok(s) => s.to_string(),
            Err(e) => e.to_string(),
        };
        assert_eq!(
            "Version must have three components but only 2 found",
            semantic
        );
    }

    #[test]
    fn parse_error_version_must_be_a_number() {
        let tag = "v0.3.90-8";
        let version_prefix = "v";
        let semantic = Semantic::parse(tag, version_prefix);

        claims::assert_err!(&semantic);
        let semantic = match semantic {
            Ok(s) => s.to_string(),
            Err(e) => e.to_string(),
        };
        assert_eq!("Version must be a number but found 90-8", semantic);
    }
    // #[error("Version must be a number")]
    // MustBeNumber,
}
