pub fn write_simple_pdf(page_count: usize, version: &str) -> Vec<u8> {
    write_simple_pdf_with_metadata(page_count, version, None, None)
}

pub(super) fn write_simple_pdf_with_metadata(
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

pub(super) fn write_rotated_simple_pdf(
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

pub(super) fn write_pdf_with_page_rotations(
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

pub(super) fn write_single_page_pdf_with_size(version: &str, width: i32, height: i32) -> Vec<u8> {
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

pub(super) fn write_pdf_with_index_entries(
    mut bytes: Vec<u8>,
    entries: &[(String, usize)],
) -> Vec<u8> {
    for (name, start) in entries {
        bytes.extend_from_slice(format!("\n/IndexEntry ({name}|{start})").as_bytes());
    }
    bytes
}

pub(super) fn write_pdf_with_dest_entries(
    mut bytes: Vec<u8>,
    entries: &[(String, usize, String)],
) -> Vec<u8> {
    for (name, page, label) in entries {
        bytes.extend_from_slice(format!("\n/DestEntry ({name}|{page}|{label})").as_bytes());
    }
    bytes
}
