# CC Prompt: Slice04 Binary Smoke And Status

You are working in `/Users/oubiwann/lab/billosys/hddl-parser/.worktrees/features`
on branch `feature/add-ci`.

Add binary-level CI smoke checks and minimal status polish.

Implement only this slice:

- Add `cargo build --release --bins`.
- Add a smoke invocation for `./target/release/hddl_analyzer --help`.
- Decide whether a README CI badge belongs in this PR. If yes, keep it minimal
  and point it at the upstream workflow. If no, record the no-op rationale.

Do not add release publishing, artifact upload, installers, or language-server
integration tests.

Before closing, update `ledger.md` with evidence and write a
`closing-report.md` with a per-row walk plus a Bubble-up to the arc section.
