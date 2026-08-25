# CC Prompt: Slice01 Workflow Scaffold

You are working in `/Users/oubiwann/lab/billosys/hddl-parser/.worktrees/features`
on branch `feature/add-ci`.

Add the first GitHub Actions workflow scaffold for HDDL-Parser.

Read before editing:

- `01-cleanup/project-plan.md` from the planning worktree.
- `01-cleanup/arc01-github-actions-ci/arc-plan.md`.
- This slice's `slice-doc.md` and `ledger.md`.

Implement only this slice:

- Create `.github/workflows/ci.yml`.
- Trigger on `pull_request` and `push` to `main`.
- Use a small matrix covering Linux and macOS unless you discover a concrete
  reason to defer one runner.
- Install stable Rust with `rustfmt` and `clippy` components so later slices can
  add gates without reworking setup.
- Add caching if it is conventional and does not obscure the workflow.

Do not migrate editions, fix audit findings, or add release automation.

Before closing, update `ledger.md` with evidence and write a
`closing-report.md` with a per-row walk plus a Bubble-up to the arc section.
