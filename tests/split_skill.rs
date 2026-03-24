use pdf::core::{inspect_pdf, write_simple_pdf};

#[test]
fn split_single_creates_one_file_per_page() {
    let dir = tempfile::tempdir().expect("tempdir");
    let input = dir.path().join("input.pdf");
    let out_dir = dir.path().join("parts");

    std::fs::write(&input, write_simple_pdf(3, "1.5")).expect("input fixture write");

    assert_cmd::Command::cargo_bin("pdf")
        .expect("binary")
        .args([
            "split",
            input.to_string_lossy().as_ref(),
            "--by",
            "single",
            "--output-dir",
            out_dir.to_string_lossy().as_ref(),
        ])
        .assert()
        .success();

    for i in 1..=3 {
        let p = out_dir.join(format!("part-{i}.pdf"));
        let info = inspect_pdf(&p).expect("part readable");
        assert_eq!(info.page_count, 1);
    }
}

#[test]
fn split_range_creates_files_for_each_range_group() {
    let dir = tempfile::tempdir().expect("tempdir");
    let input = dir.path().join("input.pdf");
    let out_dir = dir.path().join("parts");

    std::fs::write(&input, write_simple_pdf(5, "1.5")).expect("input fixture write");

    assert_cmd::Command::cargo_bin("pdf")
        .expect("binary")
        .args([
            "split",
            input.to_string_lossy().as_ref(),
            "--by",
            "range:1-2,4-5",
            "--output-dir",
            out_dir.to_string_lossy().as_ref(),
        ])
        .assert()
        .success();

    let part1 = inspect_pdf(&out_dir.join("part-1.pdf")).expect("part 1 readable");
    let part2 = inspect_pdf(&out_dir.join("part-2.pdf")).expect("part 2 readable");
    assert_eq!(part1.page_count, 2);
    assert_eq!(part2.page_count, 2);
}

#[test]
fn split_chunk_creates_chunked_parts() {
    let dir = tempfile::tempdir().expect("tempdir");
    let input = dir.path().join("input.pdf");
    let out_dir = dir.path().join("parts");

    std::fs::write(&input, write_simple_pdf(5, "1.5")).expect("input fixture write");

    assert_cmd::Command::cargo_bin("pdf")
        .expect("binary")
        .args([
            "split",
            input.to_string_lossy().as_ref(),
            "--by",
            "chunk:2",
            "--output-dir",
            out_dir.to_string_lossy().as_ref(),
        ])
        .assert()
        .success();

    let c1 = inspect_pdf(&out_dir.join("part-1.pdf")).expect("part 1 readable");
    let c2 = inspect_pdf(&out_dir.join("part-2.pdf")).expect("part 2 readable");
    let c3 = inspect_pdf(&out_dir.join("part-3.pdf")).expect("part 3 readable");
    assert_eq!(c1.page_count, 2);
    assert_eq!(c2.page_count, 2);
    assert_eq!(c3.page_count, 1);
}

#[test]
fn split_fails_for_invalid_mode() {
    let dir = tempfile::tempdir().expect("tempdir");
    let input = dir.path().join("input.pdf");
    let out_dir = dir.path().join("parts");

    std::fs::write(&input, write_simple_pdf(3, "1.5")).expect("input fixture write");

    assert_cmd::Command::cargo_bin("pdf")
        .expect("binary")
        .args([
            "split",
            input.to_string_lossy().as_ref(),
            "--by",
            "weird",
            "--output-dir",
            out_dir.to_string_lossy().as_ref(),
        ])
        .assert()
        .failure();
}

#[test]
fn split_chunk_mode_is_case_insensitive() {
    let dir = tempfile::tempdir().expect("tempdir");
    let input = dir.path().join("input.pdf");
    let out_dir = dir.path().join("parts");

    std::fs::write(&input, write_simple_pdf(5, "1.5")).expect("input fixture write");

    assert_cmd::Command::cargo_bin("pdf")
        .expect("binary")
        .args([
            "split",
            input.to_string_lossy().as_ref(),
            "--by",
            "CHUNK:2",
            "--output-dir",
            out_dir.to_string_lossy().as_ref(),
        ])
        .assert()
        .success();

    let c1 = inspect_pdf(&out_dir.join("part-1.pdf")).expect("part 1 readable");
    let c2 = inspect_pdf(&out_dir.join("part-2.pdf")).expect("part 2 readable");
    let c3 = inspect_pdf(&out_dir.join("part-3.pdf")).expect("part 3 readable");
    assert_eq!(c1.page_count, 2);
    assert_eq!(c2.page_count, 2);
    assert_eq!(c3.page_count, 1);
}

#[test]
fn split_range_mode_is_case_insensitive() {
    let dir = tempfile::tempdir().expect("tempdir");
    let input = dir.path().join("input.pdf");
    let out_dir = dir.path().join("parts");

    std::fs::write(&input, write_simple_pdf(5, "1.5")).expect("input fixture write");

    assert_cmd::Command::cargo_bin("pdf")
        .expect("binary")
        .args([
            "split",
            input.to_string_lossy().as_ref(),
            "--by",
            "RANGE:1-2,4-5",
            "--output-dir",
            out_dir.to_string_lossy().as_ref(),
        ])
        .assert()
        .success();

    let part1 = inspect_pdf(&out_dir.join("part-1.pdf")).expect("part 1 readable");
    let part2 = inspect_pdf(&out_dir.join("part-2.pdf")).expect("part 2 readable");
    assert_eq!(part1.page_count, 2);
    assert_eq!(part2.page_count, 2);
}

#[test]
fn split_chunk_mode_rejects_zero_with_stable_error_code() {
    let dir = tempfile::tempdir().expect("tempdir");
    let input = dir.path().join("input.pdf");
    let out_dir = dir.path().join("parts");

    std::fs::write(&input, write_simple_pdf(3, "1.5")).expect("input fixture write");

    assert_cmd::Command::cargo_bin("pdf")
        .expect("binary")
        .args([
            "split",
            input.to_string_lossy().as_ref(),
            "--by",
            "chunk:0",
            "--output-dir",
            out_dir.to_string_lossy().as_ref(),
        ])
        .assert()
        .failure()
        .stderr(predicates::str::contains("error[invalid_split_mode]:"));
}
