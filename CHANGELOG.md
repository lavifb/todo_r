# todor Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- ## [Unreleased] -->
<!-- ### Added -->
<!-- ### Changed -->
<!-- ### Deprecated -->
<!-- ### Removed -->
<!-- ### Fixed -->
<!-- ### Security -->

## [Unreleased]
### Added
- debug statements using log crate

### Changed
- stderr output using `--verbose` flag
- pulled `bin/todor.rs` into separate files for potentially better compilation optimization

## v0.4.2 (2018-12-10)
### Added
- Windows release
- `--check` tag to exit nicely only when no TODOs are found

### Fixed
- [Windows]: path walker when files are not specified

## v0.4.1 (2018-12-8)
### Added
- releases on Github using Travis CI

## v0.4 (2018-12-7)
- Initial release
