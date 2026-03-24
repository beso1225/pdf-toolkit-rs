pub(super) fn build_link_annotations(
    dest_entries: &[(String, usize, String)],
) -> Vec<(String, String, String)> {
    let mut out = Vec::with_capacity(dest_entries.len());
    for (idx, (dest_name, _, label)) in dest_entries.iter().enumerate() {
        out.push((
            format!("index-{}", idx + 1),
            dest_name.clone(),
            label.clone(),
        ));
    }
    out
}

pub(super) fn write_pdf_with_link_annotations(
    mut bytes: Vec<u8>,
    links: &[(String, String, String)],
) -> Vec<u8> {
    for (index_name, dest_name, label) in links {
        bytes.extend_from_slice(
            format!("\n/LinkAnnot ({index_name}|{dest_name}|{label})").as_bytes(),
        );
    }
    bytes
}
