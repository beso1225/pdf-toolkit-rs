# Usage

## Build

```bash
cargo build
```

## Common commands

```bash
cargo run -- info input.pdf
cargo run -- merge a.pdf b.pdf -o merged.pdf
cargo run -- extract-pages input.pdf --pages 2,4-5 -o out.pdf
cargo run -- remove-pages input.pdf --pages 1,3 -o out.pdf
cargo run -- rotate-pages input.pdf --pages 2,4 --deg 90 -o out.pdf
cargo run -- create blank --size A4 -o blank.pdf
cargo run -- set-meta input.pdf --title "Spec Driven" --author "Yutaro" -o out.pdf
cargo run -- reorder-pages input.pdf --order 4,2,1,3 -o out.pdf
cargo run -- split input.pdf --by chunk:2 --output-dir parts
```

## Notes

- Page ranges use 1-based indexing.
- `split` supports:
  - `single`
  - `range:<ranges>` (example: `range:1-2,4-5`)
  - `chunk:<size>` (example: `chunk:3`)
