mod error;
mod ranges;
mod simple_pdf;
#[cfg(test)]
mod tests;
mod types;

pub use error::PdfError;
pub use ranges::parse_page_ranges;
pub use simple_pdf::{extract_pages, inspect_pdf, merge_pdfs, write_simple_pdf};
pub use types::PdfInfo;
