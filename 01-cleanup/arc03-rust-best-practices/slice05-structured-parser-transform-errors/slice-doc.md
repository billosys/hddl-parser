# Slice05: Structured Parser And Transform Errors

Version: 1.0
Date: 2026-08-25
Arc: `arc03-rust-best-practices`
Expected implementation branch: `fix/structured-parser-transform-errors`

## Goal

Fix RUST-002 and RUST-003 by replacing recoverable parser/transpiler/transform panics with structured errors on existing `Result` surfaces.

## Scope

In scope:

- `HDDLProgram::from_hddl` domain/problem kind mismatch handling.
- `Transpiler::from_hddl` behavior inherited from `HDDLProgram::from_hddl`.
- `RemoveEqualityConstraints` domain-only behavior.
- Minimal `ParsingError` extension or equivalent existing error-channel use.
- Updating the relevant Slice02 baselines in `tests/current_behavior.rs`.

Out of scope:

- CLI exit-code repair unless Slice04 left a specific bubble-up.
- LSP repairs.
- Cargo dependency/lockfile policy.
- Public API byte-slice or re-export cleanup.

## Expected Behavior

- Domain-as-problem and problem-as-domain inputs return `Err`, not panic.
- Domain-only `RemoveEqualityConstraints` returns `Err`, not panic.
- CLI invocations reaching these failures report errors without process panic once combined with Slice04 behavior.

## Verification Approach

Replace `catch_unwind` current-behavior assertions with ordinary `Err` assertions, rerun the full gate, and repeat the two panic runtime probes to confirm they no longer exit through panic code `101`.

## Exit Criteria

- RUST-002 and RUST-003 are fixed and covered by updated tests.
- Existing successful parse/transpile/transform tests still pass.
- No LSP, Cargo policy, or public API cleanup is mixed in.
