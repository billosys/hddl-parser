# Slice01: Ignored Test Investigation

## Ledger

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
|----|-----------|--------|--------------|--------|--------|----------|-------|
| I5-1 | The slice inventories every ignored Rust test under `src/` and `tests/`, including file, test name, and ignore reason. | `rg -n "#\\[ignore" src tests -g '*.rs'` and inspect the investigation report. | serious | slice-doc | open | | |
| I5-2 | The slice records Git provenance for each ignored-test annotation. | `git blame -L <range> -- <file>` for every ignored annotation and inspect the investigation report. | correctness | slice-doc | open | | |
| I5-3 | The slice runs each ignored test explicitly or records a bounded-timeout / non-run rationale. | Inspect command transcript in the investigation report for `cargo test --locked <ignored-test-name> -- --ignored --exact` or bounded timeout evidence. | serious | slice-doc | open | | Long-running IPC/JSON tests may require bounded probes. |
| I5-4 | The slice identifies the intended behavior and implementation code path for each ignored test. | Inspect the investigation report for per-test `intended behavior` and `code path` fields. | correctness | operator-follow-up | open | | |
| I5-5 | The slice classifies each ignored test into exactly one follow-up route: test-only fix, code-and-test repair, code-and-test rewrite, slow/corpus gate, or valid deferral. | `rg -n "test-only fix|code-and-test repair|code-and-test rewrite|slow/corpus gate|valid deferral" workbench 01-cleanup/arc05-skipped-test-repair/slice01-ignored-test-investigation` and inspect matrix. | serious | operator-follow-up | open | | |
| I5-6 | The slice does not modify production code, tests, manifests, workflows, README, fixtures, or ignored-test annotations. | `git diff --name-status` in the implementation worktree shows only allowed workbench investigation artifacts, or no implementation diff. | serious | project-management | open | | Planning close artifacts are allowed in the planning worktree. |
| I5-7 | The slice does not create downstream repair-slice open sets before the investigation closes. | `find 01-cleanup/arc05-skipped-test-repair -maxdepth 2 -name 'slice-doc.md' -print` shows only Slice01 during this slice. | correctness | project-management | open | | |
