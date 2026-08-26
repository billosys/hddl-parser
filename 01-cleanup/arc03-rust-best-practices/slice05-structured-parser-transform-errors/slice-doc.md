# Slice05: Structured Parser And Transform Errors

Version: 1.1
Date: 2026-08-25
Arc: `arc03-rust-best-practices`
Expected implementation branch: `fix/structured-parser-transform-errors`
Expected base: `fix/cli-error-exit-codes` at `f3a3f8d`, unless Slice04 has
already been merged upstream.

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
- When based on Slice04, CLI invocations reaching these failures should exit
  non-zero through the ordinary `[Error]` path, not through panic exit `101`.

## Verification Approach

Replace `catch_unwind` current-behavior assertions with ordinary `Err` assertions, rerun the full gate, and repeat the two panic runtime probes to confirm they no longer exit through panic code `101`.

## Exit Criteria

- RUST-002 and RUST-003 are fixed and covered by updated tests.
- Existing successful parse/transpile/transform tests still pass.
- No LSP, Cargo policy, or public API cleanup is mixed in.

## Version History

### v1.1 - 2026-08-25

Rebased the open set on the now-verified Slice04 contract. Slice05 should start
from `f3a3f8d` or a merged equivalent so structured parser/transform errors can
be verified through the non-zero CLI error path.

### v1.0 - 2026-08-25

Initial open set from Slice03 fix mapping.
