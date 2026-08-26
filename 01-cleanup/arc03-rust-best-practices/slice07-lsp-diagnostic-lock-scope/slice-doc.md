# Slice07: LSP Diagnostic Lock Scope

Version: 1.1
Date: 2026-08-26
Arc: `arc03-rust-best-practices`
Expected implementation branch: `fix/lsp-diagnostic-lock-scope`
Expected base: `fix/lsp-error-boundaries-and-metadata` at `fbb27d7`, unless
Slice06 has already been merged upstream.

## Goal

Fix RUST-004 by ensuring the language-server diagnostic path does not hold the document-map `RwLock` read guard across awaited work.

## Scope

In scope:

- `src/language_server/request_handler.rs` diagnostic document lookup and lock scope.
- Minimal test or source-level regression evidence proving the read guard is dropped before later awaits.
- Keeping existing LSP behavior tests green.

Out of scope:

- URI/filesystem error-boundary repairs from RUST-005 unless Slice06 explicitly left a dependency.
- Metadata version repair from RUST-008.
- CLI/parser/transform/Cargo/public API cleanup.

## Expected Behavior

The diagnostic handler should clone or otherwise own the document bytes while holding the read guard, drop the guard, and only then perform awaited logging, filesystem scans, file reads, or diagnostic generation.

## Verification Approach

Prefer a deterministic runtime test if it can be written without public API widening or timing fragility. If not, use source-level evidence that the guard is scoped before awaited work and document the runtime-test re-entry condition.

## Exit Criteria

- No document-map read guard is held across an `.await` in the diagnostic path.
- Existing LSP behavior tests pass.
- Any missing deterministic contention test has a concrete re-entry condition.
- The full local workflow-equivalent gate passes.

## Version History

### v1.1 - 2026-08-26

Refreshed the open set after Slice06 CDC verification. Slice07 should start
from `fbb27d7` or a merged equivalent so the lock-scope-only diff is checked
against the settled LSP error-boundary and metadata repairs.

### v1.0 - 2026-08-25

Initial open set from Slice03 fix mapping.
