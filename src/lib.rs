#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs)]
#![cfg_attr(docsrs, warn(rustdoc::missing_doc_code_examples))]
#![cfg_attr(docsrs, warn(rustdoc::invalid_codeblock_attributes))]

//! Semantic Versioning Management
//!
//! Calculates the next semantic version number and level based on
//! the current version number and the conventional commits made
//! since the last version has been released.
//!
//! ## Usage
//!
//! Add the dependency to Cargo.toml
//!
//! ```toml
//!
//! [dependencies]
//! nextsv = "0.4.0"
//!
//! ```
//!
//! ```no_run
//! # fn main() -> Result<(), nextsv::Error> {
//!     use nextsv::VersionCalculator;
//!     let version_prefix = "v"; // Identifies a version tag
//!
//!     let latest_version = VersionCalculator::new(version_prefix)?;
//!
//!     let (next_version, next_level) = latest_version.commits()?.next_version();
//!
//!     println!("Next Version: {}\nNext Level: {}", next_version, next_level);
//!
//! #    Ok(())
//! # }
//! ```

mod calculator;
mod conventional;
mod error;
mod semantic;

pub use calculator::{ForceLevel, VersionCalculator};
pub(crate) use conventional::ConventionalCommits;
pub use error::Error;
pub use semantic::{Level, Semantic};
