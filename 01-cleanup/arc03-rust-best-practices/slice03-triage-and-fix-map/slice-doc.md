# Slice03: Triage And Fix Map

Version: 1.0
Date: 2026-08-25
Arc: `arc03-rust-best-practices`
Expected branch: `planning`
Implementation branch under review: `audit/rust-best-practices`

## Goal

Convert the Arc03 audit findings and Slice02 baseline-test evidence into a
focused, reviewable repair plan. This slice decides which findings stay in
Arc03, which move to Arc04 cohesion work, which are deferred with explicit
re-entry conditions, and how the remaining repairs should be grouped into
upstream-friendly branches/PRs.

This is a planning and triage slice. It should open the next repair work from
a known baseline without changing Rust implementation behavior itself.

## Scope

In scope:

- Read the committed audit artifacts:
  - `workbench/2026.08.25-audit-index.md`
  - `workbench/2026.08.25-audit-results-rust.md`
- Read Slice01 and Slice02 close evidence, especially the Slice02 CDC
  verification and LSP harness deferrals.
- Inspect the current Rust source and tests only as needed to make grouping
  and sequencing decisions.
- Create a fix-map artifact under this slice directory that covers every
  audit finding `RUST-001` through `RUST-008`.
- For every finding, record:
  - disposition: fix in Arc03, defer to Arc04, defer outside this cleanup
    project, or no-op;
  - target repair slice or deferral destination;
  - rationale;
  - expected behavior change;
  - test baseline or missing-test action;
  - PR grouping recommendation.
- Update `arc03-rust-best-practices/arc-plan.md` so the post-Slice03 slice
  breakdown reflects the chosen repair slices instead of the placeholder
  `slice04-plus-focused-fixes`, if the map opens concrete repair slices.
- Create open sets for any immediate next repair slices that Slice03 opens.
  Prefer opening only slices whose scope is clear enough to hand to CC without
  another planning pass.
- Update `01-cleanup/project-plan.md` if the fix map changes project-level
  sequencing, PR grouping, or Arc04 responsibilities.

Out of scope:

- Production Rust source changes.
- Test changes, except creating planning artifacts for future test work.
- Dependency-version, lockfile, API, CLI, or LSP repairs.
- Marking any finding complete without a concrete fix, deferral, or no-op
  rationale.
- Opening close artifacts for Slice03 before implementation evidence exists.
- Silently dropping lower-severity findings because higher-severity fixes are
  more urgent.

## Verification Approach

CDC should verify this slice by reading the fix map, comparing it against the
audit and Slice02 CDC evidence, and confirming that every finding has exactly
one explicit disposition. Because this is planning-only, the primary evidence
is artifact consistency rather than runtime behavior.

Expected verification commands:

```bash
test -f 01-cleanup/arc03-rust-best-practices/slice03-triage-and-fix-map/fix-map.md
rg -n "RUST-001|RUST-002|RUST-003|RUST-004|RUST-005|RUST-006|RUST-007|RUST-008" \
  01-cleanup/arc03-rust-best-practices/slice03-triage-and-fix-map/fix-map.md
rg -n "fix in Arc03|defer to Arc04|defer outside|no-op|PR grouping|baseline|re-entry" \
  01-cleanup/arc03-rust-best-practices/slice03-triage-and-fix-map/fix-map.md
rg -n "slice04|slice05|slice06|RUST-001|RUST-008|Arc04|deferred" \
  01-cleanup/arc03-rust-best-practices/arc-plan.md \
  01-cleanup/project-plan.md
git diff --name-only
git diff --check
```

If Slice03 opens additional repair-slice directories, CDC should also verify
that each opened slice has exactly the open-set files:

```bash
find 01-cleanup/arc03-rust-best-practices -maxdepth 2 -type f | sort
```

## Exit Criteria

- A fix-map artifact exists and covers all eight audit findings.
- Every finding has one explicit disposition and rationale.
- Every Arc03 repair finding has a target slice and proposed PR grouping.
- Every deferral has a destination and re-entry condition.
- Slice02 baselines are mapped to the repair slices that will update them.
- LSP harness limits from Slice02 are carried forward explicitly.
- The Arc03 arc plan is updated to match the selected repair-slice breakdown.
- Any project-level sequencing or Arc04 scope change is bubbled into the
  project plan.
- No production Rust, tests, manifests, workflows, or README changes are made
  by this planning slice.
