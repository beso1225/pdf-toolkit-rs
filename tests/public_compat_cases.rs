use std::{fs, path::Path};

use pdf::core::{inspect_pdf, parse_page_ranges, write_simple_pdf};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct RangeCase {
    input: String,
    max_page: usize,
    expect: RangeExpect,
}

#[derive(Debug, Deserialize)]
struct RangeExpect {
    pages: Vec<usize>,
}

#[derive(Debug, Deserialize)]
struct ExtractCase {
    input_pages: usize,
    range: String,
    expect: ExtractExpect,
}

#[derive(Debug, Deserialize)]
struct ExtractExpect {
    output_pages: usize,
}

#[derive(Debug, Deserialize)]
struct InfoCase {
    input_pages: usize,
    input_version: String,
    expect: InfoExpect,
}

#[derive(Debug, Deserialize)]
struct InfoExpect {
    version: String,
    pages: usize,
}

#[test]
fn pdfjs_range_dedupe_case_matches_behavior() {
    let raw = fs::read_to_string("tests/public_cases/pdfjs_range_dedupe_case.json")
        .expect("case file should exist");
    let case: RangeCase = serde_json::from_str(&raw).expect("case should parse");

    let pages = parse_page_ranges(&case.input, case.max_page).expect("range should parse");
    assert_eq!(pages, case.expect.pages);
}

#[test]
fn qpdf_extract_subset_case_matches_behavior() {
    let raw = fs::read_to_string("tests/public_cases/qpdf_extract_subset_case.json")
        .expect("case file should exist");
    let case: ExtractCase = serde_json::from_str(&raw).expect("case should parse");

    let dir = tempfile::tempdir().expect("tempdir");
    let input = dir.path().join("input.pdf");
    let output = dir.path().join("out.pdf");

    fs::write(&input, write_simple_pdf(case.input_pages, "1.5")).expect("input write");

    assert_cmd::Command::cargo_bin("pdf")
        .expect("binary")
        .args([
            "extract-pages",
            input.to_string_lossy().as_ref(),
            "--pages",
            &case.range,
            "-o",
            output.to_string_lossy().as_ref(),
        ])
        .assert()
        .success();

    let info = inspect_pdf(Path::new(&output)).expect("output parse");
    assert_eq!(info.page_count, case.expect.output_pages);
}

#[test]
fn pdfium_info_version_case_matches_behavior() {
    let raw = fs::read_to_string("tests/public_cases/pdfium_info_version_case.json")
        .expect("case file should exist");
    let case: InfoCase = serde_json::from_str(&raw).expect("case should parse");

    let dir = tempfile::tempdir().expect("tempdir");
    let input = dir.path().join("input.pdf");
    fs::write(
        &input,
        write_simple_pdf(case.input_pages, &case.input_version),
    )
    .expect("write");

    let info = inspect_pdf(Path::new(&input)).expect("input parse");
    assert_eq!(info.version, case.expect.version);
    assert_eq!(info.page_count, case.expect.pages);
}
