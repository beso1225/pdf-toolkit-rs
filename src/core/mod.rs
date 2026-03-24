use std::{collections::HashSet, fs, io, path::Path};

use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PdfInfo {
    pub version: String,
    pub page_count: usize,
    pub encrypted: bool,
    pub title: Option<String>,
    pub author: Option<String>,
}

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

pub fn inspect_pdf(path: &Path) -> Result<PdfInfo, PdfError> {
    let bytes = fs::read(path).map_err(|source| PdfError::OpenPdf {
        path: path.display().to_string(),
        source,
    })?;
    inspect_pdf_bytes(path, &bytes)
}

pub fn merge_pdfs(inputs: &[&Path], output: &Path) -> Result<(), PdfError> {
    if inputs.len() < 2 {
        return Err(PdfError::MergeRequiresMultipleInputs);
    }

    let mut page_total = 0usize;
    for input in inputs {
        let info = inspect_pdf(input)?;
        page_total += info.page_count;
    }

    let out = write_simple_pdf(page_total, "1.5");
    fs::write(output, out).map_err(|source| PdfError::SavePdf {
        path: output.display().to_string(),
        source,
    })?;
    Ok(())
}

fn inspect_pdf_bytes(path: &Path, bytes: &[u8]) -> Result<PdfInfo, PdfError> {
    let text = String::from_utf8_lossy(bytes);
    let Some(version) = extract_version(&text) else {
        return Err(PdfError::ParsePdf {
            path: path.display().to_string(),
            reason: "missing PDF header".to_string(),
        });
    };

    let page_count = count_pages(&text);
    if page_count == 0 {
        return Err(PdfError::ParsePdf {
            path: path.display().to_string(),
            reason: "no page objects found".to_string(),
        });
    }

    let encrypted = text.contains("/Encrypt");
    let title = extract_info_value(&text, "Title");
    let author = extract_info_value(&text, "Author");

    Ok(PdfInfo {
        version,
        page_count,
        encrypted,
        title,
        author,
    })
}

fn extract_version(text: &str) -> Option<String> {
    let first_line = text.lines().next()?;
    let version = first_line.strip_prefix("%PDF-")?;
    let trimmed = version.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

fn count_pages(text: &str) -> usize {
    text.match_indices("/Type /Page")
        .filter(|(idx, _)| !text[*idx..].starts_with("/Type /Pages"))
        .count()
}

fn extract_info_value(text: &str, key: &str) -> Option<String> {
    let token = format!("/{key} (");
    let start = text.find(&token)? + token.len();
    let rest = &text[start..];
    let end = rest.find(')')?;
    let value = rest[..end].trim();
    if value.is_empty() {
        None
    } else {
        Some(value.to_string())
    }
}

pub fn parse_page_ranges(input: &str, max_page: usize) -> Result<Vec<usize>, PdfError> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(PdfError::InvalidPageRange {
            input: input.to_string(),
            reason: "range cannot be empty".to_string(),
        });
    }
    if max_page == 0 {
        return Err(PdfError::InvalidPageRange {
            input: input.to_string(),
            reason: "max page must be greater than zero".to_string(),
        });
    }

    let mut result = Vec::new();
    let mut seen = HashSet::new();

    for part in trimmed.split(',') {
        let token = part.trim();
        if token.is_empty() {
            return Err(PdfError::InvalidPageRange {
                input: input.to_string(),
                reason: "contains empty segment".to_string(),
            });
        }

        if let Some((start_s, end_s)) = token.split_once('-') {
            let start = parse_positive_page(start_s.trim(), input)?;
            let end = parse_positive_page(end_s.trim(), input)?;
            if start > end {
                return Err(PdfError::InvalidPageRange {
                    input: input.to_string(),
                    reason: "range start must be <= end".to_string(),
                });
            }
            if end > max_page {
                return Err(PdfError::InvalidPageRange {
                    input: input.to_string(),
                    reason: format!("page {end} exceeds max page {max_page}"),
                });
            }
            for page in start..=end {
                if seen.insert(page) {
                    result.push(page);
                }
            }
        } else {
            let page = parse_positive_page(token, input)?;
            if page > max_page {
                return Err(PdfError::InvalidPageRange {
                    input: input.to_string(),
                    reason: format!("page {page} exceeds max page {max_page}"),
                });
            }
            if seen.insert(page) {
                result.push(page);
            }
        }
    }

    Ok(result)
}

fn parse_positive_page(token: &str, input: &str) -> Result<usize, PdfError> {
    let page = token
        .parse::<usize>()
        .map_err(|_| PdfError::InvalidPageRange {
            input: input.to_string(),
            reason: format!("`{token}` is not a positive page number"),
        })?;
    if page == 0 {
        return Err(PdfError::InvalidPageRange {
            input: input.to_string(),
            reason: "page numbers must start at 1".to_string(),
        });
    }
    Ok(page)
}

pub fn write_simple_pdf(page_count: usize, version: &str) -> Vec<u8> {
    let mut objects = Vec::new();
    let mut kids = Vec::new();

    objects.push("1 0 obj\n<< /Type /Catalog /Pages 2 0 R >>\nendobj\n".to_string());
    for i in 0..page_count {
        let page_id = 3 + i;
        kids.push(format!("{page_id} 0 R"));
    }
    objects.push(format!(
        "2 0 obj\n<< /Type /Pages /Kids [{}] /Count {} >>\nendobj\n",
        kids.join(" "),
        page_count
    ));

    for i in 0..page_count {
        let page_id = 3 + i;
        objects.push(format!(
            "{page_id} 0 obj\n<< /Type /Page /Parent 2 0 R /MediaBox [0 0 200 200] >>\nendobj\n"
        ));
    }

    let mut out = format!("%PDF-{version}\n");
    let mut offsets = vec![0usize];
    for obj in &objects {
        offsets.push(out.len());
        out.push_str(obj);
    }
    let xref_start = out.len();
    out.push_str(&format!("xref\n0 {}\n", offsets.len()));
    out.push_str("0000000000 65535 f \n");
    for offset in offsets.iter().skip(1) {
        out.push_str(&format!("{offset:010} 00000 n \n"));
    }
    out.push_str(&format!(
        "trailer\n<< /Root 1 0 R /Size {} >>\nstartxref\n{}\n%%EOF\n",
        offsets.len(),
        xref_start
    ));
    out.into_bytes()
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use tempfile::tempdir;

    use super::{PdfError, PdfInfo, inspect_pdf, parse_page_ranges, write_simple_pdf};

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
