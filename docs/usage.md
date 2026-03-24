# Usage

## Build

```bash
cargo build
```

## Interactive shell mode

Run without subcommands:

```bash
cargo run --
```

Inside the shell:

```text
pdf> help
pdf> info input.pdf
pdf> merge a.pdf b.pdf -o merged.pdf
pdf> quit
```

## Common commands

```bash
cargo run -- info input.pdf
cargo run -- info input.pdf --format json
cargo run -- merge a.pdf b.pdf -o merged.pdf
cargo run -- merge a.pdf b.pdf -o merged.pdf --format json
cargo run -- merge a.pdf b.pdf --index -o merged-index.pdf
cargo run -- merge a.pdf b.pdf --index --links=false --outlines=false -o merged-index-basic.pdf
cargo run -- extract-pages input.pdf --pages 2,4-5 -o out.pdf
cargo run -- extract-pages input.pdf --pages 2,4-5 -o out.pdf --format json
cargo run -- remove-pages input.pdf --pages 1,3 -o out.pdf
cargo run -- rotate-pages input.pdf --pages 2,4 --deg 90 -o out.pdf
cargo run -- create blank --size A4 -o blank.pdf --format json
cargo run -- set-meta input.pdf --title "Spec Driven" --author "Yutaro" -o out.pdf
cargo run -- reorder-pages input.pdf --order 4,2,1,3 -o out.pdf
cargo run -- split input.pdf --by chunk:2 --output-dir parts
cargo run -- split input.pdf --by chunk:2 --output-dir parts --format json
```

## Notes

- Page ranges use 1-based indexing.
- Error output is standardized as `error[<code>]: <message>` for stable scripting/diagnostics.
- Successful command output begins with:
  - `status=ok`
  - `command=<command-name>`
- `--format text|json` (default: `text`) is supported by `info`, `merge`, `extract-pages`, `remove-pages`, `rotate-pages`, `create blank`, `set-meta`, `reorder-pages`, and `split`.
- Running `pdf` without a subcommand opens the interactive shell.
- In shell mode, you can type normal commands directly (for example `info input.pdf`) and use `help`/`quit`.
- `split` supports:
  - `single`
  - `range:<ranges>` (example: `range:1-2,4-5`)
  - `chunk:<size>` (example: `chunk:3`)
- `merge --index` enables index/destination markers. Optional flags:
  - `--links=<true|false>` (default `true` when `--index` is enabled)
  - `--outlines=<true|false>` (default `true` when `--index` is enabled)
