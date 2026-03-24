use pdf::core::{PdfError, parse_page_ranges};

#[test]
fn parses_single_numbers_and_ranges() {
    let parsed = parse_page_ranges("1,3-5,8", 10).expect("range should parse");
    assert_eq!(parsed, vec![1, 3, 4, 5, 8]);
}

#[test]
fn deduplicates_and_preserves_first_seen_order() {
    let parsed = parse_page_ranges("3,1-3,2,5", 10).expect("range should parse");
    assert_eq!(parsed, vec![3, 1, 2, 5]);
}

#[test]
fn rejects_zero_or_negative_pages() {
    let err = parse_page_ranges("0,1", 10).expect_err("zero page should fail");
    assert!(matches!(err, PdfError::InvalidPageRange { .. }));
}

#[test]
fn rejects_out_of_bounds_pages() {
    let err = parse_page_ranges("1,12", 10).expect_err("out of bounds should fail");
    assert!(matches!(err, PdfError::InvalidPageRange { .. }));
}

#[test]
fn rejects_descending_range() {
    let err = parse_page_ranges("5-2", 10).expect_err("descending range should fail");
    assert!(matches!(err, PdfError::InvalidPageRange { .. }));
}

#[test]
fn rejects_empty_input() {
    let err = parse_page_ranges("  ", 10).expect_err("empty range should fail");
    assert!(matches!(err, PdfError::InvalidPageRange { .. }));
}
