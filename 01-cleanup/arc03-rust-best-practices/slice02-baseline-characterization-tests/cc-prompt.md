# CC Prompt: Arc03 Slice02 Baseline Characterization Tests

You are working in HDDL-Parser on Arc03 Slice02:
`arc03-rust-best-practices/slice02-baseline-characterization-tests`.

Read these planning and audit files first:

- `/Users/oubiwann/lab/billosys/hddl-parser/.worktrees/planning/01-cleanup/project-plan.md`
- `/Users/oubiwann/lab/billosys/hddl-parser/.worktrees/planning/01-cleanup/arc03-rust-best-practices/arc-plan.md`
- `/Users/oubiwann/lab/billosys/hddl-parser/.worktrees/planning/01-cleanup/arc03-rust-best-practices/slice01-diagnosis-only-audit/cdc-verification.md`
- `/Users/oubiwann/lab/billosys/hddl-parser/.worktrees/planning/01-cleanup/arc03-rust-best-practices/slice02-baseline-characterization-tests/slice-doc.md`
- `/Users/oubiwann/lab/billosys/hddl-parser/.worktrees/planning/01-cleanup/arc03-rust-best-practices/slice02-baseline-characterization-tests/ledger.md`
- `workbench/2026.08.25-audit-index.md`
- `workbench/2026.08.25-audit-results-rust.md`

This slice is test-only. Add characterization tests that pass against today's
behavior before any production repairs are made.

Important boundary:

- Do not fix CLI exit codes yet.
- Do not replace panic paths with structured errors yet.
- Do not change dependency-version or lockfile policy yet.
- Do not widen public APIs yet.
- Do not make private internals public only for tests.
- Do not add new ignored tests.
- Keep any known-bad baseline tests clearly named or commented as
  `current_behavior` / `characterization`, so later repair slices can update
  them intentionally.

Suggested implementation shape:

- Prefer a new integration test file for CLI process behavior using
  `std::process::Command` and `env!("CARGO_BIN_EXE_hddl_analyzer")`.
- Characterize these CLI paths:
  - missing input exits `0` and writes an error to stderr;
  - unsupported extension exits `0` and writes an error to stderr;
  - semantic/parse failure exits `0` and writes an error to stderr;
  - output write failure exits `0` and writes an error to stderr;
  - known-good verification exits `0` and prints success.
- Add public API characterization for `HDDLProgram::from_hddl` and
  `Transpiler::from_hddl` domain/problem mismatches. Use `catch_unwind` or an
  equivalent panic-capturing pattern so the tests pass today while documenting
  that the behavior is not the desired future contract.
- Add transform characterization for domain-only
  `Transformation::RemoveEqualityConstraints` with the same current/undesired
  panic baseline.
- Add LSP characterization where feasible without production hooks. If
  `tower-lsp` does not expose a stable way to exercise initialize/diagnostic
  behavior without changing production code, record a precise deferral in the
  ledger and closing report instead of adding test-only public hooks.
- For the diagnostic `RwLock` finding, do not write a desired post-fix test
  that expects concurrent updates not to block unless it can pass today. This
  slice records current behavior; the repair slice will change the assertion.

Run and record:

```bash
cargo fmt --check
cargo check --all-targets
RUSTFLAGS="-D rust-2024-compatibility" cargo check --all-targets
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
cargo build --release --bins
./target/release/hddl_analyzer --help
actionlint .github/workflows/ci.yml
git diff --check
git diff --name-only
git diff -- tests src Cargo.toml | rg "#\\[ignore\\]|ignore ="
```

Also rerun the four runtime probes from
`workbench/2026.08.25-audit-results-rust.md` and confirm the baseline exit
codes remain `0`, `0`, `101`, `101`.

Closing report requirements:

- Walk every ledger row `C3-1` through `C3-12`; no silent drops.
- State exactly which tests were added and which audit findings they baseline.
- State any LSP harness limitations as explicit deferrals with re-entry
  conditions.
- Bubble up what Slice03 must account for when opening focused repair slices.
