# Arc03 Slice02 Closing Report: Baseline Characterization Tests

Date: 2026-08-25
Feature branch: `audit/rust-best-practices`
Feature base: current local `main` as prepared for Arc03

## Summary

Slice02 added characterization coverage only. The feature worktree now has two new integration test files:

- `tests/current_behavior.rs`: CLI success/error baselines, public HDDL constructor mismatch panics, transpiler mismatch panics, and domain-only `remove-equality-constraints` panic.
- `tests/lsp_current_behavior.rs`: stdio JSON-RPC harness for reachable language-server behavior.

No production source, workflow, manifest, lockfile, or dependency-policy changes were made for this slice.

## Tests Added

`tests/current_behavior.rs` adds 10 current-behavior tests:

- Missing CLI input exits `0` and writes an error to stderr.
- Unsupported CLI extension exits `0` and writes an error to stderr.
- Semantic verification failure exits `0` and writes diagnostics to stderr.
- Output write failure exits `0` and writes an error to stderr.
- Known-good verification exits `0` and prints success.
- `HDDLProgram::from_hddl` panics for domain-as-problem and problem-as-domain mismatches.
- `Transpiler::from_hddl` panics for domain-as-problem and problem-as-domain mismatches.
- `RemoveEqualityConstraints` on a domain-only program panics.

`tests/lsp_current_behavior.rs` adds 3 current-behavior tests:

- `initialize` reports stale server version `0.1.0`, which differs from `CARGO_PKG_VERSION`.
- `textDocument/diagnostic` for an unsynced document returns JSON-RPC error `-32602`.
- A synced problem document without its domain returns a full diagnostic report with zero items.

## LSP Harness Limits

The slice characterized the LSP behavior that is reachable through a stable stdio harness without changing private internals. The following cases are deferred to repair slices:

- Deterministic `RwLock` contention coverage for diagnostics. Re-entry condition: the diagnostic path either stops holding the document read guard across awaited work or exposes a narrow test hook that can block awaited work under test control.
- Panic-oriented protocol paths such as non-file URI conversion, unreadable/missing sibling files, or `didSave` filesystem edge cases. Re-entry condition: the repair slice introduces behavior-preserving error boundaries or injectable filesystem/request helpers so these can be asserted without brittle process-death timing.

No post-fix LSP assertions were added in this baseline slice.

## Verification

All required gates passed:

- `cargo fmt --check`
- `cargo check --all-targets`
- `RUSTFLAGS="-D rust-2024-compatibility" cargo check --all-targets`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test --all-targets`
- `cargo build --release --bins`
- `./target/release/hddl_analyzer --help`
- `actionlint .github/workflows/ci.yml`
- `git diff --check`

Additional scope checks passed:

- `git diff --name-only main..HEAD` lists only `tests/current_behavior.rs` and `tests/lsp_current_behavior.rs` for Slice02 feature changes.
- `git diff -- Cargo.toml .gitignore Cargo.lock` produced no output.
- `git diff -- tests src Cargo.toml | rg "#\\[ignore\\]|ignore ="` produced no output and exited `1`, confirming no new ignored tests.

The four audit runtime probes still match the pre-fix baseline exit codes: `0`, `0`, `101`, `101`.

## Ledger Walk

- C3-1: Done. Feature diff is test-only.
- C3-2: Done. CLI success and recoverable error behavior is characterized.
- C3-3: Done. Public constructor and transpiler mismatch panics are characterized.
- C3-4: Done. Domain-only remove-equality behavior is characterized.
- C3-5: Done. LSP initialize stale metadata is characterized.
- C3-6: Done. Reachable diagnostic behavior is characterized; brittle panic paths are deferred with re-entry conditions.
- C3-7: Done. `RwLock` contention is explicitly deferred as a current-behavior limitation, not asserted as a post-fix property.
- C3-8: Done. Dependency and lockfile policy stayed out of scope.
- C3-9: Done. No ignored tests were introduced.
- C3-10: Done. Full local workflow-equivalent gate passed.
- C3-11: Done. Audit probes still show pre-fix behavior.
- C3-12: Done. This report records baselines and Slice03 handoff.

## Bubble-Up To Slice03

Slice03 should use these tests as the pre-fix safety net and split repairs narrowly:

- Fix RUST-001 CLI recoverable error exit codes first, then update the CLI current-behavior assertions from exit `0` to the desired non-zero behavior.
- Convert RUST-002 and RUST-003 panic paths into structured errors, then replace the `catch_unwind` baselines with normal error assertions.
- Repair LSP metadata and diagnostic error boundaries with deterministic tests added alongside the implementation.
- Keep RUST-006 dependency and lockfile policy separate from behavior repairs unless a repair explicitly requires a manifest change.
