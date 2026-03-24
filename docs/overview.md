# Overview

`pdf-toolkit-rs` is a Rust project that provides:

- a CLI binary (`pdf`)
- a reusable crate API (`pdf::core`)

The project uses a strict test-first workflow and currently focuses on deterministic, simple PDF operations.

## Implemented capabilities

- inspect PDF metadata/page count
- merge PDFs
- extract/remove/reorder page subsets
- rotate selected pages
- create blank PDF pages
- set basic metadata (`title`, `author`)
- split PDFs by single/range/chunk strategy

## Current technical direction

- no PDF-specific external crate dependency for core behavior
- incremental fidelity improvements via skill-based implementation
- compatibility and regression safety through layered test suites
- core implementation module renamed from `simple_pdf` to `pdf_engine` for clearer intent
