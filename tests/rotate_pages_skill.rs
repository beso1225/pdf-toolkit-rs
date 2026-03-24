use pdf::core::{inspect_pdf, write_simple_pdf};

#[test]
fn rotate_pages_rotates_selected_pages() {
    let dir = tempfile::tempdir().expect("tempdir");
    let input = dir.path().join("input.pdf");
    let output = dir.path().join("output.pdf");

    std::fs::write(&input, write_simple_pdf(5, "1.5")).expect("input fixture write");

    assert_cmd::Command::cargo_bin("pdf")
        .expect("binary")
        .args([
            "rotate-pages",
            input.to_string_lossy().as_ref(),
            "--pages",
            "2,4",
            "--deg",
            "90",
            "-o",
            output.to_string_lossy().as_ref(),
        ])
        .assert()
        .success();

    let info = inspect_pdf(&output).expect("output readable");
    assert_eq!(info.page_count, 5);

    let raw = std::fs::read_to_string(&output).expect("output text read");
    assert_eq!(raw.matches("/Rotate 90").count(), 2);
}

#[test]
fn rotate_pages_fails_for_invalid_degrees() {
    let dir = tempfile::tempdir().expect("tempdir");
    let input = dir.path().join("input.pdf");
    let output = dir.path().join("output.pdf");

    std::fs::write(&input, write_simple_pdf(3, "1.5")).expect("input fixture write");

    assert_cmd::Command::cargo_bin("pdf")
        .expect("binary")
        .args([
            "rotate-pages",
            input.to_string_lossy().as_ref(),
            "--pages",
            "1-2",
            "--deg",
            "45",
            "-o",
            output.to_string_lossy().as_ref(),
        ])
        .assert()
        .failure();
}
