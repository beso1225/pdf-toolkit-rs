use std::path::Path;

fn write_pdf(path: &Path) {
    std::fs::write(path, pdf::core::write_simple_pdf(1, "1.5")).expect("fixture should save");
}

fn page_count_in_bytes(bytes: &[u8]) -> usize {
    let text = String::from_utf8_lossy(bytes);
    text.match_indices("/Type /Page")
        .filter(|(idx, _)| !text[*idx..].starts_with("/Type /Pages"))
        .count()
}

#[test]
fn merge_command_merges_two_single_page_pdfs() {
    let dir = tempfile::tempdir().expect("tempdir");
    let a = dir.path().join("a.pdf");
    let b = dir.path().join("b.pdf");
    write_pdf(&a);
    write_pdf(&b);

    let output = dir.path().join("merged.pdf");
    let assert = assert_cmd::Command::cargo_bin("pdf")
        .expect("bin")
        .args([
            "merge",
            a.to_string_lossy().as_ref(),
            b.to_string_lossy().as_ref(),
            "-o",
            output.to_string_lossy().as_ref(),
        ])
        .assert();

    assert.success();
    let merged = std::fs::read(&output).expect("merged output should exist");
    assert_eq!(page_count_in_bytes(&merged), 2);
}

#[test]
fn merge_command_fails_when_any_input_is_missing() {
    let dir = tempfile::tempdir().expect("tempdir");
    let a = dir.path().join("a.pdf");
    let missing = dir.path().join("missing.pdf");
    write_pdf(&a);

    let output = dir.path().join("merged.pdf");
    assert_cmd::Command::cargo_bin("pdf")
        .expect("bin")
        .args([
            "merge",
            a.to_string_lossy().as_ref(),
            missing.to_string_lossy().as_ref(),
            "-o",
            output.to_string_lossy().as_ref(),
        ])
        .assert()
        .failure();
}
