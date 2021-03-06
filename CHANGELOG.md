# Todo_r Changelog
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

## v0.7.3 (2020-01-17)
### Added
- Support for empty TODOs


## v0.7.2 (2019-02-10)
### Changed
- Global config file on MacOS now follows XDG standard so its location can be changed by setting `XDG_CONFIG_HOME`


## v0.7.1 (2019-01-14)
### Added
- improved help when using `-h` and `--help` tags


## v0.7.0 (2019-01-11)
### Added
- More formatting options for `-f` flag
	- `default` prints in the normal ANSI style
	- `usermarkdown` prints a markdown tables organized by tagged users
	- `csv` prints a csv table
- `-e` tag for reading piped content from stdin

### Changed
- filtering now occurs at the parsing stage instead of while printing
- rewrite of hashmap that handles extensions for a small performance gain.
- internal rewrite of formatting printing that may slightly improve performance

### Library changes
- all TodoR methods that with `_filtered_` in the name are removed. Instead filter while parsing using `open_filtered_todos()`.
- full rewrite of `printer.rs` and iterators of `Todo` and `TodoFile`
- renamed what is left of `printer` mod as `format`
- added `maps.rs` to handle specialized HashMaps


## v0.6.0 (2019-01-03)
### Added
- completions for bash, zsh, fish, and powershell
- formula for `brew` package manager
- global config support
- formatted output formats using `-f` flag
	- JSON
	- Pretty JSON
	- Markdown

### Changed
- ignore paths are now entirely handled by `todor` bin

### Deprecated
- `ignore` config option
	- use a `.todorignore` file or `-i` flag to ignore paths

### Fixed
- ANSI style support for numbered ANSI colors


## v0.5.1 (2018-12-19)
### Fixed
- output when tags are styled to be underlined
- `--check` output when you use `--user` to filter output


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


## v0.4.1 (2018-12-08)
### Added
- releases on Github using Travis CI


## v0.4 (2018-12-07)
- Initial release
