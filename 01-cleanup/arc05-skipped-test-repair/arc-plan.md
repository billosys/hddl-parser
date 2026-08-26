# Arc05: Skipped Test Repair

Version: 1.0
Date: 2026-08-26
Expected investigation branch: `audit/ignored-tests`
Expected repair branches: smaller `fix/...` branches as Slice01 requires

## Capability

Repair the inherited ignored-test debt so every skipped test has an explicit,
evidence-backed disposition: enabled, replaced, intentionally moved behind a
separate slow-test gate, or deferred with a concrete reason and re-entry
condition.

This arc may repair only tests, update production code and tests together, or
rewrite production code and tests together. Slice01 decides which route applies
to each ignored test by investigating the current behavior, original intent,
fixture cost, and tested code quality before any implementation changes land.

## Relationship To Earlier Arcs

Arc01 provides the CI and local gate shape. Arc02 sets the Rust 2024 baseline.
Arc03 and Arc04 repair correctness, error-boundary, reproducibility, and
cohesion issues before skipped-test repair begins in earnest.

Arc05 should start implementation from the final Arc04 feature baseline, not
from an in-progress Slice05 state, unless the operator explicitly chooses to
interleave the work. The investigation slice may run while Arc04 closes because
it is read-only and records findings rather than applying repairs.

## Slice Breakdown

### slice01-ignored-test-investigation

Run an investigation-only, read-only audit of all ignored Rust tests. For each
ignored test, record the test name, file, ignore reason, Git provenance,
runtime behavior when run explicitly, intended covered behavior, tested code
path, likely failure class, and recommended follow-up route. Status: open.

### Later Repair Slices

Do not pre-open repair slices until Slice01 closes. Later slices should be cut
from the investigation findings by route and review boundary, for example:

- Test-only repairs where the implementation already behaves correctly and the
  test is stale, slow, or poorly scoped.
- Code-and-test repairs where the test exposes a real bug or underspecified
  behavior.
- Code-and-test rewrites where both the tested implementation and inherited
  test shape are too stale or incoherent to repair safely in place.
- Slow/corpus test policy work if long-running IPC/JSON coverage should move
  behind a named opt-in command rather than stay as ordinary ignored tests.

## Dependencies

- Arc01 workflow-equivalent local gate.
- Arc02 Rust 2024 compatibility gate.
- Arc03 and Arc04 final feature state, once Arc04 is CDC-closed.
- Current ignored-test inventory from the repository, not memory or prior chat
  summaries.

## Operating Rules

- Slice01 is read-only. It may create workbench investigation reports and
  planning close evidence, but it must not edit source, tests, manifests,
  workflows, README, fixtures, or ignored-test annotations.
- Do not remove `#[ignore]` from any test until a later repair slice has an
  explicit behavior-preservation story and acceptance criteria.
- Running ignored tests explicitly is allowed during Slice01, but failures,
  panics, timeouts, and long runtimes are evidence to record, not defects to fix
  in that slice.
- Do not assume all ignored tests should become default CI tests. Long-running
  corpus coverage may need a named opt-in command or scheduled gate.
- If an ignored test is tied to malformed syntax, semantic validation, parser
  routing, JSON round-tripping, or IPC corpus behavior, identify the exact
  implementation path before recommending a repair route.
- Later slices must preserve or improve coverage; deleting a skipped test is
  only acceptable when replacement coverage is present and verified.

## Arc Ledger

Definition of done: inherited ignored tests are investigated and repaired or
given explicit, policy-backed disposition so skipped coverage no longer hides
unknown behavior.

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
|----|-----------|--------|--------------|--------|--------|----------|-------|
| A5-1 | Slice01 investigates every ignored Rust test without changing source, tests, manifests, workflows, README, fixtures, or ignore annotations. | `rg -n "#\\[ignore" src tests -g '*.rs'` and inspect Slice01 close evidence; `git diff --name-status` confirms read-only implementation scope. | serious | operator-follow-up | open | | |
| A5-2 | Slice01 classifies each ignored test into a follow-up route: test-only fix, code-and-test repair, code-and-test rewrite, slow/corpus gate, or valid deferral. | `test -f 01-cleanup/arc05-skipped-test-repair/slice01-ignored-test-investigation/closing-report.md` and inspect classification matrix. | serious | operator-follow-up | open | | |
| A5-3 | Later repair slices are opened only after Slice01 findings identify concrete route boundaries. | `find 01-cleanup/arc05-skipped-test-repair -maxdepth 2 -name 'slice-doc.md' -print` and inspect arc version history. | correctness | project-management | open | | Only Slice01 is opened at arc start. |
| A5-4 | Repaired tests either run in the default locked test gate or move behind an explicit slow/corpus gate with documented invocation. | `cargo test --locked --all-targets` and inspect any slow-test command introduced by later slices. | serious | arc-plan | open | | |
| A5-5 | Arc05 closes with a composition report showing no inherited ignored-test behavior remains unexplained. | `test -f 01-cleanup/arc05-skipped-test-repair/closing-report.md` and inspect row walk. | serious | project-management | open | | |

## Version History

### v1.0 - 2026-08-26

Arc opened as a remediation follow-up for inherited ignored tests. Slice01 is
the only open slice: a read-only investigation that records enough evidence to
choose whether later slices fix tests, repair code and tests, rewrite code and
tests, or establish a slow/corpus-test policy.
