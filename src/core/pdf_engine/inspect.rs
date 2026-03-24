use std::{fs, path::Path};

use crate::core::{PdfError, PdfInfo};

pub fn inspect_pdf(path: &Path) -> Result<PdfInfo, PdfError> {
    let bytes = fs::read(path).map_err(|source| PdfError::OpenPdf {
        path: path.display().to_string(),
        source,
    })?;
    inspect_pdf_bytes(path, &bytes)
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

pub(super) fn extract_page_rotations(text: &str) -> Vec<Option<i32>> {
    text.split("endobj")
        .filter_map(|obj| {
            if !obj.contains("/Type /Page") || obj.contains("/Type /Pages") {
                return None;
            }
            Some(extract_rotation_value(obj))
        })
        .collect()
}

fn extract_rotation_value(page_obj: &str) -> Option<i32> {
    let token = "/Rotate ";
    let start = page_obj.find(token)? + token.len();
    let rest = &page_obj[start..];
    let value = rest
        .split_whitespace()
        .next()
        .and_then(|v| v.parse::<i32>().ok())?;
    Some(value)
}
