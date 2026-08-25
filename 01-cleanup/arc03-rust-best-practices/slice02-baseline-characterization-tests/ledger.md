# Slice02: Baseline Characterization Tests Ledger

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
|----|-----------|--------|--------------|--------|--------|----------|-------|
| C3-1 | The slice diff is test-only and contains no production behavior changes. | `git diff --name-only` and inspect any `src/` edits for `#[cfg(test)]`-only additions. | serious | operator-question | open | | `Cargo.toml` changes are allowed only for test targets/dev-dependencies with rationale. |
| C3-2 | CLI current behavior is characterized for success and recoverable error paths. | `rg -n "current_behavior|characterization|missing|unsupported|semantic|write|success" tests src` and `cargo test --all-targets` | serious | RUST-001 | open | | Error-path tests should assert today's exit `0`, not the later desired non-zero exit. |
| C3-3 | Public HDDL constructor and transpiler domain/problem mismatch behavior is characterized. | `rg -n "from_hddl|domain.*problem|problem.*domain|catch_unwind|should_panic|current_behavior" tests src` and `cargo test --all-targets` | serious | RUST-002 | open | | Panic-capturing tests are acceptable only when named as current undesired behavior. |
| C3-4 | Domain-only `remove-equality-constraints` behavior is characterized. | `rg -n "remove-equality|RemoveEqualityConstraints|catch_unwind|current_behavior|characterization" tests src` and `cargo test --all-targets` | serious | RUST-003 | open | | Capture today's panic/error shape before the repair slice changes it. |
| C3-5 | LSP initialize metadata behavior is characterized or explicitly deferred with a harness limitation. | `rg -n "initialize|ServerInfo|CARGO_PKG_VERSION|0\\.1\\.0|characterization|deferred" tests src 01-cleanup/arc03-rust-best-practices/slice02-baseline-characterization-tests` | correctness | RUST-008 | open | | Today's baseline should record the stale version behavior if a harness can reach it. |
| C3-6 | LSP diagnostic failure-path behavior is characterized or explicitly deferred with a harness limitation. | `rg -n "did_save|diagnostic|non-file|missing|domain|unwrap|characterization|deferred" tests src 01-cleanup/arc03-rust-best-practices/slice02-baseline-characterization-tests` | correctness | RUST-005 | open | | Do not make private internals public only to satisfy this row. |
| C3-7 | The diagnostic `RwLock` contention candidate is handled as current behavior, not as a post-fix assertion. | `rg -n "RwLock|read guard|contention|diagnostic|current behavior|post-fix|deferred" tests src 01-cleanup/arc03-rust-best-practices/slice02-baseline-characterization-tests` | serious | RUST-004 | open | | If a stable test cannot be written before refactor, defer with the exact re-entry condition. |
| C3-8 | RUST-006 dependency/lockfile policy is not mixed into this test baseline slice. | `git diff -- Cargo.toml .gitignore Cargo.lock` and inspect for absence of dependency-version/lockfile policy changes. | correctness | RUST-006 | open | | Dev-dependencies for tests are allowed only if justified; production dependency policy waits for Slice03/fix map. |
| C3-9 | No new ignored tests are introduced. | `git diff -- tests src Cargo.toml | rg "#\\[ignore\\]|ignore ="` | serious | ledger-discipline | open | | If the command finds pre-existing ignores, closing must prove they are not new. |
| C3-10 | The full local workflow-equivalent quality gate passes. | `cargo fmt --check`; `cargo check --all-targets`; `RUSTFLAGS="-D rust-2024-compatibility" cargo check --all-targets`; `cargo clippy --all-targets -- -D warnings`; `cargo test --all-targets`; `cargo build --release --bins`; `./target/release/hddl_analyzer --help`; `actionlint .github/workflows/ci.yml`; `git diff --check` | serious | arc01-arc02 | open | | |
| C3-11 | The audit runtime probes still show current behavior, proving no repair slipped into the baseline slice. | Run the four probes from `workbench/2026.08.25-audit-results-rust.md` and compare exit codes `0`, `0`, `101`, `101`. | serious | cdc-verification | open | | Later fix slices will intentionally change these baselines. |
| C3-12 | Slice02 closing report bubbles up the exact test baselines and any deferred LSP harness work before Slice03 opens. | Inspect `closing-report.md` for a row walk and Bubble-up to the arc section. | correctness | project-management | open | | Slice03 depends on this map to open focused repair slices. |

## What Worked

_(Fill during close with patterns that made the characterization baseline easy to verify.)_

## Closure

_(Filled by CC at slice close.)_
