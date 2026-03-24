use std::io;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum PdfError {
    #[error("failed to open PDF at `{path}`: {source}")]
    OpenPdf { path: String, source: io::Error },
    #[error("failed to parse PDF at `{path}`: {reason}")]
    ParsePdf { path: String, reason: String },
    #[error("merge requires at least two input files")]
    MergeRequiresMultipleInputs,
    #[error("failed to save merged PDF to `{path}`: {source}")]
    SavePdf { path: String, source: io::Error },
    #[error("invalid page range `{input}`: {reason}")]
    InvalidPageRange { input: String, reason: String },
    #[error("remove-pages would remove all pages from the document")]
    RemoveAllPagesForbidden,
    #[error("invalid rotation degrees `{degrees}`: allowed values are 90, 180, 270")]
    InvalidRotationDegrees { degrees: i32 },
    #[error("invalid blank page size `{size}`; expected A4, Letter, or WxH (e.g., 400x300)")]
    InvalidBlankSize { size: String },
    #[error("set-meta requires at least one metadata field (title or author)")]
    MetadataRequiresField,
    #[error("invalid split mode `{mode}`; expected `single`, `range:<ranges>`, or `chunk:<size>`")]
    InvalidSplitMode { mode: String },
    #[error("merge options `--links` or `--outlines` require `--index`")]
    MergeIndexRequiredForNavOptions,
}

impl PdfError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::OpenPdf { .. } => "open_pdf",
            Self::ParsePdf { .. } => "parse_pdf",
            Self::MergeRequiresMultipleInputs => "merge_requires_multiple_inputs",
            Self::SavePdf { .. } => "save_pdf",
            Self::InvalidPageRange { .. } => "invalid_page_range",
            Self::RemoveAllPagesForbidden => "remove_all_pages_forbidden",
            Self::InvalidRotationDegrees { .. } => "invalid_rotation_degrees",
            Self::InvalidBlankSize { .. } => "invalid_blank_size",
            Self::MetadataRequiresField => "metadata_requires_field",
            Self::InvalidSplitMode { .. } => "invalid_split_mode",
            Self::MergeIndexRequiredForNavOptions => "merge_index_required_for_nav_options",
        }
    }
}
