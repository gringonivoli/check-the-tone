use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_invalid_argument() {
    let mut cmd = Command::cargo_bin("ctt").unwrap();
    cmd.arg("--invalid");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("unexpected argument"));
}

#[test]
#[ignore]
fn test_a_message() {
    let test_msg = "please help";
    let mut cmd = Command::cargo_bin("ctt").unwrap();
    cmd.args(["-m", test_msg]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Revised message:"));
}
