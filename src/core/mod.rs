#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PdfInfo {
    pub version: String,
    pub page_count: usize,
    pub encrypted: bool,
}

pub fn inspect_pdf(_path: &str) -> Result<PdfInfo, String> {
    Err("not implemented".to_string())
}

#[cfg(test)]
mod tests {
    use super::inspect_pdf;

    #[test]
    fn inspect_pdf_is_explicitly_unimplemented_for_now() {
        let err = inspect_pdf("tests/fixtures/minimal.pdf").expect_err("must fail for baseline");
        assert!(err.contains("not implemented"));
    }
}
