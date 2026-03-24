use pdf::core::{inspect_pdf, write_simple_pdf};

#[test]
fn remove_pages_command_removes_selected_subset() {
    let dir = tempfile::tempdir().expect("tempdir");
    let input = dir.path().join("input.pdf");
    let output = dir.path().join("output.pdf");

    std::fs::write(&input, write_simple_pdf(5, "1.5")).expect("input fixture write");

    assert_cmd::Command::cargo_bin("pdf")
        .expect("binary")
        .args([
            "remove-pages",
            input.to_string_lossy().as_ref(),
            "--pages",
            "2,4",
            "-o",
            output.to_string_lossy().as_ref(),
        ])
        .assert()
        .success();

    let info = inspect_pdf(&output).expect("output must be readable");
    assert_eq!(info.page_count, 3);
}

#[test]
fn remove_pages_fails_when_all_pages_are_removed() {
    let dir = tempfile::tempdir().expect("tempdir");
    let input = dir.path().join("input.pdf");
    let output = dir.path().join("output.pdf");

    std::fs::write(&input, write_simple_pdf(3, "1.5")).expect("input fixture write");

    assert_cmd::Command::cargo_bin("pdf")
        .expect("binary")
        .args([
            "remove-pages",
            input.to_string_lossy().as_ref(),
            "--pages",
            "1-3",
            "-o",
            output.to_string_lossy().as_ref(),
        ])
        .assert()
        .failure();
}
