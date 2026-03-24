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
        .write_stdin("quit\n")
        .assert()
        .success()
        .stdout(contains("PDF Toolkit Shell"))
        .stdout(contains("Type `help` for shell commands"))
        .stdout(contains("Try: info <file.pdf>"))
        .stdout(contains("Bye!"));
}

#[test]
fn interactive_shell_help_command_works() {
    Command::cargo_bin("pdf")
        .expect("binary should build")
        .write_stdin("help\nquit\n")
        .assert()
        .success()
        .stdout(contains("Shell commands:"))
        .stdout(contains("run <pdf-command>"))
        .stdout(contains("quit, exit"));
}

#[test]
fn interactive_shell_dispatches_info_command() {
    let dir = tempdir().expect("temp dir should be created");
    let file_path = dir.path().join("minimal.pdf");
    write_minimal_pdf(&file_path);

    Command::cargo_bin("pdf")
        .expect("binary should build")
        .write_stdin(format!("info {}\nquit\n", file_path.to_string_lossy()))
        .assert()
        .success()
        .stdout(contains("status=ok"))
        .stdout(contains("command=info"))
        .stdout(contains("version=1.5"));
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

#[test]
fn merge_help_includes_examples_and_flag_note() {
    Command::cargo_bin("pdf")
        .expect("binary should build")
        .args(["merge", "--help"])
        .assert()
        .success()
        .stdout(contains("Examples:"))
        .stdout(contains("pdf merge a.pdf b.pdf -o merged.pdf"))
        .stdout(contains(
            "Note: --links/--outlines are only effective when --index is enabled.",
        ));
}

#[test]
fn split_help_includes_mode_examples() {
    Command::cargo_bin("pdf")
        .expect("binary should build")
        .args(["split", "--help"])
        .assert()
        .success()
        .stdout(contains("Examples:"))
        .stdout(contains("--by single"))
        .stdout(contains("--by range:1-2,4-5"))
        .stdout(contains("--by chunk:3"));
}
