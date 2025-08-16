use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn fails_when_running_as_root() {
    let mut cmd = Command::cargo_bin("lightshuttle-cli").unwrap();
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Refusing to run as root."));
}
