extern crate assert_cmd;
extern crate escargot;
#[macro_use]
extern crate lazy_static;

use assert_cmd::prelude::*;
use escargot::CargoRun;
use std::process::Command;
use std::path::Path;
use std::fs;

lazy_static! {
	static ref CARGO_RUN: CargoRun = escargot::CargoBuild::new()
		.bin("todor")
		.current_release()
		.current_target()
		.run()
		.unwrap();
 }

fn todor() -> Command {
	let mut cmd = CARGO_RUN.command();
	cmd.current_dir("tests/inputs");
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
	let mut cmd = CARGO_RUN.command();
	cmd.current_dir("tests/inputs")
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
		.arg("-T")
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
		.arg("-T")
		.arg("todo")
		.arg("foo")
		.arg("tag")
		.assert()
		.success()
		.stdout("test1.rs\n  line 2      TODO   item\n  line 3      TAG    item tag\n  line 4      FOO    bar\n")
		.stderr("");
}

#[test]
fn custom_tags3() {
	todor()
		.arg("test1.rs")
		.arg("-t")
		.arg("foo")
		.assert()
		.success()
		.stdout("test1.rs\n  line 2      TODO   item\n  line 4      FOO    bar\n")
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
		.stderr("[31m[todor error][0m: '..' is a directory\n");
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
		.assert()
		.success()
		.stdout("test1.rs\n  line 4      FOO    bar\n  line 5      ITEM   item2\n")
		.stderr("");
}

#[test]
fn multiple() {
	todor()
		.arg("test1.rs")
		.arg("test2.py")
		.assert()
		.success()
		.stdout("test1.rs\n  line 2      TODO   item\ntest2.py\n  line 2      TODO   docstring comment\n  line 4      TODO   item\n")
		.stderr("");
}

#[test]
fn ignore() {
	todor()
		.arg("test1.rs")
		.arg("test2.py")
		.arg("-i")
		.arg("test2.py")
		.assert()
		.success()
		.stdout("test1.rs\n  line 2      TODO   item\n")
		.stderr("");
}

#[test]
fn config_ignore() {
	todor()
		.arg("test1.rs")
		.arg("test2.py")
		.arg("-c")
		.arg("config3.toml")
		.assert()
		.success()
		.stdout("test1.rs\n  line 2      TODO   item\n")
		.stderr("");
}

#[test]
fn init() {
	let mut cmd = CARGO_RUN.command();
	cmd.current_dir("tests/inputs")
		.arg("init")
		.assert()
		.success()
		.stdout("")
		.stderr("");

	let todor_config = Path::new("tests/inputs/.todor");
	// check that file is created
	assert!(todor_config.is_file(), true);

	// remove file
	fs::remove_file(todor_config).unwrap();
}

#[test]
fn walk1() {
	todor()
		.current_dir("tests")
		.assert()
		.success()
		.stdout("inputs/test1.rs\n  line 2      TODO   item\ninputs/test2.py\n  line 2      TODO   docstring comment\n  line 4      TODO   item\ninputt/test1.rs\n  line 1      TODO   item2\n")
		.stderr("");
}

#[test]
fn walk2() {
	todor()
		.current_dir("tests")
		.arg("-T")
		.arg("foo")
		.assert()
		.success()
		.stdout("inputs/test1.rs\n  line 4      FOO    bar\ninputt/test1.rs\n  line 3      FOO    bar2\n")
		.stderr("");
}

#[test]
fn walk3() {
	todor()
		.arg("-T")
		.arg("foo")
		.assert()
		.success()
		.stdout("test1.rs\n  line 4      FOO    bar\n../inputt/test1.rs\n  line 3      FOO    bar2\n")
		.stderr("");
}