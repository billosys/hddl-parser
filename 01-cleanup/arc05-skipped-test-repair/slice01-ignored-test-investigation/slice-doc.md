# Slice01: Ignored Test Investigation

Version: 1.0
Date: 2026-08-26
Arc: `arc05-skipped-test-repair`
Expected branch: `audit/ignored-tests`

## Goal

Investigate every ignored Rust test in HDDL-Parser and record enough evidence
to decide how later slices should repair the skipped coverage.

This slice is investigation-only and read-only. It does not fix tests, rewrite
code, change ignore annotations, or introduce policy. Its job is to replace
uncertainty with a concrete route map for follow-up slices.

## Scope

In scope:

- Inventory every `#[ignore]` annotation under `src/` and `tests/`.
- Record each ignored test's name, file path, ignore reason, and Git provenance.
- Run each ignored test explicitly, one at a time where practical, and record
  pass/fail/panic/timeout/runtime behavior.
- Identify the code path or behavior each ignored test appears intended to
  cover.
- Classify each ignored test into one recommended route: fix only the test,
  repair code and test together, rewrite code and test together, move behind a
  slow/corpus gate, or defer with a concrete re-entry condition.
- Produce a workbench investigation report and update planning close evidence.

Out of scope:

- Editing source or test code.
- Removing, adding, or changing `#[ignore]` annotations.
- Refactoring parser, semantic analyzer, transpiler, JSON, IPC, or LSP code.
- Adding CI slow-test gates.
- Creating downstream repair-slice open sets before the investigation findings
  are complete.

## Verification Approach

Run discovery and read-only behavior probes:

```bash
rg -n "#\\[ignore" src tests -g '*.rs'
cargo test --locked --all-targets
cargo test --locked -- --ignored
cargo test --locked <ignored-test-name> -- --ignored --exact
git blame -L <range> -- <file>
git diff --name-status
git diff --check
```

If an ignored test is too slow or hangs, stop it with a bounded timeout and
record the timeout command, duration, and partial output. Do not rewrite the
test in this slice.

## Exit Criteria

- Every ignored Rust test is inventoried with file, name, reason, and Git
  provenance.
- Every ignored Rust test has an explicit runtime probe result or a documented
  reason why it could not be safely run.
- Every ignored Rust test has an intended-behavior/code-path note.
- Every ignored Rust test has a recommended follow-up route for later slices.
- The implementation worktree has no source/test/manifest/workflow/README or
  fixture edits from this slice.
