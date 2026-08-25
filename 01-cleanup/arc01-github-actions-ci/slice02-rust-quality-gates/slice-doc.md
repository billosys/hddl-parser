# Slice02: Rust Quality Gates

Date: 2026-08-25
Branch: `feature/add-ci`
Arc: `arc01-github-actions-ci`

## Goal

Add the standard Rust quality gates to CI: formatting, checking, and Clippy with
warnings treated as failures.

## In Scope

- `cargo fmt --check`
- `cargo check --all-targets`
- `cargo clippy --all-targets -- -D warnings`
- Any small workflow changes required to run these gates reliably

## Out Of Scope

- Broad Clippy-driven code rewrites
- Edition migration
- Dependency policy changes

## Verification Approach

Run the same commands locally from the implementation worktree. If a gate fails
because the warning-fix PR has not landed, record the failure and rebase/retry
after the prerequisite merges rather than weakening the gate.

## Exit Criteria

- CI contains the three quality gates.
- The gates pass locally on the warning-fix baseline.
- Any failure is either fixed in the warning PR baseline or documented with a
  concrete re-entry condition.
