# Arc03 Closing Report: Rust Best-Practices Audit And Fixes

Date: 2026-08-26
Arc: `arc03-rust-best-practices`
Final feature branch: `fix/cargo-reproducibility-policy`
Final feature commit: `d820065 Fix Cargo reproducibility policy`
Planning branch: `planning`

## Verdict

Arc03 is closed locally. The arc delivered its capability: an evidence-based
Rust audit, behavior baselines before production repairs, focused repair
slices, and final workflow-equivalent verification on the composed code state.

All eight slices in the arc have closing reports and CDC verification. All six
arc-ledger rows are closed done. No arc-scale silent drop was found.

## Slice Walk

| Slice | Outcome | Evidence |
|-------|---------|----------|
| slice01-diagnosis-only-audit | delivered | `slice01-diagnosis-only-audit/cdc-verification.md` verifies the read-only audit and workbench-only feature diff. |
| slice02-baseline-characterization-tests | delivered | `slice02-baseline-characterization-tests/cdc-verification.md` verifies baseline tests before production repairs. |
| slice03-triage-and-fix-map | delivered | `slice03-triage-and-fix-map/cdc-verification.md` verifies all eight audit findings were mapped to repair slices or deferrals. |
| slice04-cli-error-exit-codes | delivered | `slice04-cli-error-exit-codes/cdc-verification.md` verifies RUST-001 CLI failure exits. |
| slice05-structured-parser-transform-errors | delivered | `slice05-structured-parser-transform-errors/cdc-verification.md` verifies RUST-002/RUST-003 structured error repairs. |
| slice06-lsp-error-boundaries-and-metadata | delivered | `slice06-lsp-error-boundaries-and-metadata/cdc-verification.md` verifies RUST-005/RUST-008 LSP error-boundary and metadata repairs. |
| slice07-lsp-diagnostic-lock-scope | delivered with one test-evidence deferral | `slice07-lsp-diagnostic-lock-scope/cdc-verification.md` verifies RUST-004 lock-scope repair and records the C7-5 runtime-contention test re-entry condition. |
| slice08-cargo-reproducibility-policy | delivered | `slice08-cargo-reproducibility-policy/cdc-verification.md` verifies RUST-006 dependency and lockfile policy. |

Slice count: 8 specified, 8 closed, 0 dropped.

## Composition Check

Final feature diff from the Arc03 base contains the expected audit baseline and
repair surface:

```text
.gitignore
Cargo.lock
Cargo.toml
src/bin/hddl_analyzer/main.rs
src/language_server/request_handler.rs
src/lib.rs
src/transpiler/transformations/remove_equality.rs
tests/current_behavior.rs
tests/lsp_current_behavior.rs
```

Final Arc03 commits on top of the Rust 2024/CI baseline:

```text
b596714 Add baseline characterization tests
f3a3f8d Fix CLI error exit codes
6bd1b0a Fix structured parser transform errors
fbb27d7 Fix LSP error boundaries and metadata
e14078c Fix LSP diagnostic lock scope
d820065 Fix Cargo reproducibility policy
```

The final code state composes: the characterization tests introduced before
repairs remain green after the repairs, the strict Rust quality gate remains
green, the release binary builds, and the CLI help smoke test succeeds.

CDC reproduced the arc-scale gate at `d820065`:

- `cargo fmt --check`
- `cargo check --all-targets`
- `cargo check --locked --all-targets`
- `RUSTFLAGS="-D rust-2024-compatibility" cargo check --all-targets`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test --all-targets`
- `cargo build --release --bins`
- `./target/release/hddl_analyzer --help`
- `actionlint .github/workflows/ci.yml`
- `git diff --check d6de756..HEAD`

All commands exited `0`. `cargo test --all-targets` passed with the existing
ignored legacy tests unchanged:

- library tests: 111 passed, 1 ignored
- `current_behavior`: 10 passed
- `integration_flawed`: 21 passed, 2 ignored
- `integration_ipc`: 1 ignored
- `integration_json`: 8 passed, 1 ignored
- `lsp_current_behavior`: 6 passed

## Arc Ledger Walk

- A3-1: Done. Slice01 produced a diagnosis-only Rust audit with no source,
  test, manifest, CI, or README edits.
- A3-2: Done. Slice02 added current-behavior characterization tests before
  production repair work began.
- A3-3: Done. Slice03 mapped all eight Rust audit findings into focused repair
  slices or explicit deferrals.
- A3-4: Done. Every production repair slice opened by Slice03 has CDC
  verification. RUST-007 is the only audit finding deferred outside Arc03, and
  it is routed to Arc04 public API cohesion.
- A3-5: Done. The final Arc03 code state at `d820065` passes the full local
  workflow-equivalent gate, with an additional locked Cargo check after
  Slice08 introduced `Cargo.lock`.
- A3-6: Done. Bubble-up findings are routed: RUST-007 remains in Arc04, and
  Slice07 C7-5 has a concrete future re-entry condition for deterministic
  runtime-contention coverage.

Rows: 6. Done: 6. Deferred: 0. No-op: 0.

## Accumulated Plan Changes

Arc03 evolved from a placeholder into a full audit/fix arc as evidence arrived:

- Slice01 established the diagnosis-only audit and normalized audit severity
  naming.
- Slice02 added behavior baselines before production repairs.
- Slice03 replaced the placeholder focused-fixes shape with five repair slices
  and one Arc04 deferral.
- Slice04 through Slice08 each closed their assigned repair scope and updated
  the arc plan as evidence landed.

No additional remediation slice is needed before Arc03 closes.

## Bubble-Up To The Project

Arc03 delivered the project roadmap capability for Rust best-practices audit
and fixes. The remaining work is not an Arc03 gap:

- RUST-007 is intentionally deferred to Arc04 because it is a public API and
  cohesion decision, not a concrete Arc03 behavior/policy repair.
- Slice07 C7-5 remains a future test-evidence improvement with a specific
  re-entry condition: add deterministic runtime-contention coverage when a
  controlled test hook or in-crate direct handler harness exists.

Project-plan change required: mark Arc03 closed locally and make Arc04 the next
cleanup arc.

## What Worked

The read-only audit followed by baseline characterization made the production
repairs deliberate instead of speculative. Splitting LSP work into
error-boundary and lock-scope slices kept the evidence clean, and separating
Cargo policy into Slice08 avoided mixing generated dependency state with
runtime behavior changes.
