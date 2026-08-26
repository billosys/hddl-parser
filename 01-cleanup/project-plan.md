# HDDL-Parser Cleanup Project Plan

Version: 2.26
Date: 2026-08-25
Planning branch: `planning` orphan branch
Implementation base: local `main` at `ec2d70e`
Upstream tracker: https://github.com/koala-planner/HDDL-Parser/issues/4

## Definition Of Done

This project prepares HDDL-Parser for a sequence of focused upstream pull
requests that improve contributor confidence without changing the tool's
planning-language semantics.

The project delivers:

- A CI workflow PR that gives maintainers automatic feedback for builds,
  formatting, linting, tests, and binary smoke checks.
- A Rust 2024 edition migration PR that is separated from CI and warning-fix
  churn.
- A Rust best-practices audit/fix PR series that is evidence-based and split
  when findings are too broad for one reviewable patch.
- A final Rust cohesion audit/fix pass that checks the whole codebase for
  intentionally consistent idioms, not merely locally-correct Rust.
- A skipped-test repair arc that investigates every ignored test, then fixes
  the tests and any underlying code they reveal as stale, incorrect, or in need
  of rewrite.

Explicit non-goals:

- No language-semantics changes unless the later audit finds a confirmed bug.
- No JSON schema redesign in this cleanup project.
- No release automation, publishing automation, or MSRV policy change unless a
  later arc explicitly scopes it.

## Roadmap

| Arc | Capability | Status | Dependencies |
|-----|------------|--------|--------------|
| arc01-github-actions-ci | Adds the first GitHub Actions CI workflow and status-facing project polish. | closed locally; PR-ready after upstream base settles | Warning-fix PR #5 merged or equivalent local baseline. |
| arc02-rust-2024-edition | Migrates the crate to the latest Rust edition using the standard edition workflow. | active; slices CDC-verified, arc close pending | arc01 gives CI coverage for the migration PR. |
| arc03-rust-best-practices | Audits Rust API, error handling, CLI, tests, and maintainability issues; baselines behavior with tests before production repairs. | closed locally | arc01 gives CI coverage; arc02 defines the target edition. |
| arc04-rust-cohesion-audit | Performs the final whole-codebase consistency pass so Rust idioms, error shapes, data-flow patterns, and module conventions feel deliberately unified. | closed locally | arc03 local feature state at `d820065` gives the repaired-codebase audit base. |
| arc05-skipped-test-repair | Investigates and repairs inherited ignored tests, including refactoring or rewriting tested code when an ignored test exposes stale or defective behavior. | active; Slice04 open | arc04 closed locally at final cohesion baseline `7e2d8a7`. |

## Current Status

Arc01 is closed locally with a workflow-equivalent verification pass and a
recorded caveat that the CI PR should be stacked on the warning-fix baseline or
rebased after PR #5 lands. Arc02 is active: both slices have CDC verification,
including the strict Rust 2024 compatibility CI gate. Arc02 still needs
arc-level composition closure and upstream base reconciliation before the Rust
2024 PR should be opened.

Arc03 is closed locally. Slice01 is CDC-verified as a diagnosis-only audit with
only workbench audit artifacts in the implementation worktree. Slice02 is
CDC-verified with test-only baseline characterization coverage before any
production repair slice begins. Slice03 maps the audit findings into five
focused Arc03 repair slices and one Arc04 deferral, and is CDC-verified: CLI
exit codes, structured parser/transform errors, LSP error boundaries and
metadata, LSP diagnostic lock scope, Cargo reproducibility policy, and RUST-007
public API cohesion for Arc04. Slice04 is CDC-verified: RUST-001 is fixed, the
updated CLI baselines pass, and the feature diff includes operator-approved
`workbench` ignore hygiene. Slice05 is CDC-verified: RUST-002/RUST-003 mapped
panic paths now return structured errors and the former panic probes exit
through Slice04's ordinary CLI error path. Slice06 is CDC-verified:
RUST-005/RUST-008 scoped LSP request-boundary and initialize metadata failures
are fixed, with RUST-004 intentionally left for Slice07. Slice07 is
CDC-verified: RUST-004 diagnostic lock scope is fixed, with deterministic
runtime contention coverage deferred to a future test hook or in-crate direct
handler harness. Slice08 is CDC-verified: RUST-006 Cargo reproducibility policy
is fixed with explicit dependency requirements and a tracked Cargo-generated
lockfile. Arc03 is closed locally: all slices have CDC verification, the final
composed feature state passes the workflow-equivalent gate, and RUST-007 is
routed to Arc04 public API cohesion.

Arc04 is active. Slice01 is CDC-verified as a diagnosis-only cohesion audit
from the Arc03 final feature state at `d820065`. The audit produced six
cohesion findings, seven negative checks, and concrete Slice02 handoff items
while preserving the read-only implementation boundary. Slice02 is
CDC-verified: it maps all six findings, opens Slice03 test-only
characterization baselines, Slice04 parser API/error-boundary repair, and
Slice05 test-helper/private-naming cohesion, and keeps public API breaking
changes behind explicit operator GO or a later public API/error/AST contract
arc. Slice03 is CDC-verified with a test-only integration baseline. Slice04 is
CDC-verified: borrowed byte-input APIs now accept `&[u8]`, Vec-backed callers
remain covered, the malformed problem-parser panic path now returns
`ParsingError::Syntactic`, and crate-root export narrowing remains skipped and
gated. Slice05 is CDC-verified: flawed-domain integration-test helper
duplication is consolidated without weakening assertion precision, scoped
private/test-only spelling drift is repaired, and public misspelled enum
variants remain deferred. All planned Arc04 slices are now CDC-verified; Arc04
is closed locally with an arc-level composition report. The close bubbles the
inherited ignored-test debt to Arc05 rather than folding test repair into the
cohesion PR.

Arc05 is active as a follow-on remediation arc for inherited ignored tests.
Slice01 is locally closed: five ignored Rust tests are inventoried, blamed,
explicitly probed, and classified into one test-only fix, two code-and-test
repairs, and two slow/corpus-gate follow-ups. Slice02 is CDC-verified: the
three fast non-corpus ignored tests are now enabled, specific, and passing in
the default locked gate. Slice03 is CDC-verified: the IPC corpus measurement
utility completed all 900 cases with zero failures and zero JSON equality
disagreements in both CC's workbench report and CDC's independent temp-report
rerun, giving Arc05 timing-backed inputs for the remaining corpus policy and
infrastructure work before the final ignored tests are moved. Slice04 is open
to make the corpus addressable and policy-ready before CI wiring or corpus
ignore removal.

The warning-fix PR is treated as predecessor work, not part of this planning
packet.

## Project Ledger

Definition of done: the expected upstream cleanup sequence is planned and each
arc can be executed as a focused PR or PR series.

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
|----|-----------|--------|--------------|--------|--------|----------|-------|
| P-1 | Arc01 CI is closed and composed. | `test -f 01-cleanup/arc01-github-actions-ci/closing-report.md` and inspect row walk. | serious | project-plan | done | `01-cleanup/arc01-github-actions-ci/closing-report.md` records all Arc01 rows done and the full local workflow-equivalent command passing. | CI PR readiness depends on upstream PR #5 base/merge sequencing. |
| P-2 | Arc02 edition migration has an active arc plan and Slice01/Slice02 open sets. | `test -f 01-cleanup/arc02-rust-2024-edition/arc-plan.md` and `test -f 01-cleanup/arc02-rust-2024-edition/slice01-edition-migration/cc-prompt.md` and `test -f 01-cleanup/arc02-rust-2024-edition/slice02-edition-verification-and-pr/cc-prompt.md` | correctness | project-plan | done | Arc02 plan promoted from placeholder; Slice01 and Slice02 open sets created. | Slice02 executes after Slice01 close evidence exists. |
| P-3 | Arc03 Rust best-practices audit/fix arc was opened with Slice01 read-only and Slice02 reserved for pre-repair characterization tests. | `test -f 01-cleanup/arc03-rust-best-practices/arc-plan.md` and `test -f 01-cleanup/arc03-rust-best-practices/slice01-diagnosis-only-audit/cc-prompt.md` and `rg -n "diagnosis-only|read-only|baseline characterization|before production" 01-cleanup/arc03-rust-best-practices/arc-plan.md 01-cleanup/arc03-rust-best-practices/slice01-diagnosis-only-audit/*.md` | correctness | project-plan | done | Arc03 plan v1.0 created and Slice01 open set added. | Arc03 is now closed locally; see `arc03-rust-best-practices/closing-report.md`. |
| P-4 | Project boundaries keep CI, edition migration, best-practices, and cohesion work in separate upstream PRs or PR families. | `rg -n "fix/cargo-warnings|feature/add-ci|edition/rust-2024|best-practices|cohesion|separate" 01-cleanup/project-plan.md 01-cleanup/arc*/arc-plan.md` | serious | issue-4 | done | Slice03 fix map separates Arc03 into PR groups for CLI exits, structured parser/transform errors, LSP robustness, and Cargo reproducibility, while RUST-007 is deferred to Arc04 cohesion. | Upstream branches should stay focused even if multiple Arc03 slices later land in one coordinated PR family. |
| P-5 | Arc04 final Rust cohesion audit records scope, dependency, and the project-wide consistency emphasis. | `test -f 01-cleanup/arc04-rust-cohesion-audit/arc-plan.md` and `rg -n "consistency|cohesion|unified" 01-cleanup/arc04-rust-cohesion-audit/arc-plan.md` | correctness | operator-follow-up | done | Arc04 opened with whole-codebase consistency scope. | Arc04 is now active. |
| P-6 | Arc04 is active and Slice01 opens as a read-only cohesion diagnosis audit from Arc03 final feature state. | `test -f 01-cleanup/arc04-rust-cohesion-audit/slice01-cohesion-diagnosis-audit/cc-prompt.md` and `rg -n "diagnosis-only|read-only|d820065|RUST-007|cohesion" 01-cleanup/arc04-rust-cohesion-audit/arc-plan.md 01-cleanup/arc04-rust-cohesion-audit/slice01-cohesion-diagnosis-audit/*.md` | serious | operator-follow-up | done | Arc04 promoted to active and Slice01 open set created. | Slice01 is the current Arc04 entry point. |
| P-7 | Arc05 skipped-test repair arc is planned with a read-only Slice01 investigation open set before any ignored tests or tested code are changed. | `test -f 01-cleanup/arc05-skipped-test-repair/arc-plan.md` and `test -f 01-cleanup/arc05-skipped-test-repair/slice01-ignored-test-investigation/cc-prompt.md` and `rg -n "investigation-only|read-only|ignored tests|fix only the test|repair the code|rewrite the code" 01-cleanup/arc05-skipped-test-repair` | correctness | operator-follow-up | done | Arc05 arc plan and Slice01 open set created. | Later repair slices remain intentionally unopened until Slice01 classifies the ignored tests. |
| P-8 | Arc04 closes with an arc-level composition report and bubbles any residual cleanup need to the project roadmap. | `test -f 01-cleanup/arc04-rust-cohesion-audit/closing-report.md` and inspect the composition check and project bubble-up. | serious | project-management | done | `arc04-rust-cohesion-audit/closing-report.md` verifies all planned Arc04 slices, final gate reproduction, public API deferrals, and Arc05 ignored-test remediation handoff. | Arc05 is now active as the follow-on remediation arc. |

## Version History

### v2.26 - 2026-08-26

Opened Arc05 Slice04 for corpus addressability and policy. The project now has
a measurement-backed next step before CI wiring: add a checked-in fast
selection, named fast/full command surfaces, and explicit string/structural
JSON assertion policy while keeping workflows and the two corpus ignored tests
unchanged.

### v2.25 - 2026-08-26

Arc05 Slice03 CDC verification landed. CDC reproduced bounded, filtered, and
full 900-case corpus measurements, confirmed zero failures and zero JSON
equality disagreements, and verified that default tests, CI, and the remaining
corpus ignored tests were not changed. `Cargo.toml` changed only to declare the
explicit `corpus_measure` example target at `tools/corpus_measure.rs`. Arc05
should next design addressable corpus policy from the measured inventory before
selecting branch, PR, and post-merge gates.

### v2.24 - 2026-08-26

Arc05 Slice03 locally closed corpus measurement. The full 900-case IPC
parse/verify and JSON round-trip measurement completed with zero failures and
zero equality disagreements, while default tests, CI, and corpus `#[ignore]`
annotations remained unchanged. The Cargo manifest change is limited to the
explicit example-target declaration for the tool. The next Arc05 boundary
should use the measurement evidence to design addressable corpus policy before
deciding branch/PR/post-merge coverage.

### v2.23 - 2026-08-26

Arc05 Slice02 CDC verification landed at feature commit `e534df2`. The fast
ignored tests are repaired and enabled: `file_type_test` now asserts domain and
problem AST classification, while `forgotten_dash_validation_test` and
`forgotten_question_mark_validation_test` pin structured syntactic diagnostics
for malformed predicate declarations. The only remaining ignored Rust tests are
the IPC and JSON corpus tests, preserving Slice03 as the measurement-first
corpus path.

### v2.22 - 2026-08-26

Opened Arc05 Slice03 as a measurement-first corpus slice. The slice adds a
Rust-native example utility for deterministic IPC corpus inventory and
phase-level parse/verify plus JSON round-trip timing, while leaving corpus
`#[ignore]` annotations and CI policy unchanged. Operator direction captured for
later discussion: branch pushes may use a fast representative corpus sample,
while PRs and post-merge `main` should run the full corpus on both Linux and
macOS once the measurement data supports a concrete design.

### v2.21 - 2026-08-26

Opened Arc05 Slice02 for the three fast non-corpus ignored tests classified by
Slice01. Scope covers the `file_type_test` test-only repair and the
`forgotten_dash_validation_test` / `forgotten_question_mark_validation_test`
code-and-test repairs. The two IPC/JSON corpus tests remain outside Slice02
while the operator and CDC discuss a measurement-first slow/corpus strategy.

### v2.20 - 2026-08-26

Arc05 Slice01 locally closed the ignored-test investigation. The workbench
report inventories all five ignored Rust tests, records blame provenance,
captures explicit runtime probes, and recommends one test-only fix, two
code-and-test repairs, and two slow/corpus-gate follow-ups. No implementation
source, tests, fixtures, manifests, workflows, README, or ignore annotations
changed, and no downstream repair-slice open sets were created. CDC
verification is pending.

### v2.19 - 2026-08-26

Arc04 closed locally. The arc-level composition report verifies the full Rust
cohesion capability and confirms that residual ignored-test debt is now owned by
Arc05. Project status moves Arc05 Slice01 to the active follow-on stream.

### v2.18 - 2026-08-26

Arc04 Slice05 CDC verification landed. All planned Arc04 slices are now
CDC-verified, the final feature state passes the full locked local gate, and
Arc04 can proceed to arc-level closure. Public API/error/AST compatibility work
remains deferred behind explicit operator GO or a future public API contract
arc.

### v2.17 - 2026-08-26

Arc05 added as a follow-on remediation arc for inherited ignored tests. Slice01
opens as a read-only investigation that must classify each ignored test before
later repair slices decide whether to fix tests, update code and tests, or
rewrite code and tests together.

### v2.16 - 2026-08-26

Arc04 Slice05 locally closed the test helper/private naming cohesion repair.
The feature branch consolidates repeated flawed-domain integration-test result
handling, preserves fixture coverage and ignored-test status, renames the
private quantifier-elimination module and test-only spelling drift, and leaves
`ParsingError::Lexiacal` plus `Transformation::QuantifierElimintation`
unchanged behind the explicit public API gate. CDC verification is pending.

### v2.15 - 2026-08-26

Arc04 Slice04 CDC verification landed. The project now has the parser
byte-slice API repair and malformed problem-parser structured-error repair
verified against the full local gate, with public export narrowing still
operator-gated. Arc04 proceeds to Slice05.

### v2.14 - 2026-08-25

Arc04 Slice04 locally closed the parser API/error-boundary repair. The feature
branch changes borrowed parser/transpiler/LSP byte-input boundaries from
`&Vec<u8>` to `&[u8]`, preserves Vec-backed callers through characterization
coverage, returns a structured syntactic error for the malformed problem-parser
path, and leaves crate-root public export narrowing behind the explicit
operator-GO gate. CDC verification is pending.

### v2.13 - 2026-08-25

Arc04 Slice03 CDC verification landed. The project now has test-only baselines
for the public parser/transpiler API surface, parser panic boundary,
transformation error messages, public misspelled variants, and formula panic
contracts before Arc04 production repairs begin. Slice04 is next.

### v2.12 - 2026-08-25

Arc04 Slice02 CDC verification landed. The project plan now treats the Arc04 fix
map as verified and proceeds to Slice03 characterization baselines while keeping
public API/error/AST breaking changes gated by operator GO or future public API
contract work.

### v2.11 - 2026-08-25

Arc04 Slice02 locally closed the triage/fix map. The plan now sequences
Slice03 characterization baselines, Slice04 parser API/error-boundary repair,
and Slice05 test-helper/private-naming cohesion. Public API breaks for
COHESION-001 export narrowing, COHESION-003 error taxonomy, COHESION-004 formula
contracts, and COHESION-006 public variant spelling remain gated by operator GO
or deferred to a future public API/error/AST contract arc.

### v2.10 - 2026-08-26

Arc04 Slice02 opened as a planning-only triage/fix-map slice. The open set
requires dispositions for all six cohesion findings, public API compatibility
gates, baseline-test routing, and downstream slice open sets only where concrete.

### v2.9 - 2026-08-26

Arc04 Slice01 CDC verification landed. The cohesion audit found six findings,
recorded seven negative checks, kept the implementation diff workbench-only, and
made Slice02 triage/fix-map the next step.

### v2.8 - 2026-08-26

Arc04 promoted from placeholder to active planning. Slice01 opened as a
diagnosis-only, read-only Rust cohesion audit from Arc03 final feature state at
`d820065`, with non-overwriting workbench filenames and RUST-007 re-entry.

### v2.7 - 2026-08-26

Arc03 closed locally. All Arc03 slices have CDC verification; the final composed
feature state at `d820065` passes the workflow-equivalent gate; RUST-007 is
routed to Arc04 public API cohesion; and Arc04 is the next cleanup arc.

### v2.6 - 2026-08-26

Arc03 Slice08 CDC verification landed. RUST-006 Cargo reproducibility policy is
fixed with explicit dependency requirements and a tracked Cargo-generated
lockfile. Arc03 is ready for arc-level closure.

### v2.5 - 2026-08-26

Arc03 Slice07 CDC verification landed. RUST-004 diagnostic lock scope is fixed,
and Slice08 Cargo reproducibility policy is next.

### v2.4 - 2026-08-26

Arc03 Slice06 CDC verification landed. RUST-005/RUST-008 LSP error-boundary
and metadata repairs are fixed, and Slice07 LSP diagnostic lock scope is next.

### v2.3 - 2026-08-26

Arc03 Slice05 CDC verification landed. RUST-002/RUST-003 structured
parser/transform errors are fixed, and Slice06 LSP error boundaries and
metadata is next.

### v2.2 - 2026-08-25

Arc03 Slice04 CDC verification landed. RUST-001 CLI error exit codes are fixed,
and Slice05 structured parser/transform errors is the next repair slice.

### v2.1 - 2026-08-25

Arc03 Slice03 CDC verification landed. Arc03 now proceeds into focused repair
slices beginning with Slice04 CLI error exit codes.

### v2.0 - 2026-08-25

Arc03 Slice03 mapped all eight Rust best-practices findings. Arc03 now has
five focused repair slices and an explicit RUST-007 deferral to Arc04 public
API cohesion, preserving separate upstream PR groupings for behavior, LSP,
Cargo policy, and cohesion work.

### v1.9 - 2026-08-25

Arc03 Slice03 opened as the planning-only triage/fix-map step before
production Rust best-practice repairs begin.

### v1.8 - 2026-08-25

Arc03 Slice02 CDC verification landed. The project now has a test-only
current-behavior baseline before production Rust best-practice repairs begin;
Arc03 Slice03 is the next open step.

### v1.7 - 2026-08-25

Arc03 Slice01 CDC verification landed, including normalization of the audit
index top severity label to `Blocker`. Arc03 Slice02 opened as the dedicated
test-only characterization baseline before production fixes.

### v1.6 - 2026-08-25

Arc03 promoted from placeholder to active planning. Slice01 is explicitly
diagnosis-only/read-only, and Slice02 is reserved as a dedicated baseline
characterization-test slice before any production repair work begins. Project
ledger row P-5 also reconciled with the already-created Arc04 placeholder.

### v1.5 - 2026-08-25

Arc04 added as the final cleanup arc. The roadmap now separates best-practice
correctness repair from a final cohesion audit that checks whether Rust idioms
are chosen consistently across the whole project.

### v1.4 - 2026-08-25

Arc02 Slice01 CDC verification landed. Project status updated to show both
Arc02 slices are verified and only arc-level composition closure remains.

### v1.3 - 2026-08-25

Arc02 Slice02 CDC verification landed. Project status updated to show the Rust
2024 verification pass and strict compatibility CI gate are done, with Arc02
arc close still pending.

### v1.2 - 2026-08-25

Arc02 Slice02 open set created while Slice01 is underway, so the independent
verification/PR-hardening handoff is ready as soon as Slice01 close evidence
lands.

### v1.1 - 2026-08-25

Arc01 status updated to locally closed/PR-ready. Arc02 promoted from placeholder
to active, with Slice01 opened for the Rust 2024 mechanical migration.

### v1.0 - 2026-08-25

Initial project roadmap created on the orphan `planning` branch. Operator
overrode the framework default layout: planning artifacts live under
`.worktrees/planning/01-cleanup/` instead of `docs/design-v0.1.0/`.
