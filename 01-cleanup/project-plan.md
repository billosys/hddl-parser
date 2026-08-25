# HDDL-Parser Cleanup Project Plan

Version: 1.0
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
| arc01-github-actions-ci | Adds the first GitHub Actions CI workflow and status-facing project polish. | active; fully planned | Warning-fix PR #5 merged or equivalent local baseline. |
| arc02-rust-2024-edition | Migrates the crate to the latest Rust edition using the standard edition workflow. | placeholder | arc01 gives CI coverage for the migration PR. |
| arc03-rust-best-practices | Audits and repairs Rust API, error handling, CLI, tests, and maintainability issues. | placeholder | arc01 gives CI coverage; arc02 defines the target edition. |

## Current Status

Arc01 is the active work and is fully broken down into open slices. Arc02 and
Arc03 are intentionally placeholders until Arc01 closes and the branch/PR state
settles upstream.

The warning-fix PR is treated as predecessor work, not part of this planning
packet.

## Project Ledger

Definition of done: the expected upstream cleanup sequence is planned and each
arc can be executed as a focused PR or PR series.

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
|----|-----------|--------|--------------|--------|--------|----------|-------|
| P-1 | Arc01 CI is closed and composed. | `test -f 01-cleanup/arc01-github-actions-ci/closing-report.md` and inspect row walk. | serious | project-plan | open | | |
| P-2 | Arc02 edition migration has a placeholder that records scope, dependency, and deferred breakdown. | `test -f 01-cleanup/arc02-rust-2024-edition/arc-plan.md` | correctness | project-plan | open | | |
| P-3 | Arc03 Rust best-practices audit has a placeholder that records scope, dependency, and deferred breakdown. | `test -f 01-cleanup/arc03-rust-best-practices/arc-plan.md` | correctness | project-plan | open | | |
| P-4 | Project boundaries keep CI, edition migration, and audit/fix work in separate upstream PRs. | `rg -n "fix/cargo-warnings|feature/add-ci|Rust 2024|best-practices|separate" 01-cleanup/project-plan.md 01-cleanup/arc*/arc-plan.md` | serious | issue-4 | open | | |

## Version History

### v1.0 - 2026-08-25

Initial project roadmap created on the orphan `planning` branch. Operator
overrode the framework default layout: planning artifacts live under
`.worktrees/planning/01-cleanup/` instead of `docs/design-v0.1.0/`.
