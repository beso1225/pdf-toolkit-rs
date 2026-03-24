mod error;
mod pdf_engine;
mod ranges;
#[cfg(test)]
mod tests;
mod types;

pub use error::PdfError;
pub use pdf_engine::{
    create_blank, extract_pages, inspect_pdf, merge_pdfs, merge_pdfs_with_index,
    merge_pdfs_with_options, remove_pages, reorder_pages, rotate_pages, set_metadata, split_pdf,
    write_simple_pdf,
};
pub use ranges::parse_page_ranges;
pub use types::PdfInfo;
