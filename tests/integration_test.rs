extern crate assert_cmd;
extern crate escargot;
#[macro_use]
extern crate lazy_static;

use assert_cmd::prelude::*;
use escargot::CargoRun;
use std::process::Command;

lazy_static! {
	static ref CARGO_RUN: CargoRun = escargot::CargoBuild::new()
		.bin("todor")
		.current_release()
		// .current_target() // No difference in targets so this is omitted so ther is no recompilation
		.run()
		.unwrap();
 }

fn todor() -> Command {
	let mut cmd = CARGO_RUN.command();
	cmd.current_dir("tests/examples");
	cmd.arg("--no-style");
	cmd
}

#[test]
fn basic() {
	todor()
		.arg("test1.rs")
		.assert()
		.success()
		.stdout("test1.rs\n  line 2      TODO   item\n")
		.stderr("");
}

#[test]
fn colors() {
	let mut cmd = Command::main_binary().unwrap();
	cmd.current_dir("tests/examples")
		.arg("test1.rs")
		.assert()
		.success()
		.stdout("[4mtest1.rs[0m\n  [38;5;8mline 2    [0m  [32mTODO [0m  [36mitem[0m\n")
		.stderr("");
}

#[test]
fn custom_tags1() {
	todor()
		.arg("test1.rs")
		.arg("-t")
		.arg("foo")
		.assert()
		.success()
		.stdout("test1.rs\n  line 4      FOO    bar\n")
		.stderr("");
}

#[test]
fn custom_tags2() {
	todor()
		.arg("test1.rs")
		.arg("-t")
		.arg("todo")
		.arg("foo")
		.arg("tag")
		.assert()
		.success()
		.stdout("test1.rs\n  line 2      TODO   item\n  line 3      TAG    item tag\n  line 4      FOO    bar\n")
		.stderr("");
}

#[test]
fn py_extension() {
	todor()
		.arg("test2.py")
		.assert()
		.success()
		.stdout("test2.py\n  line 2      TODO   docstring comment\n  line 4      TODO   item\n")
		.stderr("");
}

#[test]
fn dir_todos() {
	todor()
		.arg("..")
		.assert()
		.success()
		.stdout("")
		.stderr("[31m[todo_r error][0m: '..' is a directory.\n");
}

#[test]
fn config1() {
	todor()
		.arg("test1.rs")
		.arg("-c")
		.arg("config1.toml")
		.assert()
		.success()
		.stdout("test1.rs\n  line 2      TODO   item\n  line 4      FOO    bar\n")
		.stderr("");
}

#[test]
fn config2() {
	todor()
		.arg("test1.rs")
		.arg("-c")
		.arg("config2.toml")
		.arg("-T")
		.assert()
		.success()
		.stdout("test1.rs\n  line 4      FOO    bar\n  line 5      ITEM   item2\n")
		.stderr("");
}

#[test]
fn config2json() {
	todor()
		.arg("test1.rs")
		.arg("-c")
		.arg("config2.json")
		.arg("-T")
		.assert()
		.success()
		.stdout("test1.rs\n  line 4      FOO    bar\n  line 5      ITEM   item2\n")
		.stderr("");
}

#[test]
fn config2yaml() {
	todor()
		.arg("test1.rs")
		.arg("-c")
		.arg("config2.yaml")
		.arg("-T")
		.assert()
		.success()
		.stdout("test1.rs\n  line 4      FOO    bar\n  line 5      ITEM   item2\n")
		.stderr("");
}