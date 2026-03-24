# pdf-toolkit-rs

A spec-driven Rust PDF toolkit implemented without PDF-specific crates.

[![CI](https://github.com/beso1225/pdf-toolkit-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/beso1225/pdf-toolkit-rs/actions/workflows/ci.yml)

## Current capabilities

- `info <input.pdf>`
- `merge <inputs...> -o <output.pdf>`
- `extract-pages <input.pdf> --pages <range> -o <output.pdf>`
- `remove-pages <input.pdf> --pages <range> -o <output.pdf>`

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
- skill-specific tests (`merge`, `extract-pages`, `remove-pages`, range engine)
- public-inspired compatibility tests (pdf.js / qpdf / pdfium style)
- golden CLI output checks
- performance smoke test

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
```
