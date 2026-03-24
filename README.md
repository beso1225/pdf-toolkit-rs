# pdf-toolkit-rs

Simple Rust PDF CLI/crate with spec-driven, test-first development.

[![CI](https://github.com/beso1225/pdf-toolkit-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/beso1225/pdf-toolkit-rs/actions/workflows/ci.yml)

## Quick start

```bash
cargo run -- info path/to/file.pdf
cargo run -- merge a.pdf b.pdf -o merged.pdf
cargo run -- split input.pdf --by chunk:2 --output-dir parts
cargo run --   # interactive shell mode
```

## Commands

- `info <input.pdf>`
- `merge <inputs...> -o <output.pdf>`
- `merge <inputs...> [--index] [--links=<true|false>] [--outlines=<true|false>] -o <output.pdf>`
- `extract-pages <input.pdf> --pages <range> -o <output.pdf>`
- `remove-pages <input.pdf> --pages <range> -o <output.pdf>`
- `rotate-pages <input.pdf> --pages <range> --deg <90|180|270> -o <output.pdf>`
- `create blank --size <A4|Letter|WxH> -o <output.pdf>`
- `set-meta <input.pdf> --title ... --author ... -o <output.pdf>`
- `reorder-pages <input.pdf> --order <range syntax> -o <output.pdf>`
- `split <input.pdf> --by <single|range:<ranges>|chunk:<size>> --output-dir <dir>`

## Docs

- Project overview: `docs/overview.md`
- Usage and examples: `docs/usage.md`
- Development flow and quality gates: `docs/development.md`
- Test specification: `docs/test-spec.md`
- Roadmap and upcoming skills: `docs/roadmap.md`
