# Test Specification

This document defines the testing contract for `pdf-toolkit-rs`.

## Goals

- verify command behavior from user-facing CLI
- protect core PDF operations from regressions
- enforce skill-by-skill TDD delivery
- keep compatibility expectations explicit

## Required test layers

1. CLI smoke tests
   - binary launches
   - help output shape
   - basic command invocation success/failure

2. Skill-focused integration tests
   - one test file per skill area (merge/extract/remove/rotate/create/set-meta/reorder/split)
   - each new skill starts with failing tests first
   - success-path + representative failure-path required

3. Compatibility-style tests
   - manifest-based cases inspired by public tool ecosystems
   - behavior checks must be deterministic

4. Golden output tests
   - stable key output tokens for successful CLI commands
   - stable error-prefix checks for failure output

5. Performance smoke tests
   - non-exhaustive guardrails for larger synthetic inputs
   - ensures no severe regressions in common operations

## Command-specific minimum assertions

- `info`: version/pages/encrypted fields are present and correct
- `merge`: expected output page count; missing input fails
- `extract-pages`: selected page count correctness; out-of-range fails
- `remove-pages`: remaining page count correctness; remove-all forbidden
- `rotate-pages`: valid degrees succeed; invalid degrees fail
- `create blank`: supported sizes succeed; invalid size fails
- `set-meta`: title/author write behavior; at least one field required
- `reorder-pages`: output count/order contract; out-of-range fails
- `split`: single/range/chunk modes; invalid mode fails
- merge navigation markers: index/destination/link/outline labels are deterministic and safely encoded

## Skill workflow requirement (mandatory)

For each new skill:

1. add failing tests first
2. implement minimal code to pass
3. run quality gates
4. update docs (`README.md` + affected files under `docs/`)
5. merge through PR with self-review note

## Quality gate commands

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features -- --nocapture
```

## Exit criteria for a skill

A skill is complete only when:

- all newly added tests pass
- full repository test suite is green
- documentation is updated for behavior changes
