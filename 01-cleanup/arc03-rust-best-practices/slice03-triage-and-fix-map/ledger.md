# Slice03: Triage And Fix Map Ledger

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
|----|-----------|--------|--------------|--------|--------|----------|-------|
| C3-1 | Slice03 remains planning-only, with no production Rust, test, manifest, workflow, README, or feature-branch implementation changes. | `git diff --name-only` from the planning worktree; `git -C ../features status --short --branch --untracked-files=all` | serious | slice-doc | open | | The feature worktree may still contain pre-existing untracked Slice01 workbench audit files; distinguish them from Slice03 changes. |
| C3-2 | The fix map covers every audit finding from `RUST-001` through `RUST-008`. | `rg -n "RUST-001|RUST-002|RUST-003|RUST-004|RUST-005|RUST-006|RUST-007|RUST-008" 01-cleanup/arc03-rust-best-practices/slice03-triage-and-fix-map/fix-map.md` | serious | audit | open | | No silent drops, including Low findings. |
| C3-3 | Each finding has exactly one explicit disposition: fix in Arc03, defer to Arc04, defer outside this cleanup project, or no-op. | Inspect the finding table in `fix-map.md` for a disposition column and one row per finding. | serious | ledger-discipline | open | | Deferrals/no-ops must not masquerade as done. |
| C3-4 | Each Arc03 fix disposition names a target repair slice and proposed upstream PR grouping. | `rg -n "target slice|PR grouping|fix in Arc03|slice04|slice05|slice06|slice07" 01-cleanup/arc03-rust-best-practices/slice03-triage-and-fix-map/fix-map.md` | serious | upstream-contribution | open | | Reviewability matters: avoid one giant best-practices PR unless justified. |
| C3-5 | Every deferral has a destination, rationale, and concrete re-entry condition. | `rg -n "defer|Arc04|outside this cleanup|re-entry|rationale" 01-cleanup/arc03-rust-best-practices/slice03-triage-and-fix-map/fix-map.md` | correctness | ledger-discipline | open | | "Later" is not a valid deferral. |
| C3-6 | Slice02 characterization baselines are mapped to the repairs that will update them. | `rg -n "tests/current_behavior.rs|tests/lsp_current_behavior.rs|current_behavior|baseline|update" 01-cleanup/arc03-rust-best-practices/slice03-triage-and-fix-map/fix-map.md` | serious | slice02-cdc | open | | The map should identify which tests change when each behavior repair lands. |
| C3-7 | LSP harness limitations from Slice02 are explicitly carried forward into target slices or deferrals. | `rg -n "RwLock|contention|non-file|didSave|unreadable|harness|re-entry" 01-cleanup/arc03-rust-best-practices/slice03-triage-and-fix-map/fix-map.md` | serious | slice02-cdc | open | | Prevents brittle LSP cases from disappearing. |
| C3-8 | The arc plan is updated to replace or refine `slice04-plus-focused-fixes` if concrete repair slices are opened. | `rg -n "slice04-plus-focused-fixes|slice04|slice05|slice06|triage|fix-map|RUST-001|RUST-008" 01-cleanup/arc03-rust-best-practices/arc-plan.md` | correctness | project-management | open | | If the placeholder remains, the closing report must justify why. |
| C3-9 | Any opened repair slice has a complete open set and no premature close-set artifacts. | `find 01-cleanup/arc03-rust-best-practices -maxdepth 2 -type f | sort` | correctness | project-management | open | | Required open files are `slice-doc.md`, `ledger.md`, and `cc-prompt.md`; close files wait for evidence. |
| C3-10 | Project-plan status is updated if Slice03 changes Arc03 sequencing, PR grouping, or Arc04 responsibilities. | `rg -n "Arc03|Slice03|Slice04|Arc04|cohesion|best-practices|PR" 01-cleanup/project-plan.md` | correctness | project-management | open | | No project-plan change is needed if the arc-plan absorbs all changes locally. |
| C3-11 | The final planning diff is internally consistent and whitespace-clean. | `git diff --check` and inspect `git diff --stat` | correctness | ledger-discipline | open | | |
| C3-12 | The closing report bubbles up whether Slice03 opened concrete repair slices, changed arc sequencing, or deferred any finding. | Inspect `closing-report.md` for a row walk and Bubble-up to the arc section. | correctness | project-management | open | | Written only after CC completes the slice. |

## What Worked

_(At slice close. Patterns that made the slice close cleanly.)_

## Closure

_(At slice close. Record commit/date/verifier/row counts.)_
