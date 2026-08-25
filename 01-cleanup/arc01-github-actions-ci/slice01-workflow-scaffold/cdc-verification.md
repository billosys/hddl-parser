# Slice01 CDC Verification: Workflow Scaffold

Date: 2026-08-25
Verifier: CDC / Sofie
Implementation worktree: `/Users/oubiwann/lab/billosys/hddl-parser/.worktrees/features`
Planning worktree: `/Users/oubiwann/lab/billosys/hddl-parser/.worktrees/planning`
Slice commit observed: `d469b70` (`ci: add GitHub Actions scaffold`)
Current worktree state: `feature/add-ci` has uncommitted `.github/workflows/ci.yml` edits from Slice02.

## Verdict

Verified. Slice01 satisfies its scaffold ledger. Current verification was run
against the in-flight workflow because Slice02 is already adding gates; the
Slice01 scaffold properties still hold after those edits.

Process note: the Slice01 ledger used `closed` as the row status. CDC normalized
those statuses to `done`, matching the ledger-discipline final-status
vocabulary. Evidence was not weakened by that terminology fix.

## Row Count

Opening ledger rows: 5
Closing report rows: 5
CDC verification rows: 5

No silent drops found.

## Row Walk

| ID | CDC result | Evidence |
|----|------------|----------|
| F-1 | done / reproduced | `test -f .github/workflows/ci.yml` exited 0 from `.worktrees/features`. |
| F-2 | done / reproduced | `rg -n "pull_request|push|main" .github/workflows/ci.yml` found `pull_request` at line 4, `push` at line 5, and `main` at line 7. |
| F-3 | done / reproduced | `rg -n "stable|rustfmt|clippy" .github/workflows/ci.yml` found stable Rust install/update/default and component setup at lines 41-44, plus clippy version and gate lines. |
| F-4 | done / reproduced | `rg -n "ubuntu|macos" .github/workflows/ci.yml` found explicit runners `ubuntu-24.04` at line 21 and `macos-15` at line 22. |
| F-5 | done / reproduced | `actionlint .github/workflows/ci.yml` exited 0. `actionlint` is available at `/opt/homebrew/bin/actionlint`. |

## Scope And Drift Check

Committed Slice01 at `d469b70` added only `.github/workflows/ci.yml`.

The current uncommitted workflow diff is Slice02 in progress and includes:

- runner labels updated from `ubuntu-latest` / `macos-latest` to
  `ubuntu-24.04` / `macos-15`;
- checkout updated from `actions/checkout@v4` to `actions/checkout@v7`;
- Rust setup split into plain `rustup` install/update/default/component steps;
- format, check, clippy, and test gates added.

Those extra gates are outside Slice01, but they do not invalidate the Slice01
scaffold criteria.

## Bubble-Up Check

Arc01 row A-1 may be treated as verified. Slice01 provides a usable workflow
scaffold for Slice02 quality gates, Slice03 tests, and Slice04 binary smoke.

No arc-plan change is required.
