#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PdfInfo {
    pub version: String,
    pub page_count: usize,
    pub encrypted: bool,
    pub title: Option<String>,
    pub author: Option<String>,
}
