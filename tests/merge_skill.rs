use std::path::Path;

fn write_pdf(path: &Path) {
    write_pdf_pages(path, 1);
}

fn write_pdf_pages(path: &Path, pages: usize) {
    std::fs::write(path, pdf::core::write_simple_pdf(pages, "1.5")).expect("fixture should save");
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

#[test]
fn merge_preserves_page_level_attributes_from_inputs() {
    let dir = tempfile::tempdir().expect("tempdir");
    let a = dir.path().join("a.pdf");
    let b_raw = dir.path().join("b-raw.pdf");
    let b = dir.path().join("b-rotated.pdf");
    write_pdf(&a);
    std::fs::write(&b_raw, pdf::core::write_simple_pdf(1, "1.5")).expect("write b raw");

    assert_cmd::Command::cargo_bin("pdf")
        .expect("bin")
        .args([
            "rotate-pages",
            b_raw.to_string_lossy().as_ref(),
            "--pages",
            "1",
            "--deg",
            "90",
            "-o",
            b.to_string_lossy().as_ref(),
        ])
        .assert()
        .success();

    let output = dir.path().join("merged.pdf");
    assert_cmd::Command::cargo_bin("pdf")
        .expect("bin")
        .args([
            "merge",
            a.to_string_lossy().as_ref(),
            b.to_string_lossy().as_ref(),
            "-o",
            output.to_string_lossy().as_ref(),
        ])
        .assert()
        .success();

    let merged = std::fs::read_to_string(&output).expect("merged output should exist");
    assert!(merged.contains("/Rotate 90"));
}

#[test]
fn merge_with_index_prepends_index_page_and_entries() {
    let dir = tempfile::tempdir().expect("tempdir");
    let a = dir.path().join("chapter-a.pdf");
    let b = dir.path().join("chapter-b.pdf");
    write_pdf_pages(&a, 2);
    write_pdf_pages(&b, 1);

    let output = dir.path().join("merged-index.pdf");
    assert_cmd::Command::cargo_bin("pdf")
        .expect("bin")
        .args([
            "merge",
            a.to_string_lossy().as_ref(),
            b.to_string_lossy().as_ref(),
            "--index",
            "-o",
            output.to_string_lossy().as_ref(),
        ])
        .assert()
        .success();

    let merged = std::fs::read_to_string(&output).expect("merged output should exist");
    assert_eq!(page_count_in_bytes(merged.as_bytes()), 4);
    assert!(merged.contains("/IndexEntry (chapter-a.pdf|2)"));
    assert!(merged.contains("/IndexEntry (chapter-b.pdf|4)"));
    assert!(merged.contains("/DestEntry (dest-1|2|chapter-a.pdf)"));
    assert!(merged.contains("/DestEntry (dest-2|4|chapter-b.pdf)"));
    assert!(merged.contains("/LinkAnnot (index-1|dest-1|chapter-a.pdf)"));
    assert!(merged.contains("/LinkAnnot (index-2|dest-2|chapter-b.pdf)"));
    assert!(merged.contains("/OutlineEntry (outline-1|dest-1|chapter-a.pdf)"));
    assert!(merged.contains("/OutlineEntry (outline-2|dest-2|chapter-b.pdf)"));
}

#[test]
fn merge_with_index_links_outlines_flags_controls_outputs() {
    let dir = tempfile::tempdir().expect("tempdir");
    let a = dir.path().join("a.pdf");
    let b = dir.path().join("b.pdf");
    write_pdf_pages(&a, 1);
    write_pdf_pages(&b, 1);

    let output = dir.path().join("merged-index-basic.pdf");
    assert_cmd::Command::cargo_bin("pdf")
        .expect("bin")
        .args([
            "merge",
            a.to_string_lossy().as_ref(),
            b.to_string_lossy().as_ref(),
            "--index",
            "--links=false",
            "--outlines=false",
            "-o",
            output.to_string_lossy().as_ref(),
        ])
        .assert()
        .success();

    let merged = std::fs::read_to_string(&output).expect("merged output should exist");
    assert!(merged.contains("/IndexEntry (a.pdf|2)"));
    assert!(merged.contains("/DestEntry (dest-1|2|a.pdf)"));
    assert!(!merged.contains("/LinkAnnot (index-1|dest-1|a.pdf)"));
    assert!(!merged.contains("/OutlineEntry (outline-1|dest-1|a.pdf)"));
}
