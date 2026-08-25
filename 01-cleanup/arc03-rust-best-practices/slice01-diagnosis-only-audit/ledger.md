# Slice01: Diagnosis-Only Rust Audit Ledger

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
|----|-----------|--------|--------------|--------|--------|----------|-------|
| R3-1 | The audit date is captured from the system command and used as the workbench filename prefix. | `date +%Y.%m.%d` and `test -f workbench/<DATE>-audit-index.md` and `test -f workbench/<DATE>-audit-results-rust.md` | correctness | code-audit | open | | Do not infer the date. |
| R3-2 | Repository context is read before auditing. | Inspect audit reports for `README.md`, root guidance file status, and architecture-doc discovery notes. | correctness | code-audit | open | | Missing architecture context must be recorded as a gap. |
| R3-3 | Language and tool detection is recorded while ignoring generated/build/worktree trees. | `rg -n "Languages detected|Rust|skipped|no skill available|target|\\.worktrees" workbench/<DATE>-audit-index.md` | correctness | code-audit | open | | Detect by manifests/configs and extensions. |
| R3-4 | Rust guideline substrate is loaded and cited as audit basis. | `rg -n "11-anti-patterns|rust-guidelines|CLI|error|async|Cargo|project structure|API" workbench/<DATE>-audit-results-rust.md` | serious | rust-guidelines | open | | Load topic-specific guides as needed. |
| R3-5 | The top-level audit index exists and summarizes language/tool scope, architecture discovery, and severity counts. | `test -f workbench/<DATE>-audit-index.md` and `rg -n "Project root|Languages detected|Finding counts|Architecture" workbench/<DATE>-audit-index.md` | correctness | code-audit | open | | |
| R3-6 | The Rust audit report exists and covers source, binaries, test modules, and integration tests. | `test -f workbench/<DATE>-audit-results-rust.md` and `rg -n "Executive summary|Findings|tests|integration|src/|src/bin" workbench/<DATE>-audit-results-rust.md` | correctness | code-audit | open | | |
| R3-7 | Each finding uses severity, concrete location, what/why/fix, and an actionable recommendation. | Inspect `workbench/<DATE>-audit-results-rust.md` for the required per-finding shape. | serious | code-audit | open | | No generic advice. |
| R3-8 | The report records at least five negative checks. | `rg -n "Things I looked for and did not find" workbench/<DATE>-audit-results-rust.md` and inspect at least five clean checks. | correctness | code-audit | open | | Prevents filler-only audits. |
| R3-9 | The Arc02 async `RwLock` candidate is explicitly reviewed. | `rg -n "RwLock|read guard|await|language-server|diagnostic" workbench/<DATE>-audit-results-rust.md` | serious | arc02-bubble-up | open | | Finding, no-op, or deferral must be explicit. |
| R3-10 | Missing baseline characterization tests for Slice02 are identified or explicitly ruled out with rationale. | `rg -n "characterization|baseline|Slice02|missing tests|no missing" workbench/<DATE>-audit-results-rust.md` | serious | operator-question | open | | This is the handoff to the test-only slice. |
| R3-11 | The audited code state passes the local workflow-equivalent quality gate. | `cargo fmt --check`; `cargo check --all-targets`; `RUSTFLAGS="-D rust-2024-compatibility" cargo check --all-targets`; `cargo clippy --all-targets -- -D warnings`; `cargo test --all-targets`; `cargo build --release --bins`; `./target/release/hddl_analyzer --help`; `actionlint .github/workflows/ci.yml`; `git diff --check` | serious | arc01-arc02 | open | | Audit should not regress the quality baseline. |
| R3-12 | The slice remains diagnosis-only/read-only with no Rust source, test, manifest, workflow, or README edits. | Inspect `git diff --name-only` in the implementation worktree and confirm only `workbench/` audit reports changed there. | serious | operator-question | open | | Planning close artifacts live in the planning worktree after implementation. |

## What Worked

_(Fill during close with patterns that made the audit easy to verify.)_

## Closure

_(Filled by CC at slice close.)_
