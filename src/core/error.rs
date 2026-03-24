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
}
