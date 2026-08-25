# HDDL-Parser Cleanup Project Plan

Version: 1.4
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
| arc03-rust-best-practices | Audits and repairs Rust API, error handling, CLI, tests, and maintainability issues. | placeholder | arc01 gives CI coverage; arc02 defines the target edition. |

## Current Status

Arc01 is closed locally with a workflow-equivalent verification pass and a
recorded caveat that the CI PR should be stacked on the warning-fix baseline or
rebased after PR #5 lands. Arc02 is active: both slices have CDC verification,
including the strict Rust 2024 compatibility CI gate. Arc02 still needs
arc-level composition closure before the project moves to Arc03. Arc03 remains a
placeholder until the edition target is settled.

The warning-fix PR is treated as predecessor work, not part of this planning
packet.

## Project Ledger

Definition of done: the expected upstream cleanup sequence is planned and each
arc can be executed as a focused PR or PR series.

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
|----|-----------|--------|--------------|--------|--------|----------|-------|
| P-1 | Arc01 CI is closed and composed. | `test -f 01-cleanup/arc01-github-actions-ci/closing-report.md` and inspect row walk. | serious | project-plan | done | `01-cleanup/arc01-github-actions-ci/closing-report.md` records all Arc01 rows done and the full local workflow-equivalent command passing. | CI PR readiness depends on upstream PR #5 base/merge sequencing. |
| P-2 | Arc02 edition migration has an active arc plan and Slice01/Slice02 open sets. | `test -f 01-cleanup/arc02-rust-2024-edition/arc-plan.md` and `test -f 01-cleanup/arc02-rust-2024-edition/slice01-edition-migration/cc-prompt.md` and `test -f 01-cleanup/arc02-rust-2024-edition/slice02-edition-verification-and-pr/cc-prompt.md` | correctness | project-plan | done | Arc02 plan promoted from placeholder; Slice01 and Slice02 open sets created. | Slice02 executes after Slice01 close evidence exists. |
| P-3 | Arc03 Rust best-practices audit has a placeholder that records scope, dependency, and deferred breakdown. | `test -f 01-cleanup/arc03-rust-best-practices/arc-plan.md` | correctness | project-plan | open | | |
| P-4 | Project boundaries keep CI, edition migration, and audit/fix work in separate upstream PRs. | `rg -n "fix/cargo-warnings|feature/add-ci|edition/rust-2024|best-practices|separate" 01-cleanup/project-plan.md 01-cleanup/arc*/arc-plan.md` | serious | issue-4 | open | | |

## Version History

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
