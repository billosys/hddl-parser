# CC Prompt: Arc04 Slice02 Triage And Fix Map

You are working in HDDL-Parser on Arc04 Slice02:
`arc04-rust-cohesion-audit/slice02-triage-and-fix-map`.

This is a planning-only slice. Do not edit Rust source, tests, Cargo files,
workflows, README/docs, or runtime behavior in this slice. Do not add
characterization tests here. Do not create close-set files for downstream slices.

## Required Reading

Read these files first:

- `/Users/oubiwann/lab/billosys/hddl-parser/.worktrees/planning/01-cleanup/project-plan.md`
- `/Users/oubiwann/lab/billosys/hddl-parser/.worktrees/planning/01-cleanup/arc04-rust-cohesion-audit/arc-plan.md`
- `/Users/oubiwann/lab/billosys/hddl-parser/.worktrees/planning/01-cleanup/arc04-rust-cohesion-audit/slice01-cohesion-diagnosis-audit/cdc-verification.md`
- `/Users/oubiwann/lab/billosys/hddl-parser/.worktrees/planning/01-cleanup/arc04-rust-cohesion-audit/slice02-triage-and-fix-map/slice-doc.md`
- `/Users/oubiwann/lab/billosys/hddl-parser/.worktrees/planning/01-cleanup/arc04-rust-cohesion-audit/slice02-triage-and-fix-map/ledger.md`
- `workbench/2026.08.25-cohesion-audit-index.md`
- `workbench/2026.08.25-cohesion-audit-results-rust.md`

Also apply `$collaboration-framework` ledger/project-management discipline and
`$rust-guidelines` API/error/test/Cargo judgment where the findings touch Rust
compatibility or public surface design.

## Goal

Create a concrete fix map for all Arc04 Slice01 cohesion findings and open the
next slices that are clear enough to hand off without another planning pass.

## Required Output

Create:

- `/Users/oubiwann/lab/billosys/hddl-parser/.worktrees/planning/01-cleanup/arc04-rust-cohesion-audit/slice02-triage-and-fix-map/fix-map.md`

Cover every finding:

- `COHESION-001` / RUST-007 public parser inputs and root exports.
- `COHESION-002` parser unexpected-token error boundary cohesion.
- `COHESION-003` error taxonomy and stringly transformation errors.
- `COHESION-004` formula normalization panic-only invariants.
- `COHESION-005` test assertion helper drift.
- `COHESION-006` public and near-public spelling/naming drift.

For each finding, include:

- severity and theme;
- disposition: focused repair slice, accepted variation/no-op, later-arc
  deferral, or duplicate of already-settled Arc03 work;
- target repair slice or deferral destination;
- rationale;
- public API compatibility risk;
- whether operator GO is required before implementation;
- expected behavior or API change;
- existing baseline or required characterization-test action;
- proposed upstream PR grouping.

## Initial Grouping Hypothesis To Evaluate

Evaluate this hypothesis, but do not blindly copy it:

- Open a test-only characterization slice before production repair if current
  behavior/API is not already pinned. Likely targets: `&[u8]` parser callers,
  intended public imports, malformed problem-parser behavior, error variants,
  formula normalization panic contracts, and public spelling/variant surfaces.
- Split `COHESION-001` if needed: `&Vec<u8>` to `&[u8]` may be a focused
  compatibility-improving repair; crate-root export narrowing is a public API
  policy decision and may require operator GO or accepted variation.
- Keep `COHESION-002` as a focused parser error-boundary repair after the
  malformed problem baseline exists.
- Treat `COHESION-003` and public parts of `COHESION-006` as compatibility-policy
  work before implementation.
- For `COHESION-004`, choose whether to document panic contracts, add fallible
  APIs, restrict visibility, or defer to a later API-design arc.
- Defer test helper consolidation until behavior/API baselines are in place.

## Slice Opening Requirements

If the fix map opens downstream slices, create a complete open set for each:

- `slice-doc.md`
- `ledger.md`
- `cc-prompt.md`

Do not create downstream `closing-report.md` or `cdc-verification.md` files.

At minimum, if you decide a downstream slice is needed but not yet handoff-ready,
record a deferral or operator-GO gate with a concrete re-entry condition instead
of opening a vague repair slice.

## Plan Updates

Update `arc04-rust-cohesion-audit/arc-plan.md` if your map replaces the
`slice03-plus-focused-cohesion-repairs` placeholder with concrete slices.

Update `project-plan.md` if Arc04 status, project sequencing, public API policy,
upstream PR grouping, or project-close readiness changes.

## Constraints

- Planning only. No implementation source/test/manifest/workflow/README changes.
- No characterization tests in this slice.
- No public API break may be treated as approved without operator GO.
- No finding may disappear because it is Low severity.
- No broad cleanup branch. Keep one pattern family per future repair slice unless
  the fix map justifies a broader grouping.
- Leave pre-existing Slice01 workbench artifacts in the feature worktree alone.

## Verification Commands

Run and record:

```bash
test -f 01-cleanup/arc04-rust-cohesion-audit/slice02-triage-and-fix-map/fix-map.md
rg -n "COHESION-001|COHESION-002|COHESION-003|COHESION-004|COHESION-005|COHESION-006"   01-cleanup/arc04-rust-cohesion-audit/slice02-triage-and-fix-map/fix-map.md
rg -n "focused repair slice|accepted variation|no-op|later-arc deferral|duplicate|disposition"   01-cleanup/arc04-rust-cohesion-audit/slice02-triage-and-fix-map/fix-map.md
rg -n "public API|compatibility|operator GO|baseline|characterization|test-only|PR grouping|re-entry"   01-cleanup/arc04-rust-cohesion-audit/slice02-triage-and-fix-map/fix-map.md
rg -n "slice03|slice04|slice05|COHESION-001|COHESION-006|Slice02"   01-cleanup/arc04-rust-cohesion-audit/arc-plan.md   01-cleanup/project-plan.md
find 01-cleanup/arc04-rust-cohesion-audit -maxdepth 2 -type f | sort
git diff --name-only
git diff --check
git -C ../features status --short --branch --untracked-files=all
```

If you open downstream slices, also run:

```bash
find 01-cleanup/arc04-rust-cohesion-audit -maxdepth 2 \( -path '*/slice-doc.md' -o -path '*/ledger.md' -o -path '*/cc-prompt.md' \) -print | sort
find 01-cleanup/arc04-rust-cohesion-audit -maxdepth 2 \( -path '*/closing-report.md' -o -path '*/cdc-verification.md' \) -print | sort
```

## Closing Report Requirements

When done, update the Slice02 ledger and add `closing-report.md`. The closing
report must:

- Walk every ledger row `M4-1` through `M4-13`.
- State the final disposition for every `COHESION-*` finding.
- List exactly which downstream slices were opened and why.
- Name any operator GO gates or later-arc deferrals with re-entry conditions.
- Bubble up arc-plan and project-plan changes.
- Prove the planning-only diff boundary.
