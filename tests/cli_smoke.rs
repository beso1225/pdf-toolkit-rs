use assert_cmd::Command;
use lopdf::{Document, Object, Stream, dictionary};
use predicates::str::contains;
use tempfile::tempdir;

fn write_minimal_pdf(path: &std::path::Path) {
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
fn runs_without_args() {
    Command::cargo_bin("pdf")
        .expect("binary should build")
        .assert()
        .success();
}

#[test]
fn accepts_info_subcommand_shape() {
    let dir = tempdir().expect("temp dir should be created");
    let file_path = dir.path().join("minimal.pdf");
    write_minimal_pdf(&file_path);

    Command::cargo_bin("pdf")
        .expect("binary should build")
        .args(["info", file_path.to_string_lossy().as_ref()])
        .assert()
        .success()
        .stdout(contains("version=1.5"))
        .stdout(contains("pages=1"))
        .stdout(contains("encrypted=false"));
}

#[test]
fn info_subcommand_fails_for_missing_file() {
    Command::cargo_bin("pdf")
        .expect("binary should build")
        .args(["info", "tests/fixtures/missing.pdf"])
        .assert()
        .failure()
        .stderr(contains("failed to open PDF"));
}

#[test]
fn help_mentions_spec_driven_toolkit() {
    Command::cargo_bin("pdf")
        .expect("binary should build")
        .arg("--help")
        .assert()
        .success()
        .stdout(contains("Spec-driven PDF toolkit in Rust"));
}
