# CC Prompt: Slice04 Clippy Remediation And Binary Smoke

You are working in `/Users/oubiwann/lab/billosys/hddl-parser/.worktrees/features`
on branch `feature/add-ci`.

Fix the strict Clippy failures uncovered by Slice02, then add binary-level CI
smoke checks and minimal status polish.

Implement only this slice:

- Fix every issue reported by `cargo clippy --all-targets -- -D warnings`.
- Keep fixes mechanical and behavior-preserving: shorthand fields, remove
  needless `return`, replace length comparisons with `is_empty`, use `if let`
  or `matches!` where Clippy asks, clean test assertions, and similar local
  rewrites.
- Do not add `#[allow(...)]`, relax `-D warnings`, remove `--all-targets`, skip
  tests, or weaken the CI workflow to make Clippy pass.
- After Clippy is green, rerun `cargo test --all-targets`.
- Add `cargo build --release --bins`.
- Add a smoke invocation for `./target/release/hddl_analyzer --help`.
- Decide whether a README CI badge belongs in this PR. If yes, keep it minimal
  and point it at the upstream workflow. If no, record the no-op rationale.

Do not add release publishing, artifact upload, installers, or language-server
integration tests. Do not do edition migration or broad Rust best-practices
refactoring; this slice is the minimum cleanup needed for the CI PR to go green
with strict Clippy.

Required local verification:

```sh
cargo fmt --check
cargo check --all-targets
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
cargo build --release --bins
./target/release/hddl_analyzer --help
```

Before closing, update `ledger.md` with evidence and write a
`closing-report.md` with a per-row walk plus a Bubble-up to the arc section.
