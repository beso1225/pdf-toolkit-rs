use std::path::Path;

use lopdf::{Document, Object};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PdfInfo {
    pub version: String,
    pub page_count: usize,
    pub encrypted: bool,
    pub title: Option<String>,
    pub author: Option<String>,
}

#[derive(Debug, Error)]
pub enum PdfError {
    #[error("failed to open PDF at `{path}`: {source}")]
    OpenPdf { path: String, source: lopdf::Error },
}

pub fn inspect_pdf(path: &Path) -> Result<PdfInfo, PdfError> {
    let doc = Document::load(path).map_err(|source| PdfError::OpenPdf {
        path: path.display().to_string(),
        source,
    })?;

    let page_count = doc.get_pages().len();
    let version = doc.version.clone();
    let encrypted = doc.is_encrypted();
    let title = info_string(&doc, b"Title");
    let author = info_string(&doc, b"Author");

    Ok(PdfInfo {
        version,
        page_count,
        encrypted,
        title,
        author,
    })
}

fn info_string(doc: &Document, key: &[u8]) -> Option<String> {
    let info_ref = doc.trailer.get(b"Info").ok()?.as_reference().ok()?;
    let info_obj = doc.get_object(info_ref).ok()?;
    let dict = info_obj.as_dict().ok()?;
    let obj = dict.get(key).ok()?;
    object_to_string(obj)
}

fn object_to_string(obj: &Object) -> Option<String> {
    match obj {
        Object::String(bytes, _) => Some(String::from_utf8_lossy(bytes).to_string()),
        Object::Name(name) => Some(String::from_utf8_lossy(name).to_string()),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use lopdf::{Document, Object, Stream, dictionary};
    use tempfile::tempdir;

    use super::{PdfInfo, inspect_pdf};

    fn write_minimal_pdf(path: &Path) {
        let mut doc = Document::with_version("1.5");
        let pages_id = doc.new_object_id();
        let page_id = doc.new_object_id();
        let contents_id = doc.add_object(Stream::new(dictionary! {}, Vec::new()));

        let page = dictionary! {
            "Type" => "Page",
            "Parent" => pages_id,
            "MediaBox" => vec![0.into(), 0.into(), 200.into(), 200.into()],
            "Contents" => contents_id,
        };
        doc.objects.insert(page_id, Object::Dictionary(page));

        let pages = dictionary! {
            "Type" => "Pages",
            "Kids" => vec![page_id.into()],
            "Count" => 1,
        };
        doc.objects.insert(pages_id, Object::Dictionary(pages));

        let catalog_id = doc.add_object(dictionary! {
            "Type" => "Catalog",
            "Pages" => pages_id,
        });
        doc.trailer.set("Root", catalog_id);

        doc.save(path).expect("pdf fixture must be writable");
    }

    #[test]
    fn inspect_pdf_reads_minimal_fixture() {
        let dir = tempdir().expect("temp dir must be created");
        let file_path = dir.path().join("minimal.pdf");
        write_minimal_pdf(&file_path);

        let info: PdfInfo = inspect_pdf(&file_path).expect("fixture should parse");
        assert_eq!(info.version, "1.5");
        assert_eq!(info.page_count, 1);
        assert!(!info.encrypted);
        assert!(info.title.is_none());
        assert!(info.author.is_none());
    }

    #[test]
    fn inspect_pdf_reports_missing_file_path() {
        let err = inspect_pdf(Path::new("tests/fixtures/does-not-exist.pdf"))
            .expect_err("missing path must error");
        let rendered = err.to_string();
        assert!(rendered.contains("does-not-exist.pdf"));
    }
}
