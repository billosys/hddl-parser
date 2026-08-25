# Slice04: CLI Error Exit Codes

Version: 1.0
Date: 2026-08-25
Arc: `arc03-rust-best-practices`
Expected implementation branch: `fix/cli-error-exit-codes`

## Goal

Fix RUST-001 by making recoverable `hddl_analyzer` CLI failures exit non-zero while preserving successful command behavior and stdout/stderr separation.

## Scope

In scope:

- `src/bin/hddl_analyzer/main.rs` exit-code plumbing.
- Minimal helper extraction if needed to avoid duplicated error/exit handling.
- Updating only the Slice02 CLI baselines in `tests/current_behavior.rs`.
- Full CI-equivalent verification after the behavior change.

Out of scope:

- Parser/transpiler panic repairs from RUST-002 and RUST-003.
- LSP repairs from RUST-004, RUST-005, and RUST-008.
- Dependency or lockfile policy from RUST-006.
- Public API cleanup from RUST-007.

## Expected Behavior

- Missing input exits non-zero and writes an error to stderr.
- Unsupported extension exits non-zero and writes an error to stderr.
- Parse or semantic verification failure exits non-zero and writes diagnostics to stderr.
- Output write failure exits non-zero and writes an error to stderr.
- Known-good verification exits `0` and prints success to stdout.

## Verification Approach

Run the updated focused tests, the full local workflow-equivalent gate, and the relevant runtime probes. The panic probes for RUST-002 and RUST-003 may still return `101` until Slice05.

## Exit Criteria

- RUST-001 is fixed and covered by updated tests.
- No unrelated Rust, LSP, Cargo, README, or workflow changes are mixed in.
- The full local workflow-equivalent gate passes.
