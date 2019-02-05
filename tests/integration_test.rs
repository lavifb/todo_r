// Integratino tests for todor

use assert_cmd::prelude::*;
use std::fs;
use std::path::Path;
use std::process::Command;

fn dir_sep() -> &'static str {
    if cfg!(windows) {
        "\\"
    } else {
        "/"
    }
}

fn todor() -> Command {
    let mut cmd = Command::cargo_bin("todor").unwrap();
    cmd.current_dir(format!("tests{0}inputs", dir_sep()));
    cmd.arg("--no-style");
    cmd
}

#[test]
fn basic() {
    todor()
        .arg("test1.rs")
        .assert()
        .success()
        .stdout(
            "test1.rs
  line 2      TODO   item\n",
        )
        .stderr("");
}

#[test]
fn colors() {
    let mut cmd = Command::cargo_bin("todor").unwrap();
    cmd.current_dir("tests/inputs")
        .arg("test1.rs")
        .assert()
        .success()
        .stdout(
            "[4mtest1.rs[0m
  [38;5;8mline 2    [0m  [32mTODO[0m   [36mitem[0m\n",
        )
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
        .stdout(
            "test1.rs
  line 4      FOO    bar\n",
        )
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
        .stdout(
            "test1.rs
  line 2      TODO   item
  line 3      TAG    item tag
  line 4      FOO    bar\n",
        )
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
        .stdout(
            "test1.rs
  line 2      TODO   item
  line 4      FOO    bar\n",
        )
        .stderr("");
}

#[test]
fn py_extension() {
    todor()
        .arg("test2.py")
        .assert()
        .success()
        .stdout(
            "test2.py
  line 2      TODO   docstring comment
  line 4      TODO   item\n",
        )
        .stderr("");
}

#[test]
fn dir_todos() {
    todor().arg("..").assert().success().stdout("").stderr("");
}

#[test]
fn config1() {
    todor()
        .arg("test1.rs")
        .arg("-c")
        .arg("config1.toml")
        .assert()
        .success()
        .stdout(
            "test1.rs
  line 2      TODO   item
  line 4      FOO    bar\n",
        )
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
        .stdout(
            "test1.rs
  line 4      FOO    bar
  line 5      ITEM   item2\n",
        )
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
        .stdout(
            "test1.rs
  line 4      FOO    bar
  line 5      ITEM   item2\n",
        )
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
        .stdout(
            "test1.rs
  line 4      FOO    bar
  line 5      ITEM   item2\n",
        )
        .stderr("");
}

#[test]
fn multiple() {
    todor()
        .arg("test1.rs")
        .arg("test2.py")
        .assert()
        .success()
        .stdout(
            "test1.rs
  line 2      TODO   item
test2.py
  line 2      TODO   docstring comment
  line 4      TODO   item\n",
        )
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
        .stdout(
            "test1.rs
  line 2      TODO   item\n",
        )
        .stderr("");
}

#[test]
fn init() {
    let mut cmd = Command::cargo_bin("todor").unwrap();
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
        .stdout(format!(
            "inputs{0}test1.rs
  line 2      TODO   item
inputs{0}test2.py
  line 2      TODO   docstring comment
  line 4      TODO   item
inputt{0}test1.rs
  line 1      TODO   item2\n",
            dir_sep()
        ))
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
        .stdout(format!(
            "inputs{0}test1.rs
  line 4      FOO    bar
inputt{0}test1.rs
  line 3      FOO    bar2\n",
            dir_sep()
        ))
        .stderr("");
}

#[test]
fn walk3() {
    todor()
        .arg("-T")
        .arg("foo")
        .assert()
        .success()
        .stdout(format!(
            "test1.rs
  line 4      FOO    bar
..{0}inputt{0}test1.rs
  line 3      FOO    bar2\n",
            dir_sep()
        ))
        .stderr("");
}

#[test]
fn check1() {
    todor().arg("--check").assert().failure();
}

#[test]
fn check2() {
    todor()
        .arg("--check")
        .arg("-T")
        .arg("none")
        .assert()
        .success();
}

#[test]
fn users() {
    todor()
        .current_dir("tests/inputt")
        .arg("ignore_this.rs")
        .assert()
        .success()
        .stdout(
            "ignore_this.rs
  line 1      TODO   @user1 ignore1
  line 2      TODO   @user1 ignore2 @user2
  line 3      TODO   ignore3 @user1 @user3\n",
        )
        .stderr("");
}

#[test]
fn users_color() {
    let mut cmd = Command::cargo_bin("todor").unwrap();
    cmd.current_dir("tests/inputt")
        .arg("ignore_this.rs")
        .assert()
        .success()
        .stdout(
            "[4mignore_this.rs[0m
  [38;5;8mline 1    [0m  [32mTODO[0m   [36m[38;5;8m@user1[36m ignore1[0m
  [38;5;8mline 2    [0m  [32mTODO[0m   [36m[38;5;8m@user1[36m ignore2 [38;5;8m@user2[36m[0m
  [38;5;8mline 3    [0m  [32mTODO[0m   [36mignore3 [38;5;8m@user1[36m [38;5;8m@user3[36m[0m\n")
        .stderr("");
}

#[test]
fn select_users1() {
    todor()
        .current_dir("tests/inputt")
        .arg("ignore_this.rs")
        .arg("-u")
        .arg("user2")
        .assert()
        .success()
        .stdout(
            "ignore_this.rs
  line 2      TODO   @user1 ignore2 @user2\n",
        )
        .stderr("");
}

#[test]
fn select_users2() {
    todor()
        .current_dir("tests/inputt")
        .arg("ignore_this.rs")
        .arg("-u")
        .arg("user2")
        .arg("user3")
        .assert()
        .success()
        .stdout(
            "ignore_this.rs
  line 2      TODO   @user1 ignore2 @user2
  line 3      TODO   ignore3 @user1 @user3\n",
        )
        .stderr("");
}

#[test]
fn json() {
    todor()
        .current_dir("tests/inputs")
        .arg("test1.rs")
        .arg("-f")
        .arg("json")
        .assert()
        .success()
        .stdout(r#"[{"file":"test1.rs","line":2,"tag":"TODO","text":"item","users":[]}]"#)
        .stderr("");
}

#[test]
fn json_check1() {
    todor()
        .current_dir("tests/inputs")
        .arg("test1.rs")
        .arg("-f")
        .arg("json")
        .arg("--check")
        .assert()
        .failure()
        .stdout(r#"[{"file":"test1.rs","line":2,"tag":"TODO","text":"item","users":[]}]"#)
        .stderr("");
}

#[test]
fn json_check2() {
    todor()
        .current_dir("tests/inputs")
        .arg("test1.rs")
        .arg("-f")
        .arg("json")
        .arg("-T")
        .arg("TOO")
        .arg("--check")
        .assert()
        .success()
        .stdout("[]")
        .stderr("");
}
