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

## v0.5.0 (2018-12-16)
### Added
- user tagging
	- Types
		1. `// TODO(user): item`
		2. `// TODO: @user1 item @user2 @user3`
	- User tags are color highlighted in output
	- output only specific users using `-u` or `--user` flag
- regex caching to not rebuild the same regexs over and over again
- support for changing ANSI printing styles in config files

### Changed
- stderr output using `--verbose` flag

### Library changes
- debug statements using log crate
- pulled `bin/todor.rs` into separate files for potentially better compilation optimization
- moved `Todo` and `TodoFile` types into `todo` module
- pulled config related types out of `comments.rs` and into `configs.rs`


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
