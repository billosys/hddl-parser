# CC Prompt: Arc05 Slice01 Ignored Test Investigation

You are working in `/Users/oubiwann/lab/billosys/hddl-parser/.worktrees/features`.

Use branch `audit/ignored-tests`, based on the final Arc04 feature baseline if
Arc04 is already closed. If Arc04 is still in progress, confirm the operator's
chosen base before creating the branch.

Planning lives in:

`/Users/oubiwann/lab/billosys/hddl-parser/.worktrees/planning/01-cleanup/arc05-skipped-test-repair/slice01-ignored-test-investigation/`

Read first:

- `slice-doc.md`
- `ledger.md`
- `/Users/oubiwann/lab/billosys/hddl-parser/.worktrees/planning/01-cleanup/arc05-skipped-test-repair/arc-plan.md`

## Assignment

Run a read-only investigation of every ignored Rust test in the project. Do not
edit source, tests, manifests, workflows, README, fixtures, or `#[ignore]`
annotations. This slice is for evidence and classification only.

Create a non-overwriting workbench report, for example:

- `workbench/2026.08.26-ignored-test-investigation.md`

For each ignored test, record:

- File path and test function name.
- Ignore annotation text and Git provenance for the annotation.
- Explicit runtime probe result, using a bounded timeout for long-running tests
  if needed.
- Intended behavior and implementation code path under test.
- Observed failure, pass, panic, timeout, or runtime-cost class.
- Recommended follow-up route: test-only fix, code-and-test repair,
  code-and-test rewrite, slow/corpus gate, or valid deferral.
- Rationale for that route and suggested slice boundary.

Useful commands:

```bash
rg -n "#\\[ignore" src tests -g '*.rs'
cargo test --locked --all-targets
cargo test --locked -- --ignored
cargo test --locked <ignored-test-name> -- --ignored --exact
git blame -L <range> -- <file>
git diff --name-status
git diff --check
```

If a test is too slow or hangs, use a bounded timeout and record the exact
timeout command and result. Do not fix it in this slice.

## Close Requirements

When done, update `ledger.md` with attested evidence and add
`closing-report.md` with a row-by-row walk plus a Bubble-up to the arc section.

The Bubble-up to the arc must recommend whether subsequent slices should:

- Fix tests only.
- Repair code and tests together.
- Rewrite code and tests together.
- Introduce a slow/corpus-test policy and gate.
- Defer specific tests with concrete re-entry conditions.

Do not create downstream repair-slice open sets in this slice.
