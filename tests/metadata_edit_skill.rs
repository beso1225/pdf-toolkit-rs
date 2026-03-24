use pdf::core::{inspect_pdf, write_simple_pdf};

#[test]
fn set_meta_updates_title_and_author_in_output() {
    let dir = tempfile::tempdir().expect("tempdir");
    let input = dir.path().join("input.pdf");
    let output = dir.path().join("output.pdf");

    std::fs::write(&input, write_simple_pdf(2, "1.5")).expect("input fixture write");

    assert_cmd::Command::cargo_bin("pdf")
        .expect("binary")
        .args([
            "set-meta",
            input.to_string_lossy().as_ref(),
            "--title",
            "Spec Driven",
            "--author",
            "Yutaro",
            "-o",
            output.to_string_lossy().as_ref(),
        ])
        .assert()
        .success();

    let info = inspect_pdf(&output).expect("output parse");
    assert_eq!(info.title.as_deref(), Some("Spec Driven"));
    assert_eq!(info.author.as_deref(), Some("Yutaro"));
    assert_eq!(info.page_count, 2);
}

#[test]
fn set_meta_requires_at_least_one_field() {
    let dir = tempfile::tempdir().expect("tempdir");
    let input = dir.path().join("input.pdf");
    let output = dir.path().join("output.pdf");

    std::fs::write(&input, write_simple_pdf(1, "1.5")).expect("input fixture write");

    assert_cmd::Command::cargo_bin("pdf")
        .expect("binary")
        .args([
            "set-meta",
            input.to_string_lossy().as_ref(),
            "-o",
            output.to_string_lossy().as_ref(),
        ])
        .assert()
        .failure();
}
