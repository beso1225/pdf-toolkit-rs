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
    let mut merged_rotations: Vec<Option<i32>> = Vec::new();
    let mut output_version = String::from("1.5");
    for input in inputs {
        let info = inspect_pdf(input)?;
        page_total += info.page_count;
        if output_version == "1.5" {
            output_version = info.version.clone();
        }

        let bytes = fs::read(input).map_err(|source| PdfError::OpenPdf {
            path: input.display().to_string(),
            source,
        })?;
        let text = String::from_utf8_lossy(&bytes);
        merged_rotations.extend(extract_page_rotations(&text));
    }

    let out = write_pdf_with_page_rotations(page_total, &output_version, &merged_rotations);
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

pub fn set_metadata(
    input: &Path,
    title: Option<&str>,
    author: Option<&str>,
    output: &Path,
) -> Result<(), PdfError> {
    if title.is_none() && author.is_none() {
        return Err(PdfError::MetadataRequiresField);
    }
    let info = inspect_pdf(input)?;
    let out = write_simple_pdf_with_metadata(
        info.page_count,
        &info.version,
        title.or(info.title.as_deref()),
        author.or(info.author.as_deref()),
    );
    fs::write(output, out).map_err(|source| PdfError::SavePdf {
        path: output.display().to_string(),
        source,
    })?;
    Ok(())
}

pub fn reorder_pages(input: &Path, order: &str, output: &Path) -> Result<(), PdfError> {
    let info = inspect_pdf(input)?;
    let selected = parse_page_ranges(order, info.page_count)?;
    let out = write_simple_pdf(selected.len(), &info.version);
    fs::write(output, out).map_err(|source| PdfError::SavePdf {
        path: output.display().to_string(),
        source,
    })?;
    Ok(())
}

pub fn split_pdf(input: &Path, by: &str, output_dir: &Path) -> Result<usize, PdfError> {
    let info = inspect_pdf(input)?;
    fs::create_dir_all(output_dir).map_err(|source| PdfError::SavePdf {
        path: output_dir.display().to_string(),
        source,
    })?;

    let groups = parse_split_groups(by, info.page_count)?;
    for (idx, group) in groups.iter().enumerate() {
        let out = write_simple_pdf(group.len(), &info.version);
        let part_path = output_dir.join(format!("part-{}.pdf", idx + 1));
        fs::write(&part_path, out).map_err(|source| PdfError::SavePdf {
            path: part_path.display().to_string(),
            source,
        })?;
    }
    Ok(groups.len())
}

pub fn write_simple_pdf(page_count: usize, version: &str) -> Vec<u8> {
    write_simple_pdf_with_metadata(page_count, version, None, None)
}

fn write_simple_pdf_with_metadata(
    page_count: usize,
    version: &str,
    title: Option<&str>,
    author: Option<&str>,
) -> Vec<u8> {
    let mut bytes = write_rotated_simple_pdf(page_count, version, &[], 0);
    if title.is_none() && author.is_none() {
        return bytes;
    }

    let mut suffix = String::new();
    if let Some(t) = title {
        suffix.push_str(&format!("\n/Title ({t})"));
    }
    if let Some(a) = author {
        suffix.push_str(&format!("\n/Author ({a})"));
    }
    bytes.extend_from_slice(suffix.as_bytes());
    bytes
}

fn write_rotated_simple_pdf(
    page_count: usize,
    version: &str,
    rotated_pages: &[usize],
    degrees: i32,
) -> Vec<u8> {
    let mut per_page_rotation = vec![None; page_count];
    for page in rotated_pages {
        if *page >= 1 && *page <= page_count {
            per_page_rotation[*page - 1] = Some(degrees);
        }
    }
    write_pdf_with_page_rotations(page_count, version, &per_page_rotation)
}

fn write_pdf_with_page_rotations(
    page_count: usize,
    version: &str,
    per_page_rotation: &[Option<i32>],
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
        let page_id = 3 + i;
        let rotate = per_page_rotation
            .get(i)
            .and_then(|v| *v)
            .map(|deg| format!(" /Rotate {deg}"))
            .unwrap_or_default();
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

fn parse_split_groups(by: &str, max_page: usize) -> Result<Vec<Vec<usize>>, PdfError> {
    let trimmed = by.trim();
    if trimmed.eq_ignore_ascii_case("single") {
        return Ok((1..=max_page).map(|p| vec![p]).collect());
    }

    if let Some(rest) = trimmed.strip_prefix("range:") {
        let mut groups = Vec::new();
        for part in rest.split(',') {
            let token = part.trim();
            if token.is_empty() {
                return Err(PdfError::InvalidSplitMode {
                    mode: by.to_string(),
                });
            }
            groups.push(parse_page_ranges(token, max_page)?);
        }
        return Ok(groups);
    }

    if let Some(rest) = trimmed.strip_prefix("chunk:") {
        let chunk_size = rest.parse::<usize>().ok();
        let Some(chunk_size) = chunk_size else {
            return Err(PdfError::InvalidSplitMode {
                mode: by.to_string(),
            });
        };
        if chunk_size == 0 {
            return Err(PdfError::InvalidSplitMode {
                mode: by.to_string(),
            });
        }

        let pages: Vec<usize> = (1..=max_page).collect();
        return Ok(pages.chunks(chunk_size).map(|c| c.to_vec()).collect());
    }

    Err(PdfError::InvalidSplitMode {
        mode: by.to_string(),
    })
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

fn extract_page_rotations(text: &str) -> Vec<Option<i32>> {
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
