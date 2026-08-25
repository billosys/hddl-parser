# Arc03: Rust Best-Practices Audit And Fixes

Version: 1.2
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
| slice03-triage-and-fix-map | Convert the audit findings and baseline-test evidence into focused fix slices and upstream PR grouping. | Slice01 and Slice02. | Decides which findings are in Arc03, which are deferred, and which cohesion-only concerns move to Arc04. |
| slice04-plus-focused-fixes | Implement the focused Rust best-practice repair slices opened by Slice03. | Slice03. | Exact slice count is intentionally deferred until the audit and baseline tests reveal the true shape. |

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

## Arc Ledger

Capability: evidence-based Rust best-practice audit and repairs, with behavior
baselined before production code changes.

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
|----|-----------|--------|--------------|--------|--------|----------|-------|
| A3-1 | Slice01 produces a diagnosis-only audit with no source, test, manifest, CI, or README edits. | `test -f 01-cleanup/arc03-rust-best-practices/slice01-diagnosis-only-audit/cdc-verification.md` and inspect the no-source-edit row. | serious | arc-plan | done | `slice01-diagnosis-only-audit/cdc-verification.md` verifies 12/12 rows, full quality gate reproduction, runtime probes, and workbench-only implementation diff. | Read-only means audit reports plus planning close artifacts only. |
| A3-2 | Slice02 records current behavior with missing characterization tests before any production repair slice begins. | `test -f 01-cleanup/arc03-rust-best-practices/slice02-baseline-characterization-tests/cdc-verification.md` and inspect that the diff is test-only. | serious | operator-question | done | `slice02-baseline-characterization-tests/cdc-verification.md` verifies 12/12 rows, 13 new characterization tests, test-only `main..HEAD` diff, full quality gate reproduction, and runtime probes `0`, `0`, `101`, `101`. | This protects behavior before repairs. |
| A3-3 | Slice03 maps audit findings to focused fix slices or explicit deferrals. | `test -f 01-cleanup/arc03-rust-best-practices/slice03-triage-and-fix-map/cdc-verification.md` and inspect fix-map rows. | correctness | arc-plan | open | | Exact fix slice count is not guessed in advance. |
| A3-4 | Every production repair slice opened by Slice03 closes with CDC verification. | `find 01-cleanup/arc03-rust-best-practices -path "*/cdc-verification.md" -print` and compare against the Slice03 fix map. | correctness | ledger-discipline | open | | Uses remediation slices rather than broad unbounded repair. |
| A3-5 | The final Arc03 code state passes the full local workflow-equivalent gate. | `cargo fmt --check`, `cargo check --all-targets`, `RUSTFLAGS="-D rust-2024-compatibility" cargo check --all-targets`, `cargo clippy --all-targets -- -D warnings`, `cargo test --all-targets`, `cargo build --release --bins`, `./target/release/hddl_analyzer --help`, `actionlint .github/workflows/ci.yml`, and `git diff --check`. | serious | arc-plan | open | | Run at arc close on the final repair branch state. |
| A3-6 | Arc03 bubble-up findings are routed into Arc04 or project-level deferrals before Arc03 closes. | `rg -n "Arc04|cohesion|deferred|bubble-up" 01-cleanup/arc03-rust-best-practices/closing-report.md 01-cleanup/project-plan.md` | correctness | project-plan | open | | Prevents audit findings from silently disappearing. |

## Version History

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
