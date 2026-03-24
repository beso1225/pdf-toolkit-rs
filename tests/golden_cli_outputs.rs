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
        .stdout(contains("status=ok"))
        .stdout(contains("command=info"))
        .stdout(contains("version=1.5"))
        .stdout(contains("pages=2"))
        .stdout(contains("encrypted=false"))
        .stdout(contains("title="))
        .stdout(contains("author="))
        .stderr("");
}

#[test]
fn info_output_supports_json_format() {
    let dir = tempfile::tempdir().expect("tempdir");
    let input = dir.path().join("input.pdf");
    std::fs::write(&input, pdf::core::write_simple_pdf(2, "1.5")).expect("write fixture");

    Command::cargo_bin("pdf")
        .expect("binary")
        .args(["info", input.to_string_lossy().as_ref(), "--format", "json"])
        .assert()
        .success()
        .stdout(contains("\"status\":\"ok\""))
        .stdout(contains("\"command\":\"info\""))
        .stdout(contains("\"version\":\"1.5\""))
        .stdout(contains("\"pages\":2"))
        .stdout(contains("\"encrypted\":false"))
        .stderr("");
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
        .stdout(contains("status=ok"))
        .stdout(contains("command=extract-pages"))
        .stdout(contains("extracted_pages=2-3"))
        .stdout(contains("output="))
        .stderr("");
}

#[test]
fn merge_output_supports_json_format() {
    let dir = tempfile::tempdir().expect("tempdir");
    let a = dir.path().join("a.pdf");
    let b = dir.path().join("b.pdf");
    let out = dir.path().join("merged.pdf");
    std::fs::write(&a, pdf::core::write_simple_pdf(1, "1.5")).expect("write fixture");
    std::fs::write(&b, pdf::core::write_simple_pdf(2, "1.5")).expect("write fixture");

    Command::cargo_bin("pdf")
        .expect("binary")
        .args([
            "merge",
            a.to_string_lossy().as_ref(),
            b.to_string_lossy().as_ref(),
            "--format",
            "json",
            "-o",
            out.to_string_lossy().as_ref(),
        ])
        .assert()
        .success()
        .stdout(contains("\"status\":\"ok\""))
        .stdout(contains("\"command\":\"merge\""))
        .stdout(contains("\"merged_pages_source_count\":2"))
        .stderr("");
}

#[test]
fn split_output_supports_json_format() {
    let dir = tempfile::tempdir().expect("tempdir");
    let input = dir.path().join("input.pdf");
    std::fs::write(&input, pdf::core::write_simple_pdf(3, "1.5")).expect("write fixture");

    Command::cargo_bin("pdf")
        .expect("binary")
        .args([
            "split",
            input.to_string_lossy().as_ref(),
            "--by",
            "single",
            "--output-dir",
            dir.path().join("parts").to_string_lossy().as_ref(),
            "--format",
            "json",
        ])
        .assert()
        .success()
        .stdout(contains("\"status\":\"ok\""))
        .stdout(contains("\"command\":\"split\""))
        .stdout(contains("\"split_by\":\"single\""))
        .stdout(contains("\"parts\":3"))
        .stderr("");
}

#[test]
fn create_blank_output_supports_json_format() {
    let dir = tempfile::tempdir().expect("tempdir");
    let output = dir.path().join("blank.pdf");

    Command::cargo_bin("pdf")
        .expect("binary")
        .args([
            "create",
            "blank",
            "--size",
            "A4",
            "-o",
            output.to_string_lossy().as_ref(),
            "--format",
            "json",
        ])
        .assert()
        .success()
        .stdout(contains("\"status\":\"ok\""))
        .stdout(contains("\"command\":\"create-blank\""))
        .stdout(contains("\"created\":\"blank\""))
        .stderr("");
}

#[test]
fn missing_info_file_has_stable_error_prefix() {
    Command::cargo_bin("pdf")
        .expect("binary")
        .args(["info", "tests/fixtures/does-not-exist.pdf"])
        .assert()
        .failure()
        .stdout("")
        .stderr(contains("error[open_pdf]: failed to open PDF"));
}

#[test]
fn rotate_invalid_degrees_has_stable_error_prefix() {
    let dir = tempfile::tempdir().expect("tempdir");
    let input = dir.path().join("input.pdf");
    std::fs::write(&input, pdf::core::write_simple_pdf(3, "1.5")).expect("write fixture");

    Command::cargo_bin("pdf")
        .expect("binary")
        .args([
            "rotate-pages",
            input.to_string_lossy().as_ref(),
            "--pages",
            "1-2",
            "--deg",
            "45",
            "-o",
            dir.path().join("out.pdf").to_string_lossy().as_ref(),
        ])
        .assert()
        .failure()
        .stdout("")
        .stderr(contains(
            "error[invalid_rotation_degrees]: invalid rotation degrees",
        ));
}

#[test]
fn split_invalid_mode_has_stable_error_prefix() {
    let dir = tempfile::tempdir().expect("tempdir");
    let input = dir.path().join("input.pdf");
    std::fs::write(&input, pdf::core::write_simple_pdf(3, "1.5")).expect("write fixture");

    Command::cargo_bin("pdf")
        .expect("binary")
        .args([
            "split",
            input.to_string_lossy().as_ref(),
            "--by",
            "weird",
            "--output-dir",
            dir.path().join("parts").to_string_lossy().as_ref(),
        ])
        .assert()
        .failure()
        .stdout("")
        .stderr(contains("error[invalid_split_mode]: invalid split mode"));
}
