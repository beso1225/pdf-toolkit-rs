use std::{fs, path::Path};

use crate::core::{PdfError, parse_page_ranges};

use super::{
    inspect::inspect_pdf,
    write::{
        write_rotated_simple_pdf, write_simple_pdf, write_simple_pdf_with_metadata,
        write_single_page_pdf_with_size,
    },
};

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
