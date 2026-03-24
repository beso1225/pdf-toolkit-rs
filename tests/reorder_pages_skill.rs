use pdf::core::{inspect_pdf, write_simple_pdf};

#[test]
fn reorder_pages_accepts_order_and_preserves_count() {
    let dir = tempfile::tempdir().expect("tempdir");
    let input = dir.path().join("input.pdf");
    let output = dir.path().join("output.pdf");

    std::fs::write(&input, write_simple_pdf(4, "1.5")).expect("input fixture write");

    assert_cmd::Command::cargo_bin("pdf")
        .expect("binary")
        .args([
            "reorder-pages",
            input.to_string_lossy().as_ref(),
            "--order",
            "4,2,1,3",
            "-o",
            output.to_string_lossy().as_ref(),
        ])
        .assert()
        .success();

    let info = inspect_pdf(&output).expect("output parse");
    assert_eq!(info.page_count, 4);
}

#[test]
fn reorder_pages_fails_for_out_of_bounds_order() {
    let dir = tempfile::tempdir().expect("tempdir");
    let input = dir.path().join("input.pdf");
    let output = dir.path().join("output.pdf");

    std::fs::write(&input, write_simple_pdf(3, "1.5")).expect("input fixture write");

    assert_cmd::Command::cargo_bin("pdf")
        .expect("binary")
        .args([
            "reorder-pages",
            input.to_string_lossy().as_ref(),
            "--order",
            "1,4",
            "-o",
            output.to_string_lossy().as_ref(),
        ])
        .assert()
        .failure();
}
