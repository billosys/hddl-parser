# Slice02: Triage And Fix Map Ledger

Definition of done: Slice02 converts the Arc04 cohesion findings into an
explicit repair map, opens concrete downstream slice open sets where justified,
and preserves a planning-only diff boundary.

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
|----|-----------|--------|--------------|--------|--------|----------|-------|
| M4-1 | Slice02 remains planning-only, with no Rust source, test, manifest, workflow, README, or feature implementation changes. | `git diff --name-only` from planning; `git -C ../features status --short --branch --untracked-files=all` | serious | slice-doc | done | Planning diff only; feature status remains pre-existing Slice01 workbench entries on `audit/rust-cohesion`. | No feature-worktree files were edited by Slice02. |
| M4-2 | The fix map exists in the Slice02 directory. | `test -f 01-cleanup/arc04-rust-cohesion-audit/slice02-triage-and-fix-map/fix-map.md` | correctness | project-management | done | Command exited 0. | |
| M4-3 | The fix map covers every Slice01 finding `COHESION-001` through `COHESION-006` exactly once in the primary finding table. | `rg -n "COHESION-001|COHESION-002|COHESION-003|COHESION-004|COHESION-005|COHESION-006" 01-cleanup/arc04-rust-cohesion-audit/slice02-triage-and-fix-map/fix-map.md` plus table inspection. | serious | slice01-cdc | done | Grep matched primary table rows 25-30; table inspection found all six findings represented. | Low findings COHESION-005/COHESION-006 retained. |
| M4-4 | Every finding has one disposition: focused repair slice, accepted variation/no-op, later-arc deferral, or duplicate of settled Arc03 work. | Inspect the disposition column in `fix-map.md`; `rg -n "focused repair slice|accepted variation|no-op|later-arc deferral|duplicate|disposition" 01-cleanup/arc04-rust-cohesion-audit/slice02-triage-and-fix-map/fix-map.md` | serious | ledger-discipline | done | Grep passed; disposition column inspected. | COHESION-001 and COHESION-006 are intentionally split between focused repair and operator-gated public API policy. |
| M4-5 | Every disposition includes rationale, target slice or deferral destination, expected behavior/API change, and upstream PR grouping. | `rg -n "rationale|target slice|deferral destination|expected behavior|expected API|PR grouping" 01-cleanup/arc04-rust-cohesion-audit/slice02-triage-and-fix-map/fix-map.md` | serious | upstream-contribution | done | Fix map primary table contains all required columns and PR grouping text. | |
| M4-6 | Public API compatibility risk is classified for all public-surface findings, and any breaking change is gated by explicit operator GO or deferred. | `rg -n "public API|compatibility|operator GO|breaking|pre-1.0|re-entry" 01-cleanup/arc04-rust-cohesion-audit/slice02-triage-and-fix-map/fix-map.md` | serious | rust-guidelines | done | Grep passed; operator GO gates section added. | Applies to export narrowing, error taxonomy, formula API, and public spelling variants. |
| M4-7 | Baseline or characterization-test needs are identified before production repairs. | `rg -n "baseline|characterization|test-only|existing tests" 01-cleanup/arc04-rust-cohesion-audit/slice02-triage-and-fix-map/fix-map.md` | serious | slice01-cdc | done | Grep passed; Slice03 opened as test-only baseline. | `&[u8]` positive tests are routed to Slice04 because they cannot pass before signature changes. |
| M4-8 | Accepted variations and no-op decisions, if any, have a specific local rationale. | `rg -n "accepted variation|no-op|local rationale|intentional divergence" 01-cleanup/arc04-rust-cohesion-audit/slice02-triage-and-fix-map/fix-map.md` | correctness | operator-follow-up | done | Grep passed; accepted variation/no-op section names local reasons. | No new no-op silently closes a Slice01 finding. |
| M4-9 | Any later-arc deferral has a concrete destination, reason, and re-entry condition. | `rg -n "later-arc deferral|destination|reason|re-entry" 01-cleanup/arc04-rust-cohesion-audit/slice02-triage-and-fix-map/fix-map.md` | correctness | ledger-discipline | done | Grep passed; later-arc deferral section names public API/error/AST contract arc and re-entry condition. | |
| M4-10 | Every downstream slice opened by Slice02 has a complete open set and no close-set artifacts. | `find 01-cleanup/arc04-rust-cohesion-audit -maxdepth 2 -type f | sort` | correctness | project-management | done | Find output includes Slice03-Slice05 `slice-doc.md`, `ledger.md`, and `cc-prompt.md`; close-set find shows Slice01 close files plus this Slice02 closing report. | No downstream `closing-report.md` or `cdc-verification.md` files were created. |
| M4-11 | Arc04 and project plans are updated to reflect the selected downstream slice breakdown, current status, and any PR grouping or deferral changes. | `rg -n "slice02|slice03|slice04|COHESION-001|COHESION-006|Slice02" 01-cleanup/arc04-rust-cohesion-audit/arc-plan.md 01-cleanup/project-plan.md` | serious | project-management | done | Grep passed; arc-plan v1.3 and project-plan v2.11 updated. | Plans now name Slice03-Slice05 and public API gates. |
| M4-12 | Planning diff is internally consistent and whitespace-clean. | `git diff --name-only` and `git diff --check` | correctness | ledger-discipline | done | `git diff --check` passed; diff name list contains only Arc04/project planning artifacts. | New planning files were marked intent-to-add for diff visibility. |
| M4-13 | Closing report walks every row and bubbles up the repair map, opened slices, operator gates, and deferrals. | `test -f 01-cleanup/arc04-rust-cohesion-audit/slice02-triage-and-fix-map/closing-report.md` and inspect row walk. | correctness | ledger-discipline | done | `closing-report.md` added with M4-1 through M4-13 walk. | |

## What Worked

Separating compatibility-improving repairs from public API breakage kept the
next work concrete without silently approving risky changes.

## Closure

Slice02 is complete pending CDC verification. It produced `fix-map.md`, opened
Slice03 through Slice05 open sets, updated the Arc04/project plans, and recorded
operator GO gates plus later-arc deferrals for public API/error/AST changes.
