use std::time::{Duration, Instant};

#[test]
fn inspect_large_simple_pdf_is_reasonably_fast() {
    let dir = tempfile::tempdir().expect("tempdir");
    let input = dir.path().join("large.pdf");
    std::fs::write(&input, pdf::core::write_simple_pdf(1000, "1.5")).expect("write fixture");

    let start = Instant::now();
    let info = pdf::core::inspect_pdf(&input).expect("inspect should succeed");
    let elapsed = start.elapsed();

    assert_eq!(info.page_count, 1000);
    assert!(
        elapsed < Duration::from_secs(2),
        "inspect is too slow for simple large PDF: {:?}",
        elapsed
    );
}
