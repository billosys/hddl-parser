# Slice02: Baseline Characterization Tests

Version: 1.0
Date: 2026-08-25
Arc: `arc03-rust-best-practices`
Expected branch: continue `audit/rust-best-practices` or use
`test/baseline-characterization`

## Goal

Add focused tests that record the current behavior behind the Arc03 audit
findings before any production fixes begin. The tests are a safety net for the
repair slices: they should make today's behavior visible, make desired
behavior changes intentional later, and avoid silently blessing known-bad
behavior as the final contract.

## Scope

In scope:

- Test-only additions under `tests/`, test fixtures under `tests/`, and
  `#[cfg(test)]` unit-test modules where internals cannot be reached through
  integration tests.
- Minimal `Cargo.toml` test-target or dev-dependency changes only if a test
  cannot be written with the existing dependency set; any such change must be
  justified in the ledger and closing report.
- CLI characterization for current process behavior:
  - missing input exits `0` and writes an error to stderr;
  - unsupported extension exits `0` and writes an error to stderr;
  - semantic/parse failure exits `0` and writes an error to stderr;
  - output write failure exits `0` and writes an error to stderr;
  - known-good verification exits `0` and prints success.
- Public parser/transpiler characterization for current domain/problem kind
  mismatch behavior, using `catch_unwind` or an equivalent panic-capturing
  shape so the tests pass today while clearly marking the behavior as
  current/undesired.
- Transform characterization for current domain-only
  `remove-equality-constraints` behavior, also panic-captured and clearly
  marked as current/undesired.
- LSP characterization where it can be added without production hooks:
  initialize metadata, ordinary failure paths, and the diagnostic/document-lock
  contention candidate.

Out of scope:

- Production Rust behavior fixes.
- CLI exit-code changes.
- Replacing panics with structured errors.
- Dependency-version or lockfile policy changes from RUST-006.
- API widening from `&Vec<u8>` to `&[u8]`.
- Making private internals public only for tests.
- New ignored tests that hide incomplete characterization.

## Verification Approach

The slice should prove that test coverage increased without changing
production behavior. CDC should inspect the diff, run the new focused tests,
rerun the full quality gate, and repeat the audit runtime probes to confirm
that repair behavior has not slipped into the baseline slice.

Expected verification commands:

```bash
cargo fmt --check
cargo check --all-targets
RUSTFLAGS="-D rust-2024-compatibility" cargo check --all-targets
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
cargo build --release --bins
./target/release/hddl_analyzer --help
actionlint .github/workflows/ci.yml
git diff --check
git diff --name-only
git diff -- tests src Cargo.toml | rg "#\\[ignore\\]|ignore ="
```

The final grep is expected to return no matches for newly added ignores. If it
matches pre-existing ignored tests, the closing report must distinguish them
from this slice's diff.

## Exit Criteria

- New characterization tests exist and pass.
- Tests are named or commented so known-bad current behavior is visibly
  temporary baseline behavior for later repair slices.
- No production behavior changes are made.
- No new ignored tests are added.
- Any LSP characterization that cannot be implemented without production hooks
  is explicitly deferred with a concrete re-entry condition for a later fix
  slice.
- The full local workflow-equivalent gate passes.
