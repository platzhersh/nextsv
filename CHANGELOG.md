<!-- markdownlint-disable MD024 -->
# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- next-header -->

## [Unreleased] - ReleaseDate

## [0.7.4] - 2022-11-19

## [0.7.3] - 2022-11-05

## [0.7.2] - 2022-09-24

## [0.7.1] - 2022-09-18

## [0.7.0] - 2022-08-22

### Bug Fixes

- Files check as part of the calculation
- Pass vec and not reference to vec

### Features

- ✨ require switch in cli
- Multiple value flag on cli config
- Check that required files are listed
- ✨ have_required method for VersionCalculator
- NoConventionalCommits error
- Error if no commits in struct
- Error message will pass the filename
- MissingRequired File error
- No files listed and file list
- Has_required function
- Collect file names during walk
- Use diff to get file list as OsStrings
- Required_level to enforce

### Miscellaneous Tasks

- (ci) remove redundant rustup in docs job

### Refactor

- Simplify options
- Rename commits walk_commits
- Trace file names found
- Use HashSet

## [0.6.2] - 2022-08-20

## [0.6.1] - 2022-08-14

### Bug Fixes

- (docs) minimum rust release graphic

### Miscellaneous Tasks

- Release

## [0.6.0] - 2022-08-14

### Bug Fixes

- (docs) update min rust version to 1.60
- (crate) update rust-version to 1.60

### Features

- Custom image for execution environment

### Miscellaneous Tasks

- (ci) remove rustup
- Release

### Ci

- Remove installs included in custom executor

## [0.5.2] - 2022-08-08

### Bug Fixes

- Allow none as valid response
- Clippy lint on unused Level

### Miscellaneous Tasks

- If test publish only not none
- Release

### Ci

- Add else block to halt instead of fail.

## [0.5.1] - 2022-08-07

### Bug Fixes

- Registry must be a https:// link not a ssh link
- Correct specification of registry

### Miscellaneous Tasks

- (ci) update address for crates.io
- Release

### Bug Fixes

- Align documentation tests

### Features

- ✨ Add logging feature to crate
- ✨ Add logging to the CLI.
- ✨ Log the command running and errors
- ✨ Logging for calculator
- 🎨 Report level with   version number
- Exit with an error

### Miscellaneous Tasks

- 🎨 Check using nextsv to fail quickly
- Update Changelogs

### Refactor

- 🎨 Remove count fields from the struct
- 🎨 replace old methods with new
- 🎨 replace specific functions with generic in verbosity
- Tidy up use statement for nextsv
- Update version help text
- Update log messages
- Help text for CLI command level
- Simplify interface by removing the subcommands
- Single function to implement force options
- Use increment_counts
- Feature flags no longer required
- Update call to nextsv in CI

## [0.4.0] - 2022-07-31

### Bug Fixes

- Update rust crate clap to 3.2.11
- Update rust crate clap to 3.2.12
- Update rust crate git-conventional to 0.12.0
- Update rust crate clap to 3.2.13
- Update rust crate clap to 3.2.14
- 🐛 Spelling error in error text

### Features

- Create enum of bump levels
- ✨ add patch level of none when no conventional commits are found
- Instead of Level::None return and error NoLevelChange
- Add error for no level change

### Miscellaneous Tasks

- 🎨 Update changelogs
- Update github/codeql-action digest to d8c9c72
- Update ossf/scorecard-action digest to 88c5e32
- Update dependency cimg/rust to v1.62
- Update ossf/scorecard-action digest to d434c40
- Update ossf/scorecard-action digest to ccd0038
- Update github/codeql-action digest to ba95eeb
- Update github/codeql-action digest to b8bd06e
- Update ossf/scorecard-action digest to 0c37758
- Update github/codeql-action digest to 8171514
- Update ossf/scorecard-action digest to 3155d13
- ✨ Add workflow to check  for and release

## [0.3.1] - 2022-07-11

## Fix

- Errors found after cargo release run

## [0.3.0] - 2022-07-11

### Bug Fixes

- Fix errors in drafted Level code
- 🐛 replace tag identification using 'v' with prefix variable

### Documentation

- ✨ Commit based changelog using git cliff application

### Features

- Create enum of bump levels
- ✨ Features for calculation of level or version number
- ✨ Error for case where no conventional commits have been found
- ✨ function to calculate next level based on recent commits
- ✨ Implement display for semantic::Level

### Refactor

- 🎨 separate version calculation into a dedicated function version
- 🎨 move level printing code to separate function for level
- 🎨 Two subcommands for version and level output
- 🎨 Tidy off testing aids

## [0.2.0] - 2022-06-27

### Bug Fixes

- 🐛 Set lower components to 0 on increment

### Features

- ✨ cli based on clap with verbose setting
- ✨ force option on cli to force a specific level of update

### Miscellaneous Tasks

- 🔥 Remove dbg! macros
- 📝 Update release version in Cargo.toml to 0.1.1
- Update version in Cargo.toml to 0.2.0

## [0.1.1] - 2022-06-26

### Bug Fixes

- 🐛 Fix failure to detect separate tag and correct calculation of the next version
- 🐛 Test both other and fix_commits values for patch increment (major=0)

## [0.1.0] - 2022-06-25

### Documentation

- 📝 Update documentation for semantic module to refer to semver standard

### Features

- ✨ Add Semantic version struct and methods to display and increment components
- ✨ Add error module for nextsv library
- ✨ Add dependencies for error ,management
- ✨ add parse method to parse a git tag into a semantic version
- Count conventional commits to last tag
- ✨ abstraction for conventional commit
- ✨ describe a version tag
- Add module references to library and testing code in main, settings updates
- ✨ create function to calculate next semantic version

### Miscellaneous Tasks

- ✨ Initial announcement to reserve crate name
- Add CI to test and check the code
- Update security and changelog notices
- Add cargo release pre-release replacements

### Refactor

- 🎨 Refactor into library and binary
- Tuning updates

<!-- generated by git-cliff -->
<!-- next-url -->
[Unreleased]: https://github.com/jerusdp/nextsv/compare/v0.7.4...HEAD
[0.7.4]: https://github.com/jerusdp/nextsv/compare/v0.7.3...v0.7.4
[0.7.3]: https://github.com/jerusdp/nextsv/compare/v0.7.2...v0.7.3
[0.7.2]: https://github.com/jerusdp/nextsv/compare/v0.7.1...v0.7.2
[0.7.1]: https://github.com/jerusdp/nextsv/compare/v0.7.0...v0.7.1
[0.7.0]: <https://github.com/jerusdp/nextsv/compare/v0.6.2...v0.7.0>
[0.6.2]: <https://github.com/jerusdp/nextsv/compare/v0.6.1...v0.6.2>
[0.6.1]: <https://github.com/jerusdp/nextsv/compare/v0.6.0...v0.6.1>
[0.6.0]: <https://github.com/jerusdp/nextsv/compare/v0.5.2...v0.6.0>
[0.5.2]: <https://github.com/jerusdp/nextsv/compare/v0.5.1...v0.5.2>
[0.5.1]: <https://github.com/jerusdp/nextsv/compare/v0.5.0...v0.5.1>
[0.5.0]: <https://github.com/jerusdp/nextsv/compare/v0.4.0...v0.5.0>
[0.4.0]: <https://github.com/jerusdp/nextsv/compare/v0.3.1...V0.4.0>
[0.3.1]: <https://github.com/jerusdp/nextsv/compare/v0.3.0...v0.3.1>
[0.3.0]: <https://github.com/jerusdp/nextsv/compare/v0.2.0...v0.3.0>"
[0.2.0]: <https://github.com/jerudp/nextsv/compare/v0.1.1...v0.2.0>
[0.1.1]: <https://github.com/jerudp/nextsv/compare/v0.1.0...v0.1.1>
[0.1.0]: <https://github.com/jerudp/nextsv/compare/...v0.1.0>
