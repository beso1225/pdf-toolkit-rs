use assert_cmd::Command;
use predicates::str::contains;
use tempfile::tempdir;

fn write_minimal_pdf(path: &std::path::Path) {
    std::fs::write(path, pdf::core::write_simple_pdf(1, "1.5")).expect("pdf fixture must write");
}

#[test]
fn runs_without_args() {
    Command::cargo_bin("pdf")
        .expect("binary should build")
        .assert()
        .success();
}

#[test]
fn accepts_info_subcommand_shape() {
    let dir = tempdir().expect("temp dir should be created");
    let file_path = dir.path().join("minimal.pdf");
    write_minimal_pdf(&file_path);

    Command::cargo_bin("pdf")
        .expect("binary should build")
        .args(["info", file_path.to_string_lossy().as_ref()])
        .assert()
        .success()
        .stdout(contains("version=1.5"))
        .stdout(contains("pages=1"))
        .stdout(contains("encrypted=false"));
}

#[test]
fn info_subcommand_fails_for_missing_file() {
    Command::cargo_bin("pdf")
        .expect("binary should build")
        .args(["info", "tests/fixtures/missing.pdf"])
        .assert()
        .failure()
        .stderr(contains("error[open_pdf]: failed to open PDF"));
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
