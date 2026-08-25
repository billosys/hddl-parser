# Arc03: Rust Best-Practices Audit And Fixes

Version: 1.4
Date: 2026-08-25
Expected branches: `audit/rust-best-practices`, then smaller `test/...` and
`fix/...` branches as the audit requires.

## Capability

Arc03 performs a full evidence-based Rust audit, records current behavior with
targeted characterization tests before production changes, then contributes the
resulting Rust best-practice fixes upstream as focused reviewable PRs.

This arc is not the final cohesion pass. Arc03 handles concrete
best-practice, correctness, maintainability, test, API, CLI, error-handling,
and runtime-safety findings. Arc04 later checks whether the whole codebase
feels intentionally consistent after those repairs land.

## Slice Breakdown

| Slice | Scope | Depends On | Notes |
|-------|-------|------------|-------|
| slice01-diagnosis-only-audit | Read-only Rust audit using the collaboration-framework code-audit discipline and rust-guidelines substrate. | Arc01/Arc02 local quality baseline. | CDC-verified. Created `workbench/2026.08.25-audit-index.md` and `workbench/2026.08.25-audit-results-rust.md`; no Rust source, tests, manifests, CI, or README edits. |
| slice02-baseline-characterization-tests | Add missing unit/integration tests that capture current behavior in areas the audit expects later fixes to touch. | Slice01 findings. | CDC-verified. Added `tests/current_behavior.rs` and `tests/lsp_current_behavior.rs`; no production behavior or policy changes. LSP contention and brittle panic-path coverage have explicit re-entry conditions. |
| slice03-triage-and-fix-map | Convert the audit findings and baseline-test evidence into focused fix slices and upstream PR grouping. | Slice01 and Slice02. | Planning-only slice; creates `fix-map.md`, updates this arc plan, and opens concrete repair-slice open sets where the scope is ready. |
| slice04-cli-error-exit-codes | Fix RUST-001 so recoverable `hddl_analyzer` CLI failures exit non-zero while success remains `0`. | Slice03. | PR group 1: CLI error exits. Updates the CLI baselines in `tests/current_behavior.rs`. |
| slice05-structured-parser-transform-errors | Fix RUST-002 and RUST-003 by replacing recoverable parser/transpiler/transform panics with structured errors. | Slice04 preferred. | PR group 2: structured parser and transform errors. Updates the panic-capture baselines in `tests/current_behavior.rs`. |
| slice06-lsp-error-boundaries-and-metadata | Fix RUST-005 and RUST-008 by making ordinary LSP request/runtime failures non-panicking and using package metadata for initialize version. | Slice03; may run after Slice05 for review order. | PR group 3a: LSP error boundaries and metadata. Updates and extends `tests/lsp_current_behavior.rs`. |
| slice07-lsp-diagnostic-lock-scope | Fix RUST-004 by dropping the diagnostic document-map read guard before awaited work. | Slice06 preferred if shared LSP helpers emerge. | PR group 3b: LSP diagnostic lock scope. Kept separate because deterministic contention proof differs from protocol error-boundary testing. |
| slice08-cargo-reproducibility-policy | Fix RUST-006 by replacing wildcard dependency requirements and settling `Cargo.lock` tracking policy. | Behavior repair slices preferred first. | PR group 4: Cargo reproducibility policy. No behavior baseline required. |

## Dependencies

Arc03 consumes:

- Arc01's CI quality gates, including fmt, check, strict Clippy, tests, release
  build, binary smoke, and actionlint.
- Arc02's Rust 2024 target edition and strict `rust-2024-compatibility` gate.
- The known Arc02 follow-up candidate around possible async `RwLock` read
  guards held across awaits in language-server diagnostics.

Arc03 leaves for Arc04:

- Cohesion-only issues where the code is locally correct but project idioms are
  inconsistent.
- Any style unification work that should wait until concrete best-practice
  fixes have landed.
- RUST-007 public API cohesion: byte-slice API ergonomics and crate-root
  re-export design should be handled after Arc03 behavior and policy repairs
  settle.

## Arc Ledger

Capability: evidence-based Rust best-practice audit and repairs, with behavior
baselined before production code changes.

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
|----|-----------|--------|--------------|--------|--------|----------|-------|
| A3-1 | Slice01 produces a diagnosis-only audit with no source, test, manifest, CI, or README edits. | `test -f 01-cleanup/arc03-rust-best-practices/slice01-diagnosis-only-audit/cdc-verification.md` and inspect the no-source-edit row. | serious | arc-plan | done | `slice01-diagnosis-only-audit/cdc-verification.md` verifies 12/12 rows, full quality gate reproduction, runtime probes, and workbench-only implementation diff. | Read-only means audit reports plus planning close artifacts only. |
| A3-2 | Slice02 records current behavior with missing characterization tests before any production repair slice begins. | `test -f 01-cleanup/arc03-rust-best-practices/slice02-baseline-characterization-tests/cdc-verification.md` and inspect that the diff is test-only. | serious | operator-question | done | `slice02-baseline-characterization-tests/cdc-verification.md` verifies 12/12 rows, 13 new characterization tests, test-only `main..HEAD` diff, full quality gate reproduction, and runtime probes `0`, `0`, `101`, `101`. | This protects behavior before repairs. |
| A3-3 | Slice03 maps audit findings to focused fix slices or explicit deferrals. | `test -f 01-cleanup/arc03-rust-best-practices/slice03-triage-and-fix-map/cdc-verification.md` and inspect fix-map rows. | correctness | arc-plan | open | | Slice03 opens Slice04-Slice08 and defers RUST-007 to Arc04; CDC verification still required before this row closes. |
| A3-4 | Every production repair slice opened by Slice03 closes with CDC verification. | `find 01-cleanup/arc03-rust-best-practices -path "*/cdc-verification.md" -print` and compare against the Slice03 fix map. | correctness | ledger-discipline | open | | Applies to Slice04-Slice08 before Arc03 closes. |
| A3-5 | The final Arc03 code state passes the full local workflow-equivalent gate. | `cargo fmt --check`, `cargo check --all-targets`, `RUSTFLAGS="-D rust-2024-compatibility" cargo check --all-targets`, `cargo clippy --all-targets -- -D warnings`, `cargo test --all-targets`, `cargo build --release --bins`, `./target/release/hddl_analyzer --help`, `actionlint .github/workflows/ci.yml`, and `git diff --check`. | serious | arc-plan | open | | Run at arc close on the final repair branch state. |
| A3-6 | Arc03 bubble-up findings are routed into Arc04 or project-level deferrals before Arc03 closes. | `rg -n "Arc04|cohesion|deferred|bubble-up" 01-cleanup/arc03-rust-best-practices/closing-report.md 01-cleanup/project-plan.md` | correctness | project-plan | open | | Prevents audit findings from silently disappearing. |

## Version History

### v1.4 - 2026-08-25

Slice03 fix mapping opened five focused Arc03 repair slices: CLI exit codes,
structured parser/transform errors, LSP error boundaries and metadata, LSP
diagnostic lock scope, and Cargo reproducibility policy. RUST-007 is deferred
to Arc04 as public API cohesion work with explicit re-entry conditions.

### v1.3 - 2026-08-25

Slice03 opened as a planning-only triage and fix-map slice. It must account
for all eight Rust audit findings, map Slice02 baselines to repair slices, and
replace or refine the placeholder focused-fixes slice before production
repairs begin.

### v1.2 - 2026-08-25

Slice02 CDC verification landed. The baseline now covers CLI current exit
behavior, public API/transpiler mismatch panics, domain-only
remove-equality panic behavior, and reachable LSP behavior before production
repair slices begin.

### v1.1 - 2026-08-25

Slice01 CDC verification landed and the audit index top severity label was
normalized to `Blocker`. Slice02 opened as a test-only characterization
baseline before any production repair slices begin.

### v1.0 - 2026-08-25

Promoted Arc03 from placeholder to active planning. The first slice is
explicitly diagnosis-only/read-only, and Slice02 is reserved for
characterization tests that record current behavior before production repair
work begins.

### v0.1 - 2026-08-25

Placeholder opened to preserve the expected third PR family while keeping the
audit/fix breakdown out of scope until CI and edition work settle.
