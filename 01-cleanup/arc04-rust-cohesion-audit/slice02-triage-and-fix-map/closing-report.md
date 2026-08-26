# Slice02 Closing Report: Triage And Fix Map

Date: 2026-08-25
Planning branch: planning
Implementation branch under review: audit/rust-cohesion

## Artifacts

- `01-cleanup/arc04-rust-cohesion-audit/slice02-triage-and-fix-map/fix-map.md`
- `01-cleanup/arc04-rust-cohesion-audit/slice03-characterization-baselines/slice-doc.md`
- `01-cleanup/arc04-rust-cohesion-audit/slice03-characterization-baselines/ledger.md`
- `01-cleanup/arc04-rust-cohesion-audit/slice03-characterization-baselines/cc-prompt.md`
- `01-cleanup/arc04-rust-cohesion-audit/slice04-parser-api-and-error-boundary/slice-doc.md`
- `01-cleanup/arc04-rust-cohesion-audit/slice04-parser-api-and-error-boundary/ledger.md`
- `01-cleanup/arc04-rust-cohesion-audit/slice04-parser-api-and-error-boundary/cc-prompt.md`
- `01-cleanup/arc04-rust-cohesion-audit/slice05-test-helper-and-private-naming-cohesion/slice-doc.md`
- `01-cleanup/arc04-rust-cohesion-audit/slice05-test-helper-and-private-naming-cohesion/ledger.md`
- `01-cleanup/arc04-rust-cohesion-audit/slice05-test-helper-and-private-naming-cohesion/cc-prompt.md`

Plan updates:

- `01-cleanup/arc04-rust-cohesion-audit/arc-plan.md`
- `01-cleanup/project-plan.md`

## Finding Dispositions

- COHESION-001 / RUST-007: focused repair for `&[u8]` parser inputs in Slice04;
  public export narrowing requires operator GO or later public API deferral.
- COHESION-002: focused parser error-boundary repair in Slice04 after Slice03
  characterizes the current panic path.
- COHESION-003: later-arc deferral to a public API/error taxonomy arc, with
  current variants/messages characterized in Slice03.
- COHESION-004: later-arc deferral to a public AST/API contract arc, with
  current formula panic contracts characterized in Slice03.
- COHESION-005: focused test helper cleanup in Slice05.
- COHESION-006: private/test naming cleanup in Slice05; public enum variant
  spelling changes require operator GO or later public API deferral.

## Downstream Slices Opened

- Slice03 `slice03-characterization-baselines`: test-only current-behavior
  coverage before production repair.
- Slice04 `slice04-parser-api-and-error-boundary`: compatibility-improving
  parser byte-slice inputs plus structured malformed-problem errors.
- Slice05 `slice05-test-helper-and-private-naming-cohesion`: test helper cleanup
  and private/test-only spelling repair.

No downstream closing reports or CDC verification files were created.

## Operator GO Gates And Deferrals

Operator GO is required before:

- narrowing crate-root public exports;
- replacing public error enum shapes or renaming `ParsingError::Lexiacal`;
- renaming `Transformation::QuantifierElimintation`;
- restricting public `Formula` visibility or replacing public panic contracts
  with fallible APIs.

Later-arc destination: a future public API/error/AST contract arc.

Re-entry condition: open that public API contract arc before any 1.0-style API
stabilization, crate publication, downstream API support commitment, or
operator-approved breaking cleanup PR.

## Ledger Walk

- M4-1: Done. Slice02 changed planning artifacts only. Feature status still
  shows the pre-existing Slice01 workbench entries on `audit/rust-cohesion`; no
  feature files were edited here.
- M4-2: Done. `fix-map.md` exists.
- M4-3: Done. The primary table covers all six Slice01 findings.
- M4-4: Done. Every finding has a disposition; two findings are intentionally
  split between focused repair and public API gate because the prompt allowed
  splitting where needed.
- M4-5: Done. Each row includes rationale, target or deferral destination,
  expected behavior/API change, and PR grouping.
- M4-6: Done. Public API compatibility risk and operator GO gates are recorded.
- M4-7: Done. Baseline/characterization needs are identified, with Slice03
  opened before production repair.
- M4-8: Done. Accepted variation/no-op treatment has local rationale and does
  not silently close a finding.
- M4-9: Done. Later-arc deferrals have destination, reason, and re-entry
  condition.
- M4-10: Done. Slice03, Slice04, and Slice05 have complete open sets; no future
  close-set files were created.
- M4-11: Done. Arc04 and project plans are updated to the concrete downstream
  sequence and public API gate policy.
- M4-12: Done. Planning diff is internally consistent and whitespace-clean.
- M4-13: Done. This report walks every row and bubbles up the repair map.

## Verification

Executed from `.worktrees/planning` unless noted:

- `test -f 01-cleanup/arc04-rust-cohesion-audit/slice02-triage-and-fix-map/fix-map.md` passed.
- `rg -n "COHESION-001|COHESION-002|COHESION-003|COHESION-004|COHESION-005|COHESION-006" 01-cleanup/arc04-rust-cohesion-audit/slice02-triage-and-fix-map/fix-map.md` passed.
- `rg -n "focused repair slice|accepted variation|no-op|later-arc deferral|duplicate|disposition" 01-cleanup/arc04-rust-cohesion-audit/slice02-triage-and-fix-map/fix-map.md` passed.
- `rg -n "public API|compatibility|operator GO|baseline|characterization|test-only|PR grouping|re-entry" 01-cleanup/arc04-rust-cohesion-audit/slice02-triage-and-fix-map/fix-map.md` passed.
- `rg -n "slice03|slice04|slice05|COHESION-001|COHESION-006|Slice02" 01-cleanup/arc04-rust-cohesion-audit/arc-plan.md 01-cleanup/project-plan.md` passed.
- `find 01-cleanup/arc04-rust-cohesion-audit -maxdepth 2 -type f | sort` showed the expected Arc04 plan, Slice01 close artifacts, Slice02 files, and Slice03-Slice05 open sets.
- `find 01-cleanup/arc04-rust-cohesion-audit -maxdepth 2 \( -path '*/slice-doc.md' -o -path '*/ledger.md' -o -path '*/cc-prompt.md' \) -print | sort` showed complete open sets for Slice01 through Slice05.
- `find 01-cleanup/arc04-rust-cohesion-audit -maxdepth 2 \( -path '*/closing-report.md' -o -path '*/cdc-verification.md' \) -print | sort` showed existing Slice01 close artifacts and this Slice02 closing report; no downstream close artifacts were present.
- `git diff --check` passed.
- `git -C ../features status --short --branch --untracked-files=all` showed only pre-existing Slice01 workbench entries on `audit/rust-cohesion`.

## Bubble-Up To Arc04

Slice02 delivers its assigned planning function. It replaces the placeholder
repair entry in `arc-plan.md` with concrete Slice03-Slice05 entries and records
public API gates/deferrals.

Arc04 now proceeds in this order:

1. Slice03 characterization baselines.
2. Slice04 parser API and parser error-boundary repair.
3. Slice05 test helper and private naming cohesion.
4. Arc close, unless the operator authorizes a public API break within Arc04 or
   opens the deferred public API/error/AST contract arc.

## Bubble-Up To Project

`project-plan.md` was updated to show Arc04 Slice02 locally closed, Slice03-Slice05
opened, and public API breaking work deferred behind operator GO. The cleanup
project remains scoped to focused upstream PRs; a future public API contract PR
is explicitly separated from the current Arc04 parser/test cleanup work.

Slice02 is closed pending CDC verification.
