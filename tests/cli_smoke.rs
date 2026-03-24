use assert_cmd::Command;
use predicates::str::contains;

#[test]
fn runs_without_args() {
    Command::cargo_bin("pdf")
        .expect("binary should build")
        .assert()
        .success();
}

#[test]
fn accepts_info_subcommand_shape() {
    Command::cargo_bin("pdf")
        .expect("binary should build")
        .args(["info", "tests/fixtures/minimal.pdf"])
        .assert()
        .success();
}

#[test]
fn help_mentions_spec_driven_toolkit() {
    Command::cargo_bin("pdf")
        .expect("binary should build")
        .arg("--help")
        .assert()
        .success()
        .stdout(contains("Spec-driven PDF toolkit in Rust"));
}
