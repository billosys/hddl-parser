# CC Prompt: Slice03 Test Matrix

You are working in `/Users/oubiwann/lab/billosys/hddl-parser/.worktrees/features`
on branch `feature/add-ci`.

Add test execution to CI.

Implement only this slice:

- Add `cargo test --all-targets`, or an explicitly equivalent command set if
  the crate needs named integration tests to be split.
- Preserve coverage for the existing `ipc`, `flawed`, and `json` integration
  tests.
- Do not mark tests ignored and do not remove corpus coverage to make CI green.

If full IPC execution proves too slow for every matrix cell, stop and document a
specific split proposal in the closing report instead of weakening the gate
silently.

Before closing, update `ledger.md` with evidence and write a
`closing-report.md` with a per-row walk plus a Bubble-up to the arc section.
