# CDC Verification: Arc03 Slice02 Baseline Characterization Tests

Date: 2026-08-25
Verifier: CDC
Feature branch: `audit/rust-best-practices`
Feature commit verified: `b596714 Add baseline characterization tests`
Planning branch: `planning`

## Verdict

Verified. Slice02 closes with 12/12 ledger rows reproduced or reconciled.

The implementation is a test-only baseline against the current Rust
best-practices audit findings. It adds characterization tests without
production source, manifest, workflow, lockfile, or dependency-policy changes.

One report detail was reconciled during CDC: CC described the two new test
files as intent-to-add and uncommitted, but the feature worktree has them
committed at `b596714`. The verified slice diff is therefore `main..HEAD`, not
the clean working-tree diff.

## Artifact Boundary

Reproduced:

- `git -C .worktrees/features diff --name-only main..HEAD`
- `git -C .worktrees/features diff --stat main..HEAD`
- `git -C .worktrees/features diff -- Cargo.toml .gitignore Cargo.lock`

Observed slice implementation files:

- `tests/current_behavior.rs`
- `tests/lsp_current_behavior.rs`

`main..HEAD` contains exactly those two test files, with 376 inserted lines.
The manifest, lockfile, ignore file, workflows, README, and production `src/`
tree are unchanged by this slice.

The feature worktree remains otherwise dirty only with the pre-existing
untracked Slice01 workbench audit files:

- `workbench/2026.08.25-audit-index.md`
- `workbench/2026.08.25-audit-results-rust.md`

## Ledger Row Walk

| ID | CDC Status | Evidence |
|----|------------|----------|
| C3-1 | verified | `git diff --name-only main..HEAD` lists only `tests/current_behavior.rs` and `tests/lsp_current_behavior.rs`; no production or policy files changed. |
| C3-2 | verified | `tests/current_behavior.rs` covers CLI missing input, unsupported extension, semantic failure, output write failure, and known-good verification; full tests pass. |
| C3-3 | verified | `tests/current_behavior.rs` uses `catch_unwind` to characterize `HDDLProgram::from_hddl` and `Transpiler::from_hddl` domain/problem mismatch panics. |
| C3-4 | verified | `remove_equality_constraints_domain_only_current_behavior_panics` captures the current domain-only transform panic. |
| C3-5 | verified | `tests/lsp_current_behavior.rs` initializes the language server and asserts current stale version `0.1.0` differs from `CARGO_PKG_VERSION`. |
| C3-6 | verified | LSP tests cover unsynced diagnostic JSON-RPC `-32602` and problem-without-domain empty full report; deferred brittle paths are documented. |
| C3-7 | verified | No desired post-fix `RwLock` contention assertion was added; the closing report records deterministic re-entry conditions. |
| C3-8 | verified | `git diff -- Cargo.toml .gitignore Cargo.lock` produced no output; no dependency or lockfile policy change is mixed in. |
| C3-9 | verified | `git diff main..HEAD -- tests src Cargo.toml | rg "#\\[ignore\\]|ignore ="` produced no matches; no new ignored tests were introduced. |
| C3-10 | verified | Full workflow-equivalent quality gate reproduced locally. |
| C3-11 | verified | Four runtime probes still return the pre-fix baseline exit codes `0`, `0`, `101`, `101`. |
| C3-12 | verified | Closing report records test inventory, LSP harness limits, row walk, runtime probes, and Slice03 bubble-up. |

## Reproduced Quality Gate

All commands passed on 2026-08-25:

- `cargo fmt --check`
- `cargo check --all-targets`
- `RUSTFLAGS="-D rust-2024-compatibility" cargo check --all-targets`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test --all-targets`
- `cargo build --release --bins`
- `./target/release/hddl_analyzer --help`
- `actionlint .github/workflows/ci.yml`
- `git diff --check`

`cargo test --all-targets` included the 13 new tests:

- 10 tests from `tests/current_behavior.rs`
- 3 tests from `tests/lsp_current_behavior.rs`

Existing ignored tests remain pre-existing corpus or legacy ignores; CDC found
no newly introduced ignored tests in the Slice02 diff.

## Runtime Probe Reproduction

The audit probes still match the current-behavior baseline:

| Probe | Exit |
|-------|------|
| Missing input: `hddl_analyzer verify /tmp/definitely-missing-hddl-parser-input.hddl` | `0` |
| Unsupported extension: `hddl_analyzer verify Cargo.toml` | `0` |
| Problem file as domain: `hddl_analyzer verify tests/ipc/Blocksworld-GTOHP/p01.hddl` | `101` |
| Domain-only remove-equality transform: `hddl_analyzer convert tests/ipc/Blocksworld-GTOHP/domain.hddl --to json --transform remove-equality-constraints` | `101` |

The two panic probes reproduce the expected current panic locations:

- `src/lib.rs:37:18`, `expected domain, found problem`
- `src/transpiler/transformations/remove_equality.rs:17:9`,
  `Compiling inequiality requires a problem instance`

## Bubble-Up

Slice03 can now open the triage and fix map with a real baseline underneath it.
Recommended grouping:

- Fix RUST-001 CLI recoverable error exit codes and update the CLI
  characterization assertions from current `0` exits to the desired non-zero
  exits.
- Convert RUST-002 and RUST-003 panic paths into structured errors, then
  replace `catch_unwind` baselines with normal `Result` assertions.
- Repair LSP metadata and diagnostic error boundaries with deterministic tests
  added alongside the implementation.
- Keep RUST-006 dependency and lockfile policy separate unless a specific
  behavior repair requires manifest changes.
- Carry the documented LSP `RwLock` contention and panic-oriented harness
  limits into Slice03 as explicit fix-map decisions, not silent deferrals.

## What Worked

The slice established a clean test safety net without changing implementation
behavior. The new tests name undesirable behavior as current behavior, which
gives later repair slices a precise before/after line to cross.
