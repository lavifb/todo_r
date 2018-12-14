Todo_r
===
[![Build Status](https://travis-ci.org/lavifb/todo_r.svg?branch=master)](https://travis-ci.org/lavifb/todo_r)

Todo_r is a simple rust command line utility that keeps track of your todo items in code.
It is pronounced "todoer" like someone that does todos.

A lot is adapted from [leasot](https://github.com/pgilad/leasot) but runs much faster.

<!-- TODO: rewrite overview -->
### Current support

* Only separate line comments are supported. So `statement; // TODO: this` is unsupported.
* Block comments like `/* TODO: this */` that stick to one line are supported.
* Custom tags are searched using the `-t` flag.
* Tagged user references
* Interactive mode for deleting comments is launched using the `-d` flag.

<!-- TODO: write about installation -->
<!-- ### Installation -->

### Language support

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

<!-- TODO: write about features -->
<!-- ### Features -->

---
written by Lavi Blumberg
