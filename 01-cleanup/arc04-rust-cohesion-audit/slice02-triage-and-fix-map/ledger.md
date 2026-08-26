# Slice02: Triage And Fix Map Ledger

Definition of done: Slice02 converts the Arc04 cohesion findings into an
explicit repair map, opens concrete downstream slice open sets where justified,
and preserves a planning-only diff boundary.

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
|----|-----------|--------|--------------|--------|--------|----------|-------|
| M4-1 | Slice02 remains planning-only, with no Rust source, test, manifest, workflow, README, or feature implementation changes. | `git diff --name-only` from planning; `git -C ../features status --short --branch --untracked-files=all` | serious | slice-doc | open | | Feature worktree may already show Slice01 workbench intent-to-add entries; no new feature changes should be introduced. |
| M4-2 | The fix map exists in the Slice02 directory. | `test -f 01-cleanup/arc04-rust-cohesion-audit/slice02-triage-and-fix-map/fix-map.md` | correctness | project-management | open | | |
| M4-3 | The fix map covers every Slice01 finding `COHESION-001` through `COHESION-006` exactly once in the primary finding table. | `rg -n "COHESION-001|COHESION-002|COHESION-003|COHESION-004|COHESION-005|COHESION-006" 01-cleanup/arc04-rust-cohesion-audit/slice02-triage-and-fix-map/fix-map.md` plus table inspection. | serious | slice01-cdc | open | | No Low finding may be silently dropped. |
| M4-4 | Every finding has one disposition: focused repair slice, accepted variation/no-op, later-arc deferral, or duplicate of settled Arc03 work. | Inspect the disposition column in `fix-map.md`; `rg -n "focused repair slice|accepted variation|no-op|later-arc deferral|duplicate|disposition" 01-cleanup/arc04-rust-cohesion-audit/slice02-triage-and-fix-map/fix-map.md` | serious | ledger-discipline | open | | |
| M4-5 | Every disposition includes rationale, target slice or deferral destination, expected behavior/API change, and upstream PR grouping. | `rg -n "rationale|target slice|deferral destination|expected behavior|expected API|PR grouping" 01-cleanup/arc04-rust-cohesion-audit/slice02-triage-and-fix-map/fix-map.md` | serious | upstream-contribution | open | | |
| M4-6 | Public API compatibility risk is classified for all public-surface findings, and any breaking change is gated by explicit operator GO or deferred. | `rg -n "public API|compatibility|operator GO|breaking|pre-1.0|re-entry" 01-cleanup/arc04-rust-cohesion-audit/slice02-triage-and-fix-map/fix-map.md` | serious | rust-guidelines | open | | Applies especially to COHESION-001, COHESION-003, COHESION-004, and COHESION-006. |
| M4-7 | Baseline or characterization-test needs are identified before production repairs. | `rg -n "baseline|characterization|test-only|existing tests" 01-cleanup/arc04-rust-cohesion-audit/slice02-triage-and-fix-map/fix-map.md` | serious | slice01-cdc | open | | Behavior-changing or API-changing repairs should not precede their baselines. |
| M4-8 | Accepted variations and no-op decisions, if any, have a specific local rationale. | `rg -n "accepted variation|no-op|local rationale|intentional divergence" 01-cleanup/arc04-rust-cohesion-audit/slice02-triage-and-fix-map/fix-map.md` | correctness | operator-follow-up | open | | If none are chosen, fix map must say so. |
| M4-9 | Any later-arc deferral has a concrete destination, reason, and re-entry condition. | `rg -n "later-arc deferral|destination|reason|re-entry" 01-cleanup/arc04-rust-cohesion-audit/slice02-triage-and-fix-map/fix-map.md` | correctness | ledger-discipline | open | | If none are chosen, fix map must say so. |
| M4-10 | Every downstream slice opened by Slice02 has a complete open set and no close-set artifacts. | `find 01-cleanup/arc04-rust-cohesion-audit -maxdepth 2 -type f | sort` | correctness | project-management | open | | Open set means `slice-doc.md`, `ledger.md`, `cc-prompt.md`; no `closing-report.md` or `cdc-verification.md` for future slices. |
| M4-11 | Arc04 and project plans are updated to reflect the selected downstream slice breakdown, current status, and any PR grouping or deferral changes. | `rg -n "slice02|slice03|slice04|COHESION-001|COHESION-006|Slice02" 01-cleanup/arc04-rust-cohesion-audit/arc-plan.md 01-cleanup/project-plan.md` | serious | project-management | open | | |
| M4-12 | Planning diff is internally consistent and whitespace-clean. | `git diff --name-only` and `git diff --check` | correctness | ledger-discipline | open | | |
| M4-13 | Closing report walks every row and bubbles up the repair map, opened slices, operator gates, and deferrals. | `test -f 01-cleanup/arc04-rust-cohesion-audit/slice02-triage-and-fix-map/closing-report.md` and inspect row walk. | correctness | ledger-discipline | open | | |

## What Worked

Pending Slice02 close.

## Closure

Pending Slice02 close.
