# Arc03 Slice03 Fix Map

Date: 2026-08-25
Arc: `arc03-rust-best-practices`
Input evidence: Slice01 audit, Slice02 characterization tests, and CDC verification for both slices.

## Summary

Arc03 should land as a small PR series rather than one broad best-practices patch. The high-risk behavior repairs have current-behavior tests from Slice02, so they can be changed intentionally. LSP work is split because error-boundary behavior and lock-scope contention need different verification strategies. Cargo reproducibility is separate because it changes policy and generated dependency state, not runtime behavior.

## Repair Slices Opened

| Slice | Findings | Proposed PR grouping | Why this split |
|-------|----------|----------------------|----------------|
| `slice04-cli-error-exit-codes` | RUST-001 | PR group 1: CLI error exits | User-facing process behavior, already covered by direct subprocess baselines. |
| `slice05-structured-parser-transform-errors` | RUST-002, RUST-003 | PR group 2: structured parser and transform errors | Both panic paths should flow through the existing `Result<_, ParsingError>` surfaces. |
| `slice06-lsp-error-boundaries-and-metadata` | RUST-005, RUST-008 | PR group 3a: LSP error boundaries and metadata | These are request/response behavior changes in the language server and share the stdio LSP harness. |
| `slice07-lsp-diagnostic-lock-scope` | RUST-004 | PR group 3b: LSP diagnostic lock scope | Concurrency/lock-scope repair needs different proof from ordinary LSP error handling. |
| `slice08-cargo-reproducibility-policy` | RUST-006 | PR group 4: Cargo reproducibility policy | Manifest and lockfile policy should be reviewable without behavior repairs mixed in. |

RUST-007 is deferred to Arc04 because it is a public API cohesion decision, not just a local signature cleanup.

## Finding Map

| Finding | Severity | Category | Disposition | Target slice or deferral | Rationale | Expected behavior change | Slice02 baseline impact | Proposed upstream PR grouping |
|---------|----------|----------|-------------|--------------------------|-----------|--------------------------|-------------------------|-------------------------------|
| RUST-001 | High | Correctness / error handling | fix in Arc03 | `slice04-cli-error-exit-codes` | Recoverable CLI failures currently print an error and exit `0`, which breaks shell/CI/editor automation. The subprocess baselines make this safe to change first. | Missing input, unsupported extension, parse/semantic failure, and output write failure return non-zero exits while successful verification remains `0`; stdout/stderr separation is preserved. | Update `tests/current_behavior.rs` CLI tests from current exit `0` on failures to desired non-zero exits; keep the success test at `0`. | PR group 1: CLI error exits. |
| RUST-002 | High | Correctness / soundness | fix in Arc03 | `slice05-structured-parser-transform-errors` | Domain/problem kind mismatches are external-input errors on public `Result`-returning APIs, not programmer invariants. | `HDDLProgram::from_hddl` and `Transpiler::from_hddl` return structured `ParsingError` values for domain-as-problem and problem-as-domain inputs instead of panicking. CLI paths should report the errors through the exit handling from Slice04. | Replace the four `catch_unwind` current-behavior tests in `tests/current_behavior.rs` with ordinary `Err` assertions. Runtime probe for problem-as-domain should stop returning panic exit `101`. | PR group 2: structured parser and transform errors. |
| RUST-003 | High | Correctness / transformation error handling | fix in Arc03 | `slice05-structured-parser-transform-errors` | `remove-equality-constraints` already sits behind a `Result`-returning transform path, so missing problem input should use that channel. | Domain-only `RemoveEqualityConstraints` returns a structured transformation/parsing error without mutating state or panicking. CLI convert reports the error with a non-zero exit after Slice04. | Replace `remove_equality_constraints_domain_only_current_behavior_panics` in `tests/current_behavior.rs` with an `Err` assertion. Runtime probe for domain-only remove-equality should stop returning panic exit `101`. | PR group 2: structured parser and transform errors. |
| RUST-004 | Medium | Concurrency / runtime safety | fix in Arc03 | `slice07-lsp-diagnostic-lock-scope` | Diagnostics currently borrow document bytes out of the document map while awaiting logging and filesystem work. The repair is small, but deterministic contention proof should be handled separately from LSP error semantics. | Diagnostic handling owns or clones the document bytes while the read guard is held, drops the guard before any `.await`, and then performs logging/filesystem/diagnostic work outside the lock. | No Slice02 test currently asserts desired contention behavior. Keep `tests/lsp_current_behavior.rs` green and add deterministic lock-scope proof in Slice07 if feasible; otherwise record source-level proof with exact `rg` evidence and re-entry. | PR group 3b: LSP diagnostic lock scope. |
| RUST-005 | Medium | Error handling / runtime safety | fix in Arc03 | `slice06-lsp-error-boundaries-and-metadata` | URI conversion, parent lookup, file reads, directory reads, sibling reads, and path rendering are ordinary LSP/runtime failures and should not panic the server. | Malformed requests return JSON-RPC `invalid_params`; filesystem/sibling-domain discovery failures return an empty diagnostic report where appropriate and log context instead of panicking. No-domain-found behavior should remain deterministic and documented. | Extend/update `tests/lsp_current_behavior.rs`: keep unsynced diagnostic error coverage, preserve or update no-domain-found behavior intentionally, and add non-file URI, `didSave`, unreadable/missing sibling-file tests where the stdio harness can make them deterministic. | PR group 3a: LSP error boundaries and metadata. |
| RUST-006 | Medium | Cargo / project structure | fix in Arc03 | `slice08-cargo-reproducibility-policy` | The package ships binaries, but wildcard dependency requirements and ignored lockfiles make CI/user dependency resolution drift over time. This is a policy change, not a behavior repair. | Dependency requirements become explicit compatible semver requirements and `Cargo.lock` is tracked or a documented project-specific exception is added. | No behavior baseline is needed. Verify with `git ls-files Cargo.lock`, manifest/ignore diff inspection, and the full CI-equivalent gate. | PR group 4: Cargo reproducibility policy. |
| RUST-007 | Low | API / invariants | defer to Arc04 | Arc04 Rust cohesion audit | The finding combines two cohesion questions: accepting `&[u8]` instead of `&Vec<u8>` and replacing broad crate-root glob re-exports with explicit public API design. The byte-slice change is local, but export narrowing can become a public compatibility decision. | No Arc03 behavior change. Arc04 should decide the intended public API surface, then make byte-slice and re-export changes consistently if accepted. | No Slice02 behavior baseline is updated in Arc03. Arc04 re-entry should add compile/integration coverage proving byte-slice callers work and intended public imports remain available. | Deferred outside the Arc03 PR series; candidate Arc04 cohesion PR. |
| RUST-008 | Low | Metadata / packaging | fix in Arc03 | `slice06-lsp-error-boundaries-and-metadata` | The stale LSP version is a small language-server metadata repair already reachable through the Slice02 stdio harness. | LSP `initialize` reports `env!("CARGO_PKG_VERSION")`, currently `0.2.0`, instead of hard-coded `0.1.0`. | Update `lsp_initialize_current_behavior_reports_stale_server_version` in `tests/lsp_current_behavior.rs` to assert equality with `CARGO_PKG_VERSION`. | PR group 3a: LSP error boundaries and metadata. |

## LSP Harness Carry-Forward

Slice02 proved the stdio LSP harness can reach initialize, unsynced diagnostics, and no-domain-found problem diagnostics. It deliberately did not add brittle post-fix tests for every panic candidate.

The following cases must remain visible:

- `RwLock` contention: handled by `slice07-lsp-diagnostic-lock-scope`. Re-entry condition for a deterministic runtime test: the diagnostic path no longer holds a document-map read guard across awaited work, or the implementation exposes a narrow test hook that lets the test block awaited work under control.
- Non-file URI diagnostics: handled by `slice06-lsp-error-boundaries-and-metadata`. Re-entry condition: assert JSON-RPC `invalid_params` or the chosen non-panicking diagnostic response through the stdio harness.
- `didSave` with missing/unreadable file: handled by `slice06-lsp-error-boundaries-and-metadata`. Re-entry condition: assert the server stays alive and returns/logs a recoverable error without process panic.
- Unreadable or missing sibling domain files: handled by `slice06-lsp-error-boundaries-and-metadata`. Re-entry condition: assert empty diagnostics or an explicit JSON-RPC error according to the request classification chosen in that slice.
- No-domain-found behavior: already covered by `tests/lsp_current_behavior.rs`; Slice06 must either preserve the empty full report intentionally or update the baseline with a documented new contract.

## Sequencing

1. `slice04-cli-error-exit-codes`: repair the user-facing process contract first.
2. `slice05-structured-parser-transform-errors`: remove panics from parser/transpiler/transform `Result` paths, using the CLI exit behavior from Slice04.
3. `slice06-lsp-error-boundaries-and-metadata`: make LSP protocol/runtime failures non-panicking and fix stale metadata.
4. `slice07-lsp-diagnostic-lock-scope`: reduce diagnostic lock scope and add deterministic proof where feasible.
5. `slice08-cargo-reproducibility-policy`: pin dependency requirements and settle lockfile tracking policy.
6. Arc04: revisit RUST-007 as part of a whole-codebase public API/cohesion pass.
