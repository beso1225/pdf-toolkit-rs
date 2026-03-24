# pdf-toolkit-rs

A spec-driven Rust PDF toolkit implemented without PDF-specific crates.

[![CI](https://github.com/beso1225/pdf-toolkit-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/beso1225/pdf-toolkit-rs/actions/workflows/ci.yml)

## Current capabilities

- `info <input.pdf>`
- `merge <inputs...> -o <output.pdf>`
- `extract-pages <input.pdf> --pages <range> -o <output.pdf>`
- `remove-pages <input.pdf> --pages <range> -o <output.pdf>`
- `rotate-pages <input.pdf> --pages <range> --deg <90|180|270> -o <output.pdf>`
- `create blank --size <A4|Letter|WxH> -o <output.pdf>`
- `set-meta <input.pdf> --title ... --author ... -o <output.pdf>`
- `reorder-pages <input.pdf> --order <range syntax> -o <output.pdf>`
- `split <input.pdf> --by <single|range:<ranges>|chunk:<size>> --output-dir <dir>`

## Roadmap (next steps)

Near-term focus:

- strengthen split behavior with additional compatibility-style cases
- tighten error UX consistency across all commands
- add explicit stdout/stderr contract tests for every command

## Farther implementation roadmap

### Phase 1: Real PDF operation fidelity

- preserve existing page objects when possible (not only synthetic rewrite)
- improve merge fidelity for metadata/catalog structure
- improve rotate/reorder/extract to keep per-page attributes
- add encrypted-PDF behavior policy (`inspect` support + edit restrictions)

### Phase 2: CLI UX and safety

- add overwrite policy flags (`--force`, `--no-clobber`)
- add deterministic output naming templates for split
- unify error messages and exit-code policy across all commands
- add `--json` output mode for machine-friendly integration

### Phase 3: Quality and compatibility hardening

- expand public-inspired compatibility corpus with edge-case manifests
- add differential checks against well-known tools on selected fixtures
- add larger performance regression suite and budget thresholds
- add fuzz-style parser robustness tests for malformed inputs

### Phase 4: Distribution and release

- publish crate API docs and examples per command
- add release automation for tags/changelog/binaries
- support install paths (`cargo install`, prebuilt binaries)
- define semver and backward-compatibility policy

For every upcoming skill we enforce:

1. Structure inspection + refactor (if needed) before coding
2. Failing tests first (unit + CLI + compatibility manifests)
3. Minimal implementation to pass
4. Quality gates (`fmt`, `clippy`, `test`)
5. Feature-branch PR, self-review, merge

## Development flow

Each skill follows:

1. Inspect structure and refactor if needed
2. Add failing tests first (TDD)
3. Implement minimum code to pass
4. Run `cargo fmt`, `cargo clippy`, `cargo test`
5. Commit, open PR, self-review, merge

## Test results (latest local run)

Command:

```bash
cargo test -q
```

Summary:

- all test targets passed
- no failed tests

The suite includes:

- CLI smoke tests
- skill-specific tests (`merge`, `extract-pages`, `remove-pages`, `rotate-pages`, `create blank`, `set-meta`, `reorder-pages`, `split`, range engine)
- public-inspired compatibility tests (pdf.js / qpdf / pdfium style)
- golden CLI output checks
- performance smoke test

When adding new features, update this section with:

- latest `cargo test -q` result summary
- newly added test categories/cases
- any new public-inspired compatibility cases

## CI

GitHub Actions workflow: `.github/workflows/ci.yml`

On push/PR it runs:

- `cargo fmt --all -- --check`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test --all-targets --all-features -- --nocapture`

## Run locally

```bash
cargo run -- info path/to/file.pdf
cargo run -- merge a.pdf b.pdf -o merged.pdf
cargo run -- extract-pages input.pdf --pages 2,4-5 -o out.pdf
cargo run -- remove-pages input.pdf --pages 1,3 -o out.pdf
cargo run -- rotate-pages input.pdf --pages 2,4 --deg 90 -o out.pdf
cargo run -- create blank --size A4 -o blank.pdf
cargo run -- set-meta input.pdf --title \"Spec Driven\" --author \"Yutaro\" -o out.pdf
cargo run -- reorder-pages input.pdf --order 4,2,1,3 -o out.pdf
cargo run -- split input.pdf --by chunk:2 --output-dir parts
```
