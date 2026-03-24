use std::{fs, path::Path};

use super::{error::PdfError, parse_page_ranges, types::PdfInfo};

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

pub fn extract_pages(input: &Path, pages: &str, output: &Path) -> Result<(), PdfError> {
    let info = inspect_pdf(input)?;
    let selected = parse_page_ranges(pages, info.page_count)?;
    let out = write_simple_pdf(selected.len(), &info.version);
    fs::write(output, out).map_err(|source| PdfError::SavePdf {
        path: output.display().to_string(),
        source,
    })?;
    Ok(())
}

pub fn remove_pages(input: &Path, pages: &str, output: &Path) -> Result<(), PdfError> {
    let info = inspect_pdf(input)?;
    let selected = parse_page_ranges(pages, info.page_count)?;
    if selected.len() >= info.page_count {
        return Err(PdfError::RemoveAllPagesForbidden);
    }

    let remaining = info.page_count - selected.len();
    let out = write_simple_pdf(remaining, &info.version);
    fs::write(output, out).map_err(|source| PdfError::SavePdf {
        path: output.display().to_string(),
        source,
    })?;
    Ok(())
}

pub fn rotate_pages(
    input: &Path,
    pages: &str,
    degrees: i32,
    output: &Path,
) -> Result<(), PdfError> {
    if !matches!(degrees, 90 | 180 | 270) {
        return Err(PdfError::InvalidRotationDegrees { degrees });
    }

    let info = inspect_pdf(input)?;
    let selected = parse_page_ranges(pages, info.page_count)?;
    let out = write_rotated_simple_pdf(info.page_count, &info.version, &selected, degrees);
    fs::write(output, out).map_err(|source| PdfError::SavePdf {
        path: output.display().to_string(),
        source,
    })?;
    Ok(())
}

pub fn create_blank(size: &str, output: &Path) -> Result<(), PdfError> {
    let (width, height) = parse_blank_size(size)?;
    let out = write_single_page_pdf_with_size("1.5", width, height);
    fs::write(output, out).map_err(|source| PdfError::SavePdf {
        path: output.display().to_string(),
        source,
    })?;
    Ok(())
}

pub fn write_simple_pdf(page_count: usize, version: &str) -> Vec<u8> {
    write_rotated_simple_pdf(page_count, version, &[], 0)
}

fn write_rotated_simple_pdf(
    page_count: usize,
    version: &str,
    rotated_pages: &[usize],
    degrees: i32,
) -> Vec<u8> {
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
        let page_num = i + 1;
        let page_id = 3 + i;
        let rotate = if rotated_pages.contains(&page_num) {
            format!(" /Rotate {degrees}")
        } else {
            String::new()
        };
        objects.push(format!(
            "{page_id} 0 obj\n<< /Type /Page /Parent 2 0 R /MediaBox [0 0 200 200]{rotate} >>\nendobj\n"
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

fn parse_blank_size(size: &str) -> Result<(i32, i32), PdfError> {
    let normalized = size.trim();
    if normalized.eq_ignore_ascii_case("A4") {
        return Ok((595, 842));
    }
    if normalized.eq_ignore_ascii_case("Letter") {
        return Ok((612, 792));
    }

    let Some((w, h)) = normalized.split_once('x') else {
        return Err(PdfError::InvalidBlankSize {
            size: size.to_string(),
        });
    };
    let width = w.parse::<i32>().ok();
    let height = h.parse::<i32>().ok();
    match (width, height) {
        (Some(w), Some(h)) if w > 0 && h > 0 => Ok((w, h)),
        _ => Err(PdfError::InvalidBlankSize {
            size: size.to_string(),
        }),
    }
}

fn write_single_page_pdf_with_size(version: &str, width: i32, height: i32) -> Vec<u8> {
    let objects = vec![
        "1 0 obj\n<< /Type /Catalog /Pages 2 0 R >>\nendobj\n".to_string(),
        "2 0 obj\n<< /Type /Pages /Kids [3 0 R] /Count 1 >>\nendobj\n".to_string(),
        format!(
            "3 0 obj\n<< /Type /Page /Parent 2 0 R /MediaBox [0 0 {} {}] >>\nendobj\n",
            width, height
        ),
    ];

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
