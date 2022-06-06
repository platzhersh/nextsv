//! Semantic Versioning Struct
//!
//! ...

use std::fmt;

use crate::Error;
/// The Semantic data structure represents a semantic version number.
#[derive(Debug, Default, PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
pub struct Semantic {
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
    // Create a new struct specifying each of the semantic version components.
    fn new(major: usize, minor: usize, patch: usize) -> Self {
        Semantic {
            major,
            minor,
            patch,
        }
    }
    /// Parse a tag and return a struct
    /// String format expect: vx.y.z
    pub fn parse(tag: &str) -> Result<Self, Error> {
        if !tag.starts_with('v') {
            return Err(Error::NotVersionTag(tag.to_string()));
        }
        let version = tag[1..].to_string();
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
            return Err(Error::NotEnoughComponents(count_numbers));
        }

        Ok(Semantic::new(numbers[0], numbers[1], numbers[2]))
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
    use super::*;

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

    #[test]
    fn parse_valid_version_tag_to_new_semantic_struct() {
        let tag = "v0.3.90";
        let semantic = Semantic::parse(tag);

        claim::assert_ok!(&semantic);
        let semantic = match semantic {
            Ok(s) => s.to_string(),
            Err(e) => e.to_string(),
        };
        assert_eq!(tag[1..], semantic);
    }

    #[test]
    fn parse_error_failed_not_version_tag() {
        let tag = "0.3.90";
        let semantic = Semantic::parse(tag);

        claim::assert_err!(&semantic);
        let semantic = match semantic {
            Ok(s) => s.to_string(),
            Err(e) => e.to_string(),
        };
        assert_eq!(
            "Version tags must start with the letter 'v' but tag is 0.3.90",
            semantic
        );
    }

    #[test]
    fn parse_error_too_many_components() {
        let tag = "v0.3.90.8";
        let semantic = Semantic::parse(tag);

        claim::assert_err!(&semantic);
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
        let semantic = Semantic::parse(tag);

        claim::assert_err!(&semantic);
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
        let semantic = Semantic::parse(tag);

        claim::assert_err!(&semantic);
        let semantic = match semantic {
            Ok(s) => s.to_string(),
            Err(e) => e.to_string(),
        };
        assert_eq!("Version must be a number but found 90-8", semantic);
    }
    // #[error("Version must be a number")]
    // MustBeNumber,
}
