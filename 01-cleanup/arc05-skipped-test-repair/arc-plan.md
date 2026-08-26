# Arc05: Skipped Test Repair

Version: 1.13
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
path, likely failure class, and recommended follow-up route. Status: locally
closed; CDC verification pending.

### slice02-fast-ignored-test-repair

Repair the three fast ignored tests identified by Slice01: rewrite
`file_type_test` as a meaningful parser classification test, repair the
malformed typed-parameter diagnostics exposed by
`forgotten_dash_validation_test` and
`forgotten_question_mark_validation_test`, and remove only those three
`#[ignore]` annotations once the tests pass in the default locked gate. Status:
CDC-verified.

### slice03-corpus-measurement

Add a Rust-native corpus measurement utility that deterministically enumerates
the IPC corpus and records phase-level timings for parse/verify and JSON
round-trip behavior. This slice measures first: it does not choose the final
slow/corpus CI policy, does not remove the corpus `#[ignore]` annotations, and
does not add sharding, manifests, or custom test harnesses. Status:
CDC-verified.

### slice04-corpus-addressability-and-policy

Make the corpus measurement utility policy-ready without wiring CI yet. This
slice adds addressable corpus selections, a checked-in fast-selection policy,
named fast/full command surfaces, and explicit JSON assertion behavior so the
remaining corpus tests can move behind intentional gates rather than inherited
`#[ignore]` annotations. Status: CDC-verified.

### slice05-corpus-test-routing

Route the two remaining inherited corpus ignored tests through the verified
Slice04 policy surface. Default tests gain fast corpus coverage for IPC
verification and JSON round-tripping, while full corpus validation remains an
explicit policy command and CI scheduling stays for the next slice. Status:
CDC-verified.

### slice06-github-actions-corpus-policy

Wire the verified corpus command surfaces into GitHub Actions policy. This
slice should decide and implement branch-push, PR, scheduled, and
post-merge/main corpus gates without changing the parser/test semantics proven
by Slice05. Status: open.

### Later Repair Slices

Do not pre-open additional repair slices beyond Slice06 until the corpus CI
policy slice is opened and verified. Later slices should be cut from the
remaining investigation findings, timing evidence, and Slice04 policy surface
by route and review boundary, for example:

- Test-only repairs where the implementation already behaves correctly and the
  test is stale, slow, or poorly scoped.
- Code-and-test repairs where the test exposes a real bug or underspecified
  behavior.
- Code-and-test rewrites where both the tested implementation and inherited
  test shape are too stale or incoherent to repair safely in place.
- Slow/corpus test policy work if long-running IPC/JSON coverage should move
  behind a named opt-in command rather than stay as ordinary ignored tests.
- Corpus structure work if measurement shows a checked-in manifest,
  addressable case IDs, representative fast samples, or structural JSON
  assertions are needed before CI policy changes.

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
| A5-1 | Slice01 investigates every ignored Rust test without changing source, tests, manifests, workflows, README, fixtures, or ignore annotations. | `rg -n "#\\[ignore" src tests -g '*.rs'` and inspect Slice01 close evidence; `git diff --name-status` confirms read-only implementation scope. | serious | operator-follow-up | open | `slice01-ignored-test-investigation/closing-report.md` locally closes the read-only investigation; implementation `git diff --name-status` is empty and the allowed workbench report is ignored. CDC verification is pending. | |
| A5-2 | Slice01 classifies each ignored test into a follow-up route: test-only fix, code-and-test repair, code-and-test rewrite, slow/corpus gate, or valid deferral. | `test -f 01-cleanup/arc05-skipped-test-repair/slice01-ignored-test-investigation/closing-report.md` and inspect classification matrix. | serious | operator-follow-up | open | Slice01 local close classifies `file_type_test` as test-only fix; the two forgotten-declaration tests as code-and-test repair; and the IPC/JSON corpus tests as slow/corpus gate. CDC verification is pending. | |
| A5-3 | Later repair slices are opened only after Slice01 findings identify concrete route boundaries. | `find 01-cleanup/arc05-skipped-test-repair -maxdepth 2 -name 'slice-doc.md' -print` and inspect arc version history. | correctness | project-management | done | `slice02-fast-ignored-test-repair` opened from Slice01's route matrix and CDC-verified; `slice03-corpus-measurement` opened for the remaining corpus route. | Additional corpus-policy slices remain unopened until Slice03 measurement evidence lands. |
| A5-4 | Repaired tests either run in the default locked test gate or move behind an explicit slow/corpus gate with documented invocation. | `cargo test --locked --all-targets` and inspect any slow-test command introduced by later slices. | serious | arc-plan | open | Slice02 repair commit `e534df2` puts the three fast non-corpus ignored tests in the default locked gate. Slice03 leaves the two corpus ignores in place and records opt-in measurement commands plus timing evidence for the later slow/corpus policy slice. | Corpus policy is not finalized yet. |
| A5-5 | Arc05 closes with a composition report showing no inherited ignored-test behavior remains unexplained. | `test -f 01-cleanup/arc05-skipped-test-repair/closing-report.md` and inspect row walk. | serious | project-management | open | | |
| A5-6 | Corpus slow-test policy decisions are based on deterministic case inventory and phase-level timing evidence rather than inferred runtime. | `test -f 01-cleanup/arc05-skipped-test-repair/slice03-corpus-measurement/closing-report.md` and inspect the measurement report for corpus counts, slowest cases, parse/verify timing, JSON phase timing, and structural-vs-string JSON comparison notes. | correctness | operator-follow-up | done | Slice03 CDC verification reproduced deterministic inventory of 900 cases, full measurement completion with 0 failures and 0 JSON equality disagreements, phase totals, distribution buckets, slowest cases/domains, and next-slice policy options. | Slice03 is measurement-first and does not finalize the CI policy by itself. |
| A5-7 | The remaining corpus ignored tests have an addressable fast/full policy substrate before CI wiring or ignore removal. | `test -f 01-cleanup/arc05-skipped-test-repair/slice04-corpus-addressability-and-policy/cc-prompt.md` and inspect Slice04 close evidence for named fast/full commands, checked-in selection policy, JSON assertion policy, and unchanged workflow files. | serious | Slice03 bubble-up | done | Slice04 CDC verification reproduced named fast/full selection support, a checked-in 43-domain fast selection, custom manifest support, explicit JSON assertion modes, clean negative policy/input failures, and unchanged workflow/corpus-test boundaries. | Slice04 did not remove the two corpus ignores or change GitHub Actions. |
| A5-8 | The two inherited corpus ignored tests are routed or replaced so no Rust `#[ignore]` annotations remain, while default tests run only fast corpus coverage. | `test -f 01-cleanup/arc05-skipped-test-repair/slice05-corpus-test-routing/cc-prompt.md`; after close, inspect Slice05 evidence for no ignored tests, fast IPC/JSON default coverage, full command preservation, and unchanged workflows. | serious | Slice04 bubble-up | done | Slice05 CDC verification reproduced: `rg -n "#\\[ignore" src tests -g '*.rs'` has no matches; `cargo test --locked --test ipc` passes 1/0 ignored; `cargo test --locked --test json` passes 9/0 ignored; `cargo test --locked --all-targets` passes with IPC/JSON enabled; full `HDDL_CORPUS_SELECTION=full HDDL_CORPUS_ASSERT=both cargo run --locked --example corpus_measure` passes 900/900 with zero failures and zero JSON assertion failures; workflows unchanged. | CI policy remains separate for Slice06. |
| A5-9 | GitHub Actions corpus policy is opened against the verified fast/default and full explicit command surfaces. | `test -f 01-cleanup/arc05-skipped-test-repair/slice06-github-actions-corpus-policy/cc-prompt.md`; inspect Slice06 open set for branch-push, PR, scheduled, and post-merge/main decisions. | serious | Slice05 bubble-up | done | Slice06 open set created with `slice-doc.md`, `ledger.md`, and `cc-prompt.md`; it scopes branch-push fast corpus, PR full corpus, post-merge main/master full corpus, and explicit scheduled-policy disposition. | Slice05 verified the command surface but did not change workflows. |
| A5-10 | GitHub Actions corpus policy is implemented and verified without changing parser/test semantics. | After Slice06 close, inspect `slice06-github-actions-corpus-policy/cdc-verification.md`, `.github/workflows/ci.yml`, `actionlint`, local fast/full corpus commands, and full locked Rust gates. | serious | Slice06 open | open | | This is the likely final implementation row before Arc05 composition close. |

## Version History

### v1.13 - 2026-08-26

Opened Slice06 GitHub Actions corpus policy. The slice wires the verified
Slice04/Slice05 command surfaces into CI: fast/default corpus coverage for
branch pushes, full corpus coverage for pull requests and post-merge
`main`/`master`, and an explicit scheduled-run disposition. Added A5-10 as the
implementation-and-verification row that gates whether Arc05 can close next.

### v1.12 - 2026-08-26

Slice05 CDC verification landed at feature commit `af0968b`. CDC reproduced
the no-ignore check, focused IPC/JSON fast corpus tests, full default locked
test gate, fast and full corpus measurement commands, unchanged workflow
boundary, `actionlint`, and all mechanical Rust gates. Arc05 now has no
remaining inherited Rust `#[ignore]` annotations; the next planned slice is
Slice06 GitHub Actions corpus policy.

### v1.11 - 2026-08-26

Slice05 locally closed. The two inherited corpus skipped tests now run through
enabled default IPC and JSON integration routes backed by the checked-in
43-case fast corpus selection. No Rust `#[ignore]` annotations remain under
`src` or `tests`; the full 900-case corpus command remains explicit and passed
with zero failures, zero JSON equality disagreements, and zero JSON assertion
failures. GitHub Actions remained unchanged. CDC verification and the later CI
policy slice are still pending.

### v1.10 - 2026-08-26

Opened Slice05 for corpus test routing. The slice is scoped to eliminating the
two inherited corpus `#[ignore]` annotations by replacing or routing them
through fast default corpus tests while preserving the explicit full corpus
policy command. CI branch, PR, scheduled, and post-merge policy remain reserved
for the following slice.

### v1.9 - 2026-08-26

Slice04 CDC verification landed. CDC reproduced the named fast/full and custom
manifest corpus runs, all four JSON assertion modes, negative policy/input
failure paths, unchanged workflow/corpus-test boundaries, and the full locked
local gate. Arc05 can now plan the final corpus test routing and CI policy
slices against the verified addressability substrate.

### v1.8 - 2026-08-26

Slice04 locally closed. The corpus runner now exposes named fast/full
selections, custom manifest support, clean non-zero policy/input errors, and
explicit JSON assertion modes. The checked-in fast selection covers 43 domains
using the fastest measured case per domain from Slice03 evidence. CI wiring and
corpus `#[ignore]` removal remain later work after CDC verification.

### v1.7 - 2026-08-26

Opened Slice04 as the corpus addressability and policy substrate. The slice is
scoped to stable selections, named fast/full commands, checked-in fast corpus
policy, and explicit JSON string/structural assertion behavior. CI wiring and
corpus `#[ignore]` removal remain later work after the policy surface exists.

### v1.6 - 2026-08-26

Slice03 CDC verification landed. CDC independently reproduced bounded,
filtered, and full 900-case corpus measurement runs, confirmed both generated
CSV reports contain 900 clean case rows, and verified that default tests, CI,
and the two corpus ignored tests remain unchanged. `Cargo.toml` changed only to
declare the explicit `corpus_measure` example target at `tools/corpus_measure.rs`.
Next Arc05 work should make the corpus addressable and policy-ready before
wiring the branch/PR/post-merge gates.

### v1.5 - 2026-08-26

Slice03 locally closed corpus measurement. The Rust-native measurement utility
enumerated 900 IPC cases, completed the full parse/verify plus JSON
round-trip run with zero failures and zero equality disagreements, and wrote
ignored workbench CSV/summary evidence. Arc05 now has timing-backed corpus
policy inputs, but the remaining IPC/JSON ignored tests intentionally stay
behind a later policy or infrastructure slice.

### v1.4 - 2026-08-26

Slice02 CDC verification landed at feature commit `e534df2`. The three fast
non-corpus ignored tests are now enabled and passing in the default locked
test gate, with specific AST/diagnostic assertions. The only remaining ignored
Rust tests are the IPC and JSON corpus tests, preserving Slice03 as the
measurement-first path for corpus policy.

### v1.3 - 2026-08-26

Opened Slice03 as a measurement-first corpus slice. The slice records
deterministic IPC corpus inventory and phase-level timings for parse/verify and
JSON round-trip behavior before the project chooses whether later corpus work
needs manifests, addressable case IDs, representative fast samples, structural
JSON assertions, sharding, or CI policy changes. Operator direction captured:
fast representative corpus coverage may belong on branch pushes, while full
corpus coverage should target PRs and post-merge `main` on both Linux and
macOS, not only one platform.

### v1.2 - 2026-08-26

Opened Slice02 from Slice01's route matrix. Scope is limited to the three fast
non-corpus ignored tests: `file_type_test`,
`forgotten_dash_validation_test`, and
`forgotten_question_mark_validation_test`. The IPC and JSON corpus ignored
tests remain deliberately outside Slice02 while the operator and CDC design the
slow/corpus measurement and infrastructure strategy.

### v1.1 - 2026-08-26

Slice01 locally closed the ignored-test investigation. Five ignored Rust tests
were inventoried, blamed, explicitly probed, and classified: one test-only fix,
two code-and-test repairs, and two slow/corpus-gate routes. No implementation
source, test, fixture, manifest, workflow, README, or ignore annotation changed.
No downstream repair-slice open sets were created. CDC verification is pending.

### v1.0 - 2026-08-26

Arc opened as a remediation follow-up for inherited ignored tests. Slice01 is
the only open slice: a read-only investigation that records enough evidence to
choose whether later slices fix tests, repair code and tests, rewrite code and
tests, or establish a slow/corpus-test policy.
