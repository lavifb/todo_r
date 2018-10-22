extern crate assert_cmd;

use assert_cmd::prelude::*;
use std::process::Command;

fn todor() -> Command {
    let mut cmd = Command::main_binary().unwrap();
    cmd.current_dir("tests/examples");
    cmd
}

#[test]
fn basic() {
    todor()
        .arg("test1.rs")
        .assert()
        .success()
        .stdout("test.rs\n  line 2\tTODO\titem\n")
        .stderr("");
}