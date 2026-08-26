# Slice05: Test Helper And Private Naming Cohesion

Version: 1.0
Date: 2026-08-25
Arc: `arc04-rust-cohesion-audit`
Expected branch: `fix/test-private-naming-cohesion`

## Goal

Clean up low-risk test helper drift and private/test-only naming drift after the
behavior-changing Arc04 repairs have their characterization baseline.

## Scope

In scope:

- Consolidate repeated integration-test assertion helpers where the helper keeps
  assertions at least as precise as the current tests.
- Preserve fixture coverage and ignored-test status unless a ledger row
  explicitly changes it.
- Rename private implementation modules with spelling drift, such as the
  quantifier-elimination module file, without changing public enum variants.
- Rename test functions with spelling drift when it does not affect product API.
- Keep public typo variants unchanged unless the operator explicitly approves a
  public API compatibility plan.

Out of scope:

- Public enum variant renames.
- Public error taxonomy redesign.
- Formula API changes.
- Parser byte-input changes or malformed-problem behavior changes.
- Cargo, workflow, README, or docs changes unless directly required by test
  compilation.

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

- Test assertion helper duplication is reduced without weakening checks.
- Private/test-only spelling drift is repaired.
- Public typo variants remain unchanged unless the operator has explicitly
  approved a compatibility plan.
- Full local gate passes.
