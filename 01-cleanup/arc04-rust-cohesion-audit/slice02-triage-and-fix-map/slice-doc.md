# Slice02: Triage And Fix Map

Version: 1.0
Date: 2026-08-26
Arc: `arc04-rust-cohesion-audit`
Expected branch: `planning`
Implementation branch under review: `audit/rust-cohesion`

## Goal

Convert the Arc04 Slice01 cohesion audit into a concrete repair map. This slice
assigns every `COHESION-*` finding to exactly one disposition and opens the next
repair or characterization slices that are clear enough to hand to CC without
another planning pass.

This is a planning-only slice. It does not change Rust source, tests, Cargo
files, workflows, README files, or runtime behavior. Its job is to make the
remaining Arc04 work reviewable, correctly sequenced, and explicit about public
API compatibility decisions.

## Scope

In scope:

- Read the Arc04 Slice01 audit reports and CDC verification.
- Inspect current Rust source and tests only as needed to validate grouping,
  compatibility risk, and baseline-test needs.
- Create `fix-map.md` in this slice directory.
- Cover every finding: `COHESION-001` through `COHESION-006`.
- For every finding, record:
  - severity and theme;
  - disposition: focused repair slice, accepted variation/no-op, later-arc
    deferral, or duplicate of already-settled Arc03 work;
  - target slice or deferral destination;
  - rationale;
  - public API compatibility risk;
  - whether operator GO is required before implementation;
  - expected behavior/API change;
  - behavior-preservation baseline or missing characterization-test action;
  - proposed upstream PR grouping.
- Decide which baseline characterization tests must land before production
  repairs. Prefer a test-only characterization slice when the behavior/API
  contract is not already pinned.
- Open complete open sets for downstream slices that the fix map makes
  concrete. Each opened slice must have `slice-doc.md`, `ledger.md`, and
  `cc-prompt.md`, and must not have close-set files yet.
- Update `arc04-rust-cohesion-audit/arc-plan.md` if the placeholder focused
  repair entry is replaced by concrete slices.
- Update `project-plan.md` if Slice02 changes Arc04 status, project sequencing,
  upstream PR grouping, or the final cleanup/project-close path.

Out of scope:

- Rust source, test, Cargo, workflow, README, or behavior changes.
- Adding characterization tests in this slice.
- Fixing any cohesion finding.
- Treating public API breaks as approved without an explicit operator GO gate.
- Marking a finding complete without a concrete rationale.
- Creating closing reports or CDC verification for Slice02 or downstream slices
  before the slice has actually run.

## Initial Grouping Hypothesis

Evaluate this hypothesis, but do not blindly copy it:

- A test-only characterization slice is likely needed before production repair,
  covering `&[u8]` public parser callers, intended public imports, malformed
  problem-parser behavior, transformation/domain-problem error variants, formula
  normalization panic contracts, and public spelling/variant compatibility.
- `COHESION-001` may need to split: borrowed byte inputs can probably be a
  focused compatibility-improving repair, while crate-root export narrowing is a
  public API decision and may need operator GO or accepted-variation treatment.
- `COHESION-002` is likely a focused parser error-boundary repair after a
  malformed-problem baseline exists.
- `COHESION-003` and public portions of `COHESION-006` need an explicit public
  compatibility policy before implementation.
- `COHESION-004` needs a policy choice: document current panic contracts, add
  fallible APIs, restrict visibility, or defer to a later public API arc.
- `COHESION-005` and private/test-only spelling cleanup should happen only after
  behavior-changing repairs have their baselines.

## Verification Approach

CDC should verify this planning slice by comparing `fix-map.md`, opened slice
open sets, and plan updates against the Slice01 audit and CDC verification. The
primary evidence is artifact consistency and a clean planning-only diff.

Expected verification commands:

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

If Slice02 opens downstream slices, CDC must verify each opened downstream slice
has exactly its open set and no premature close artifacts:

```bash
find 01-cleanup/arc04-rust-cohesion-audit -maxdepth 2 \( -path '*/slice-doc.md' -o -path '*/ledger.md' -o -path '*/cc-prompt.md' \) -print | sort
find 01-cleanup/arc04-rust-cohesion-audit -maxdepth 2 \( -path '*/closing-report.md' -o -path '*/cdc-verification.md' \) -print | sort
```

## Exit Criteria

- `fix-map.md` exists.
- Every Slice01 finding `COHESION-001` through `COHESION-006` appears exactly
  once in the primary finding map.
- Every finding has one explicit disposition, rationale, target, compatibility
  risk, baseline need, and PR grouping recommendation.
- Public API changes are either gated by explicit operator GO, documented as
  accepted variation/no-op, or deferred with a re-entry condition.
- Required characterization-test needs are routed before production repairs.
- Downstream slices opened by Slice02 have complete open sets and no close-set
  artifacts.
- Arc04 and project plans are updated if sequencing or status changes.
- No implementation source, tests, manifests, workflows, README files, or
  behavior are changed by this slice.
