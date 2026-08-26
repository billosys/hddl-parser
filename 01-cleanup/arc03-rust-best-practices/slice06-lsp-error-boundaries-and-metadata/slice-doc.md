# Slice06: LSP Error Boundaries And Metadata

Version: 1.1
Date: 2026-08-26
Arc: `arc03-rust-best-practices`
Expected implementation branch: `fix/lsp-error-boundaries-and-metadata`
Expected base: `fix/structured-parser-transform-errors` at `6bd1b0a`, unless
Slice05 has already been merged upstream.

## Goal

Fix RUST-005 and RUST-008 by making reachable LSP request failures non-panicking and by reporting the package version in `initialize`.

## Scope

In scope:

- `src/language_server/request_handler.rs` ordinary URI/filesystem/path error handling.
- Updating `ServerInfo.version` to use package metadata.
- Extending or updating `tests/lsp_current_behavior.rs` through the existing stdio harness.
- Logging recoverable filesystem/domain-discovery failures where appropriate.

Out of scope:

- Diagnostic `RwLock` lock-scope repair from RUST-004.
- CLI/parser/transform repairs from RUST-001 through RUST-003.
- Cargo reproducibility policy.
- Public API cohesion cleanup.

## Expected Behavior

- LSP `initialize` reports `env!("CARGO_PKG_VERSION")`.
- Non-file or malformed diagnostic requests do not panic the server.
- `didSave` missing-file paths do not panic the server.
- Missing/unreadable sibling-domain discovery does not panic the server.
- No-domain-found problem diagnostics are either preserved as empty reports or changed with an explicit documented rationale.

## Verification Approach

Use `tests/lsp_current_behavior.rs` where the stdio harness can exercise behavior deterministically. If an edge case cannot be made deterministic without a larger refactor, record that limit with a concrete re-entry condition in the closing report.

## Exit Criteria

- RUST-005 and RUST-008 are fixed or any remaining LSP edge case has an explicit deferral and re-entry condition.
- The LSP harness tests pass and no server panic is observed for covered failure paths.
- The full local workflow-equivalent gate passes.

## Version History

### v1.1 - 2026-08-26

Refreshed the open set after Slice05 CDC verification. Slice06 should start
from `6bd1b0a` or a merged equivalent so LSP-only scope can be checked against
the settled CLI and structured-error repairs.

### v1.0 - 2026-08-25

Initial open set from Slice03 fix mapping.
