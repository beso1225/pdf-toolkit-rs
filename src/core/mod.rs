mod error;
mod ranges;
mod simple_pdf;
#[cfg(test)]
mod tests;
mod types;

pub use error::PdfError;
pub use ranges::parse_page_ranges;
pub use simple_pdf::{
    create_blank, extract_pages, inspect_pdf, merge_pdfs, merge_pdfs_with_index, remove_pages,
    reorder_pages, rotate_pages, set_metadata, split_pdf, write_simple_pdf,
};
pub use types::PdfInfo;
