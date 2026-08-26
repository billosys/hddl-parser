# Slice04: Parser API And Error Boundary

Version: 1.0
Date: 2026-08-25
Arc: `arc04-rust-cohesion-audit`
Expected branch: `fix/parser-api-error-boundary`

## Goal

Repair the non-breaking parser API and recoverable parser error-boundary parts
of the Arc04 cohesion findings after Slice03 baselines are in place.

## Scope

In scope:

- Change parser/transpiler/LSP byte-input APIs that only borrow data from
  `&Vec<u8>` to `&[u8]`.
- Add or update tests proving `&[u8]` callers work.
- Preserve existing Vec-backed callers through deref coercion.
- Replace the remaining malformed problem-parser panic path with a structured
  `ParsingError::Syntactic` result.
- Add a small parser-local helper for repeated syntactic error construction if
  it reduces duplication without widening the slice.
- Update Slice03 characterization expectations that intentionally change.
- Stop and ask for operator GO before narrowing crate-root public exports.

Out of scope:

- Typed transformation error redesign.
- Public enum spelling fixes.
- Public formula API changes.
- Test helper consolidation beyond what is necessary for this repair.
- Cargo, workflow, README, or docs changes unless directly required by tests.

## Verification Approach

Run the full local gate:

```bash
cargo fmt --check
cargo check --locked --all-targets
RUSTFLAGS="-D rust-2024-compatibility" cargo check --locked --all-targets
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked --all-targets
cargo build --locked --release --bins
./target/release/hddl_analyzer --help
actionlint .github/workflows/ci.yml
git diff --check
```

## Exit Criteria

- Borrowed byte inputs accept `&[u8]` where no Vec-specific behavior is used.
- Existing Vec-backed callers still compile and tests pass.
- Malformed problem parser behavior returns a structured syntactic error instead
  of panicking.
- No public export narrowing lands without operator GO.
- Full local gate passes.
