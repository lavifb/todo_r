Todo_r
===

Todo_r is a simple rust command line utility that keeps track of your todo items in code.

A lot is adapted from [leasot](https://github.com/pgilad/leasot) but runs much faster.


### Current support

* Only separate line comments are supported. So `statement; // TODO: this` is unsupported.
* Block comments like `/* TODO: this */` that stick to one line are supported.
* Custom tags are searched using the `-t` flag.
* Interactive mode for deleting comments is launched using the `-d` flag.

### Language support

<!-- TODO: finish adding support to REAMDE -->
| Filetype | Extension           | Commnet Types |
|----------|---------------------|---------------|
|C/C++     |`.c`,`.h`,`.cpp`     |`//`,`/* */`   |
|C#        |`.cs`                |`//`,`/* */`   |
|Go        |`.go`                |`//`,`/* */`   |
|Python    |`.py`                |`#`,`""" """`  |
|Ruby      |`.rb`                |`#`            |
|Rust      |`.rs`                |`//`,`/* */`   |
|Shell     |`.sh`,`.bash`,`.zsh` |`#`            |

---
written by Lavi Blumberg
