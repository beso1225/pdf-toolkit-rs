use pdf::core::inspect_pdf;

#[test]
fn create_blank_a4_creates_single_page_pdf() {
    let dir = tempfile::tempdir().expect("tempdir");
    let output = dir.path().join("blank-a4.pdf");

    assert_cmd::Command::cargo_bin("pdf")
        .expect("binary")
        .args([
            "create",
            "blank",
            "--size",
            "A4",
            "-o",
            output.to_string_lossy().as_ref(),
        ])
        .assert()
        .success();

    let info = inspect_pdf(&output).expect("output readable");
    assert_eq!(info.page_count, 1);
}

#[test]
fn create_blank_custom_size_creates_single_page_pdf() {
    let dir = tempfile::tempdir().expect("tempdir");
    let output = dir.path().join("blank-custom.pdf");

    assert_cmd::Command::cargo_bin("pdf")
        .expect("binary")
        .args([
            "create",
            "blank",
            "--size",
            "400x300",
            "-o",
            output.to_string_lossy().as_ref(),
        ])
        .assert()
        .success();

    let info = inspect_pdf(&output).expect("output readable");
    assert_eq!(info.page_count, 1);
}

#[test]
fn create_blank_fails_for_invalid_size() {
    let dir = tempfile::tempdir().expect("tempdir");
    let output = dir.path().join("blank-invalid.pdf");

    assert_cmd::Command::cargo_bin("pdf")
        .expect("binary")
        .args([
            "create",
            "blank",
            "--size",
            "bad-size",
            "-o",
            output.to_string_lossy().as_ref(),
        ])
        .assert()
        .failure();
}
