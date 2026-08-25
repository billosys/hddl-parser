# CC Prompt: Slice02 Rust Quality Gates

You are working in `/Users/oubiwann/lab/billosys/hddl-parser/.worktrees/features`
on branch `feature/add-ci`.

Add Rust quality gates to the GitHub Actions workflow from Slice01.

Implement only this slice:

- Add `cargo fmt --check`.
- Add `cargo check --all-targets`.
- Add `cargo clippy --all-targets -- -D warnings`.
- Keep command text simple and easy for maintainers to recognize.

Run the local verify command from the ledger. If Clippy or check fails because
the warning-fix prerequisite has not landed, do not weaken CI. Record the
failure and the exact re-entry condition.

Before closing, update `ledger.md` with evidence and write a
`closing-report.md` with a per-row walk plus a Bubble-up to the arc section.
