# HDDL-Parser Cleanup Project Plan

Version: 1.9
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
| arc03-rust-best-practices | Audits Rust API, error handling, CLI, tests, and maintainability issues; baselines behavior with tests before production repairs. | active; Slice03 open for triage/fix-map | arc01 gives CI coverage; arc02 defines the target edition. |
| arc04-rust-cohesion-audit | Performs the final whole-codebase consistency pass so Rust idioms, error shapes, data-flow patterns, and module conventions feel deliberately unified. | placeholder opened | arc03 fixes land first so cohesion is audited against the settled codebase. |

## Current Status

Arc01 is closed locally with a workflow-equivalent verification pass and a
recorded caveat that the CI PR should be stacked on the warning-fix baseline or
rebased after PR #5 lands. Arc02 is active: both slices have CDC verification,
including the strict Rust 2024 compatibility CI gate. Arc02 still needs
arc-level composition closure and upstream base reconciliation before the Rust
2024 PR should be opened.

Arc03 is active. Slice01 is CDC-verified as a diagnosis-only audit with only
workbench audit artifacts in the implementation worktree. Slice02 is
CDC-verified with test-only baseline characterization coverage before any
production repair slice begins. Slice03 is open to triage the audit findings
and baseline evidence into focused fix slices or explicit deferrals. Arc04
remains the final placeholder: a whole-codebase cohesion pass that runs after
Arc03 so consistency is judged against the repaired codebase.

The warning-fix PR is treated as predecessor work, not part of this planning
packet.

## Project Ledger

Definition of done: the expected upstream cleanup sequence is planned and each
arc can be executed as a focused PR or PR series.

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
|----|-----------|--------|--------------|--------|--------|----------|-------|
| P-1 | Arc01 CI is closed and composed. | `test -f 01-cleanup/arc01-github-actions-ci/closing-report.md` and inspect row walk. | serious | project-plan | done | `01-cleanup/arc01-github-actions-ci/closing-report.md` records all Arc01 rows done and the full local workflow-equivalent command passing. | CI PR readiness depends on upstream PR #5 base/merge sequencing. |
| P-2 | Arc02 edition migration has an active arc plan and Slice01/Slice02 open sets. | `test -f 01-cleanup/arc02-rust-2024-edition/arc-plan.md` and `test -f 01-cleanup/arc02-rust-2024-edition/slice01-edition-migration/cc-prompt.md` and `test -f 01-cleanup/arc02-rust-2024-edition/slice02-edition-verification-and-pr/cc-prompt.md` | correctness | project-plan | done | Arc02 plan promoted from placeholder; Slice01 and Slice02 open sets created. | Slice02 executes after Slice01 close evidence exists. |
| P-3 | Arc03 Rust best-practices audit/fix arc is active, with Slice01 opened as read-only and Slice02 reserved for pre-repair characterization tests. | `test -f 01-cleanup/arc03-rust-best-practices/arc-plan.md` and `test -f 01-cleanup/arc03-rust-best-practices/slice01-diagnosis-only-audit/cc-prompt.md` and `rg -n "diagnosis-only|read-only|baseline characterization|before production" 01-cleanup/arc03-rust-best-practices/arc-plan.md 01-cleanup/arc03-rust-best-practices/slice01-diagnosis-only-audit/*.md` | correctness | project-plan | done | Arc03 plan v1.0 created and Slice01 open set added. | Slice02 open set is now open after Slice01 CDC verification. |
| P-4 | Project boundaries keep CI, edition migration, best-practices, and cohesion work in separate upstream PRs or PR families. | `rg -n "fix/cargo-warnings|feature/add-ci|edition/rust-2024|best-practices|cohesion|separate" 01-cleanup/project-plan.md 01-cleanup/arc*/arc-plan.md` | serious | issue-4 | open | | |
| P-5 | Arc04 final Rust cohesion audit has a placeholder that records scope, dependency, and the project-wide consistency emphasis. | `test -f 01-cleanup/arc04-rust-cohesion-audit/arc-plan.md` and `rg -n "consistency|cohesion|unified" 01-cleanup/arc04-rust-cohesion-audit/arc-plan.md` | correctness | operator-follow-up | done | Arc04 placeholder opened with whole-codebase consistency scope. | Arc04 remains deferred until Arc03 repairs land. |

## Version History

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
