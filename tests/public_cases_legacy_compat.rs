use std::{fs, path::Path};

use pdf::core::{inspect_pdf, write_simple_pdf};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct LegacyInfoCase {
    input: String,
    expect: LegacyInfoExpect,
}

#[derive(Debug, Deserialize)]
struct LegacyInfoExpect {
    page_count: usize,
    encrypted: bool,
}

#[derive(Debug, Deserialize)]
struct LegacyMergeCase {
    inputs: Vec<String>,
    expect: LegacyMergeExpect,
}

#[derive(Debug, Deserialize)]
struct LegacyMergeExpect {
    page_count: usize,
}

#[test]
fn legacy_pdfjs_minimal_info_case_stays_green() {
    let raw = fs::read_to_string("tests/public_cases/pdf_js_minimal_info_case.json")
        .expect("legacy case should exist");
    let case: LegacyInfoCase = serde_json::from_str(&raw).expect("legacy info case should parse");

    let path = Path::new(&case.input);
    if !path.exists() {
        fs::create_dir_all("tests/fixtures").expect("fixtures dir");
        fs::write(path, write_simple_pdf(case.expect.page_count, "1.1")).expect("fixture write");
    }

    let info = inspect_pdf(path).expect("info should parse");
    assert_eq!(info.page_count, case.expect.page_count);
    assert_eq!(info.encrypted, case.expect.encrypted);
}

#[test]
fn legacy_qpdf_merge_smoke_case_stays_green() {
    let raw = fs::read_to_string("tests/public_cases/qpdf_merge_smoke_case.json")
        .expect("legacy case should exist");
    let case: LegacyMergeCase = serde_json::from_str(&raw).expect("legacy merge case should parse");

    let dir = tempfile::tempdir().expect("tempdir");
    let output = dir.path().join("merged.pdf");

    let mut args: Vec<String> = vec!["merge".to_string()];
    for input in &case.inputs {
        let p = Path::new(input);
        if !p.exists() {
            fs::create_dir_all("tests/fixtures").expect("fixtures dir");
            fs::write(p, write_simple_pdf(1, "1.5")).expect("fixture write");
        }
        args.push(input.clone());
    }
    args.push("-o".to_string());
    args.push(output.to_string_lossy().to_string());

    let arg_refs: Vec<&str> = args.iter().map(String::as_str).collect();
    assert_cmd::Command::cargo_bin("pdf")
        .expect("binary")
        .args(arg_refs)
        .assert()
        .success();

    let info = inspect_pdf(&output).expect("merged output parse");
    assert_eq!(info.page_count, case.expect.page_count);
}
