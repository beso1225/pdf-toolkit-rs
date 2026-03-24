use std::collections::HashSet;

use super::error::PdfError;

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
