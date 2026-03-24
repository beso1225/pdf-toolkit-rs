use assert_cmd::Command;
use predicates::str::contains;

#[test]
fn info_output_has_stable_key_value_shape() {
    let dir = tempfile::tempdir().expect("tempdir");
    let input = dir.path().join("input.pdf");
    std::fs::write(&input, pdf::core::write_simple_pdf(2, "1.5")).expect("write fixture");

    Command::cargo_bin("pdf")
        .expect("binary")
        .args(["info", input.to_string_lossy().as_ref()])
        .assert()
        .success()
        .stdout(contains("version=1.5"))
        .stdout(contains("pages=2"))
        .stdout(contains("encrypted=false"))
        .stdout(contains("title="))
        .stdout(contains("author="));
}

#[test]
fn extract_output_mentions_selected_range_and_path() {
    let dir = tempfile::tempdir().expect("tempdir");
    let input = dir.path().join("input.pdf");
    let output = dir.path().join("out.pdf");
    std::fs::write(&input, pdf::core::write_simple_pdf(4, "1.5")).expect("write fixture");

    Command::cargo_bin("pdf")
        .expect("binary")
        .args([
            "extract-pages",
            input.to_string_lossy().as_ref(),
            "--pages",
            "2-3",
            "-o",
            output.to_string_lossy().as_ref(),
        ])
        .assert()
        .success()
        .stdout(contains("extracted_pages=2-3"))
        .stdout(contains("output="));
}

#[test]
fn missing_info_file_has_stable_error_prefix() {
    Command::cargo_bin("pdf")
        .expect("binary")
        .args(["info", "tests/fixtures/does-not-exist.pdf"])
        .assert()
        .failure()
        .stderr(contains("error: failed to open PDF"));
}
