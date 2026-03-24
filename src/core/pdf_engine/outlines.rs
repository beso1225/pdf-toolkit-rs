pub(super) fn build_outline_entries(
    dest_entries: &[(String, usize, String)],
) -> Vec<(String, String, String)> {
    let mut out = Vec::with_capacity(dest_entries.len());
    for (idx, (dest_name, _, label)) in dest_entries.iter().enumerate() {
        out.push((
            format!("outline-{}", idx + 1),
            dest_name.clone(),
            label.clone(),
        ));
    }
    out
}

pub(super) fn write_pdf_with_outline_entries(
    mut bytes: Vec<u8>,
    outlines: &[(String, String, String)],
) -> Vec<u8> {
    for (outline_name, dest_name, label) in outlines {
        bytes.extend_from_slice(
            format!("\n/OutlineEntry ({outline_name}|{dest_name}|{label})").as_bytes(),
        );
    }
    bytes
}
