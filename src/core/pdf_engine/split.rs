use std::{fs, path::Path};

use crate::core::{PdfError, parse_page_ranges};

use super::{inspect::inspect_pdf, write::write_simple_pdf};

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

fn parse_split_groups(by: &str, max_page: usize) -> Result<Vec<Vec<usize>>, PdfError> {
    let trimmed = by.trim();
    let lower = trimmed.to_ascii_lowercase();
    if lower == "single" {
        return Ok((1..=max_page).map(|p| vec![p]).collect());
    }

    if let Some(rest) = lower.strip_prefix("range:") {
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

    if let Some(rest) = lower.strip_prefix("chunk:") {
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
