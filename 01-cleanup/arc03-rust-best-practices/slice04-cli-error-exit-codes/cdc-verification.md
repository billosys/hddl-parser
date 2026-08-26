# CDC Verification: Arc03 Slice04 CLI Error Exit Codes

Date: 2026-08-25
Verifier: CDC
Feature branch: `fix/cli-error-exit-codes`
Feature base: `b596714 Add baseline characterization tests`
Feature commit verified: `f3a3f8d Fix CLI error exit codes`
Planning branch: `planning`
Planning commit reviewed: `c160856 Close Arc03 Slice04 CLI exit codes`

## Verdict

Verified with scope reconciliation. Slice04 fixes RUST-001: recoverable
`hddl_analyzer` CLI failures now exit non-zero while successful verification
continues to exit `0`.

All C4-1 through C4-9 rows are verified. C4-1 required reconciliation because
the feature commit includes `.gitignore` in addition to the Rust CLI and test
files. The operator confirmed that `workbench` artifacts are not tracked in
their repositories and explicitly asked CDC to include the `.gitignore` commit.
CDC accepts that line as repository hygiene; the Rust behavior change remains
limited to CLI exit plumbing and directly updated CLI baselines.

## Artifact Boundary

`git diff --name-only b596714..HEAD`:

```text
.gitignore
src/bin/hddl_analyzer/main.rs
tests/current_behavior.rs
```

The `.gitignore` diff only adds `workbench`. No LSP, parser, transpiler,
manifest, workflow, README, or corpus fixture files changed in the feature
commit.

## Runtime Probes

| Probe | Expected | Observed |
|-------|----------|----------|
| Missing input | non-zero with stderr error | exit `1`, `[Error] No such file or directory (os error 2)` |
| Unsupported extension | non-zero with stderr error | exit `1`, `[Error] unrecognized input extension '.toml' (expected .hddl or .json)` |
| Semantic verification failure | non-zero with stderr diagnostics | exit `1`, `[Error] line 53: subtask undefined_task is not defined.` |
| Output write failure | non-zero with stderr error | exit `1`, `[Error] Is a directory (os error 21)` |
| Known-good verification | exit `0` with success on stdout | exit `0`, `[Ok]` |
| RUST-002 public parser panic baseline | unchanged for Slice05 | exit `101`, panic at `src/lib.rs:37:18` |
| RUST-003 transform panic baseline | unchanged for Slice05 | exit `101`, panic at `src/transpiler/transformations/remove_equality.rs:17:9` |

## Quality Gate

CDC reproduced or inspected the Slice04 gate:

- `cargo fmt --check`
- `cargo check --all-targets`
- `RUSTFLAGS="-D rust-2024-compatibility" cargo check --all-targets`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test --test current_behavior`
- `cargo test --all-targets`
- `cargo build --release --bins`
- `./target/release/hddl_analyzer --help`
- `actionlint .github/workflows/ci.yml`
- `git diff --check`
- `git diff --check b596714..HEAD`

All commands exited `0`.

## Ledger Walk

- C4-1: Verified with operator-approved scope reconciliation. Feature diff is
  `.gitignore`, `src/bin/hddl_analyzer/main.rs`, and
  `tests/current_behavior.rs`; `.gitignore` only adds `workbench`.
- C4-2: Verified. Missing input exits `1` and preserves stderr error output.
- C4-3: Verified. Unsupported extension exits `1` and preserves stderr error
  output.
- C4-4: Verified. Semantic verification failure exits `1` and reports
  diagnostics on stderr.
- C4-5: Verified. Output write failure exits `1` and preserves stderr error
  output.
- C4-6: Verified. Known-good verification still exits `0` and writes success
  to stdout.
- C4-7: Verified. RUST-002 and RUST-003 panic baselines still exit `101`;
  Slice05 owns those repairs.
- C4-8: Verified. Full local workflow-equivalent gate passed.
- C4-9: Verified. Closing report walks every row and states the final RUST-001
  CLI process contract.

## Bubble-Up

Slice05 can proceed. The CLI wrapper now correctly maps ordinary `Result`
errors to process failure, so parser/transpiler and transform panic repairs in
Slice05 should flow through the Slice04 error path once they are converted to
structured errors.
