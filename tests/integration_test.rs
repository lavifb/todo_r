extern crate assert_cmd;

use assert_cmd::prelude::*;
use std::process::Command;

fn todor() -> Command {
	let mut cmd = Command::main_binary().unwrap();
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
		.stdout("test1.rs\n  line 2\tTODO\titem\n")
		.stderr("");
}

#[test]
fn colors() {
	let mut cmd = Command::main_binary().unwrap();
	cmd.current_dir("tests/examples")
		.arg("test1.rs")
		.assert()
		.success()
		.stdout("[4mtest1.rs[0m\n  [38;5;8mline 2[0m\t[32mTODO[0m\t[36mitem[0m\n")
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
		.stdout("test1.rs\n  line 4\tFOO\tbar\n")
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
		.stdout("test1.rs\n  line 2\tTODO\titem\n  line 3\tTAG\titem tag\n  line 4\tFOO\tbar\n")
		.stderr("");
}

#[test]
fn py_extension_temp() {
	todor()
		.arg("test2.py")
		.assert()
		.success()
		.stdout("test2.py\n  line 4\tTODO\titem\n")
		.stderr("");
}

// #[test]
// TODO: implement comment blocks and then use this test instead of py_extension_temp()
fn py_extension() {
	todor()
		.arg("test2.py")
		.assert()
		.success()
		.stdout("test2.py\n  line 2\tTODO\tdocstring comment\n  line 4\tTODO\titem\n")
		.stderr("");
}