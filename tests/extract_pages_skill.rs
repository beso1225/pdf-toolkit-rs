use pdf::core::{inspect_pdf, write_simple_pdf};

#[test]
fn extract_pages_command_creates_selected_page_subset() {
    let dir = tempfile::tempdir().expect("tempdir");
    let input = dir.path().join("input.pdf");
    let output = dir.path().join("output.pdf");

    std::fs::write(&input, write_simple_pdf(5, "1.5")).expect("input fixture write");

    assert_cmd::Command::cargo_bin("pdf")
        .expect("binary")
        .args([
            "extract-pages",
            input.to_string_lossy().as_ref(),
            "--pages",
            "2,4-5",
            "-o",
            output.to_string_lossy().as_ref(),
        ])
        .assert()
        .success();

    let info = inspect_pdf(&output).expect("output must be readable");
    assert_eq!(info.page_count, 3);
}

#[test]
fn extract_pages_fails_for_out_of_bounds_range() {
    let dir = tempfile::tempdir().expect("tempdir");
    let input = dir.path().join("input.pdf");
    let output = dir.path().join("output.pdf");

    std::fs::write(&input, write_simple_pdf(3, "1.5")).expect("input fixture write");

    assert_cmd::Command::cargo_bin("pdf")
        .expect("binary")
        .args([
            "extract-pages",
            input.to_string_lossy().as_ref(),
            "--pages",
            "2,5",
            "-o",
            output.to_string_lossy().as_ref(),
        ])
        .assert()
        .failure();
}
