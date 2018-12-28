Todo_r
======
[![Build Status](https://travis-ci.org/lavifb/todo_r.svg?branch=master)](https://travis-ci.org/lavifb/todo_r)

### Find all your notes with one command!

Todo_r is a simple rust command line utility that keeps track of your todo items in code.
It is pronounced "todoer" like someone that does todos.

Find all your TODO notes with one command!

A lot is adapted from [leasot](https://github.com/pgilad/leasot) but runs much faster.

## Installation

The latest release can be downloaded from the releases page.

If you use macOS Homebrew or Linuxbrew you can currently install the latest version using
```console
$ brew tap lavifb/todo_r https://github.com/lavifb/todo_r.git
$ brew install todor
```

## Features

- Reads TODO comments that are on their own line.
```rust
// TODO: do this
/* TODO: do that */
```
Note: comments that are not on their own line are __not__ supported.

- User references are tracked and can be found using `--user` flag.
```rust
// TODO(user1): item
// TODO: tagging @user2 and @user3
// TODO(user1): @user3 both are also found!
```
Comments 1 and 3 are found with `todor -u user1`.

- Custom tags can be searched using the `-t` flag.
- Interactive mode for deleting comments is launched using the `-d` flag.
- If files are not provided for input, todo_r searches the entire git repository.
    - .gitignore files are respected
    - More ignores can be added using .todorignore files that use the same syntax
    - If you are not using git, you can instead use a .todor file in the root directory

## Config files
Create a .todor file in the root of your workspace with `todor init`.

`.todor` files can also used as a config file to set custom tags, comments types, output styles, etc.

Todo_r also supports a global config file at `~/.config/todor/todor.conf` for Mac/Linux and `~\AppData\Roaming\lavifb\todor\todor.conf` on Windows.

<!-- TODO: document config file features -->

## Default Language Support
These common languages are supported by default. 
More support can be added using config files above.

| Filetype    | Extensions          | Comment Types |
|-------------|---------------------|---------------|
|C/C++        |`.c`,`.h`,`.cpp`     |`//`,`/* */`   |
|C#           |`.cs`                |`//`,`/* */`   |
|CoffeeScript |`.coffee`            |`#`            |
|Go           |`.go`                |`//`,`/* */`   |
|Haskell      |`.hs`                |`--`           |
|HTML         |`.html`,`.htm`       |`<!-- -->`     |
|Java         |`.java`              |`//`,`/* */`   |
|JavaScript   |`.js`,`.es`,`.es6`   |`//`,`/* */`   |
|Obj-C/C++    |`.m`,`.mm`           |`//`,`/* */`   |
|Less         |`.less`              |`//`,`/* */`   |
|Markdown     |`.md`                |`<!-- -->`     |
|Perl         |`.pl`,`.pm`          |`#`            |
|PHP          |`.php`               |`//`,`/* */`   |
|Python       |`.py`                |`#`,`""" """`  |
|Ruby         |`.rb`                |`#`            |
|Rust         |`.rs`                |`//`,`/* */`   |
|Sass         |`.sass`,`scss`       |`//`,`/* */`   |
|Scala        |`.scala`             |`//`,`/* */`   |
|Shell        |`.sh`,`.bash`,`.zsh` |`#`            |
|SQL          |`.sql`               |`--`,`/* */`   |
|Stylus       |`.styl`              |`//`,`/* */`   |
|Swift        |`.swift`             |`//`,`/* */`   |
|TeX          |`.tex`               |`%`            |
|TypeScript   |`.ts`,`.tsx`         |`//`,`/* */`   |
|YAML         |`.yaml`,`.yml`       |`#`            |

If there are any more languages/extensions that you feel should supported by default, feel free to submit an issue/pull request.

---
written by Lavi Blumberg
