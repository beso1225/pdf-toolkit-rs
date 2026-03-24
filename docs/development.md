# Development

## Workflow

Each skill is delivered with this flow:

1. inspect/refactor structure if needed
2. write failing tests first
3. implement minimum code to pass
4. run quality gates
5. update documentation (`README.md` and relevant `docs/*.md`)
6. open PR, self-review, merge

## Quality gates

Run locally:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features -- --nocapture
```

## Test strategy

- CLI smoke tests
- skill-focused integration tests
- public-inspired compatibility tests
- golden output checks
- performance smoke checks

Detailed contract: `docs/test-spec.md`
