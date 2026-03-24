use serde::Deserialize;
use std::{fs, path::Path};

#[derive(Debug, Deserialize)]
struct CaseManifest {
    source: String,
    case_id: String,
    description: String,
}

#[test]
fn public_case_manifests_are_well_formed() {
    let dir = Path::new("tests/public_cases");
    let entries = fs::read_dir(dir).expect("public case directory must exist");

    let mut count = 0usize;
    for entry in entries {
        let entry = entry.expect("entry must read");
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("json") {
            continue;
        }

        let raw = fs::read_to_string(&path).expect("manifest should be readable");
        let manifest: CaseManifest =
            serde_json::from_str(&raw).expect("manifest must be valid JSON");

        assert!(!manifest.source.trim().is_empty(), "source must be set");
        assert!(!manifest.case_id.trim().is_empty(), "case_id must be set");
        assert!(
            !manifest.description.trim().is_empty(),
            "description must be set"
        );

        count += 1;
    }

    assert!(
        count >= 2,
        "at least two public-inspired cases are required"
    );
}
