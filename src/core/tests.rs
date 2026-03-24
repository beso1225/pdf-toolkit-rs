#[cfg(test)]
mod core_tests {
    use std::path::Path;

    use tempfile::tempdir;

    use super::super::{PdfError, PdfInfo, inspect_pdf, parse_page_ranges, write_simple_pdf};

    #[test]
    fn inspect_pdf_reads_minimal_fixture() {
        let dir = tempdir().expect("temp dir must be created");
        let file_path = dir.path().join("minimal.pdf");
        std::fs::write(&file_path, write_simple_pdf(1, "1.5")).expect("fixture should write");

        let info: PdfInfo = inspect_pdf(&file_path).expect("fixture should parse");
        assert_eq!(info.version, "1.5");
        assert_eq!(info.page_count, 1);
        assert!(!info.encrypted);
        assert!(info.title.is_none());
        assert!(info.author.is_none());
    }

    #[test]
    fn inspect_pdf_reports_missing_file_path() {
        let err = inspect_pdf(Path::new("tests/fixtures/does-not-exist.pdf"))
            .expect_err("missing path must error");
        let rendered = err.to_string();
        assert!(rendered.contains("does-not-exist.pdf"));
    }

    #[test]
    fn parse_page_ranges_handles_mixed_tokens() {
        let parsed = parse_page_ranges("1,3-5,8", 10).expect("must parse");
        assert_eq!(parsed, vec![1, 3, 4, 5, 8]);
    }

    #[test]
    fn parse_page_ranges_rejects_zero() {
        let err = parse_page_ranges("0,1", 10).expect_err("must reject zero");
        assert!(matches!(err, PdfError::InvalidPageRange { .. }));
    }
}
