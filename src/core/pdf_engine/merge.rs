use std::{fs, path::Path};

use crate::core::PdfError;

use super::{
    inspect::{extract_page_rotations, inspect_pdf},
    links::{build_link_annotations, write_pdf_with_link_annotations},
    write::{
        write_pdf_with_dest_entries, write_pdf_with_index_entries, write_pdf_with_page_rotations,
    },
};

pub fn merge_pdfs(inputs: &[&Path], output: &Path) -> Result<(), PdfError> {
    merge_pdfs_with_index(inputs, output, false)
}

pub fn merge_pdfs_with_index(inputs: &[&Path], output: &Path, index: bool) -> Result<(), PdfError> {
    let plan = collect_merge_plan(inputs, index)?;
    let out = write_pdf_with_page_rotations(
        plan.page_total,
        &plan.output_version,
        &plan.merged_rotations,
    );
    let out = if plan.include_index {
        write_pdf_with_index_entries(out, &plan.index_entries)
    } else {
        out
    };
    let out = if plan.include_index {
        write_pdf_with_dest_entries(out, &plan.dest_entries)
    } else {
        out
    };
    let out = if plan.include_index {
        let annotations = build_link_annotations(&plan.dest_entries);
        write_pdf_with_link_annotations(out, &annotations)
    } else {
        out
    };
    fs::write(output, out).map_err(|source| PdfError::SavePdf {
        path: output.display().to_string(),
        source,
    })?;
    Ok(())
}

struct MergePlan {
    page_total: usize,
    output_version: String,
    merged_rotations: Vec<Option<i32>>,
    index_entries: Vec<(String, usize)>,
    dest_entries: Vec<(String, usize, String)>,
    include_index: bool,
}

fn collect_merge_plan(inputs: &[&Path], include_index: bool) -> Result<MergePlan, PdfError> {
    if inputs.len() < 2 {
        return Err(PdfError::MergeRequiresMultipleInputs);
    }

    let mut page_total = 0usize;
    let mut merged_rotations: Vec<Option<i32>> = Vec::new();
    let mut output_version = String::from("1.5");
    let mut index_entries: Vec<(String, usize)> = Vec::new();
    let mut dest_entries: Vec<(String, usize, String)> = Vec::new();
    let mut running_start = if include_index { 2usize } else { 1usize };
    let mut destination_counter = 1usize;
    for input in inputs {
        let info = inspect_pdf(input)?;
        let display_name = input_display_name(input);
        if include_index {
            index_entries.push((display_name.clone(), running_start));
            dest_entries.push((
                format!("dest-{destination_counter}"),
                running_start,
                display_name,
            ));
            destination_counter += 1;
        }
        page_total += info.page_count;
        running_start += info.page_count;
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

    if include_index {
        page_total += 1;
        merged_rotations.insert(0, None);
    }

    Ok(MergePlan {
        page_total,
        output_version,
        merged_rotations,
        index_entries,
        dest_entries,
        include_index,
    })
}

fn input_display_name(path: &Path) -> String {
    path.file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| path.display().to_string())
}
