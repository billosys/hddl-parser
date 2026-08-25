# Arc03 Slice03 Closing Report: Triage And Fix Map

Date: 2026-08-25
Branch: `planning`
Implementation branch under review: `audit/rust-best-practices`

## Summary

Slice03 is complete as a planning-only triage slice. It created `fix-map.md`,
opened five focused Arc03 repair-slice open sets, updated the Arc03 arc plan,
and updated the project plan to record PR-family separation and the RUST-007
Arc04 deferral.

No Rust source, tests, manifests, workflows, README, dependency policy, or
feature-branch implementation files were changed.

## Final Finding Dispositions

| Finding | Disposition | Destination |
|---------|-------------|-------------|
| RUST-001 | fix in Arc03 | `slice04-cli-error-exit-codes`, PR group 1 |
| RUST-002 | fix in Arc03 | `slice05-structured-parser-transform-errors`, PR group 2 |
| RUST-003 | fix in Arc03 | `slice05-structured-parser-transform-errors`, PR group 2 |
| RUST-004 | fix in Arc03 | `slice07-lsp-diagnostic-lock-scope`, PR group 3b |
| RUST-005 | fix in Arc03 | `slice06-lsp-error-boundaries-and-metadata`, PR group 3a |
| RUST-006 | fix in Arc03 | `slice08-cargo-reproducibility-policy`, PR group 4 |
| RUST-007 | defer to Arc04 | Public API cohesion audit/fix pass |
| RUST-008 | fix in Arc03 | `slice06-lsp-error-boundaries-and-metadata`, PR group 3a |

No finding is marked no-op. No finding is deferred outside this cleanup project.

## Repair Slices Opened

- `slice04-cli-error-exit-codes`: isolates RUST-001 because the Slice02 CLI baselines can be flipped directly from current exit `0` failures to non-zero failures.
- `slice05-structured-parser-transform-errors`: groups RUST-002 and RUST-003 because both should use existing structured `Result<_, ParsingError>` paths instead of recoverable panics.
- `slice06-lsp-error-boundaries-and-metadata`: groups RUST-005 and RUST-008 because both are observable through the stdio LSP harness and change LSP request/response behavior.
- `slice07-lsp-diagnostic-lock-scope`: isolates RUST-004 because lock-scope proof is different from ordinary LSP protocol error-boundary testing.
- `slice08-cargo-reproducibility-policy`: isolates RUST-006 because it changes Cargo policy and generated lockfile state, not runtime behavior.

Each opened repair slice has only an open set: `slice-doc.md`, `ledger.md`, and
`cc-prompt.md`. No later repair-slice closing report or CDC verification file
was created.

## Deferrals

RUST-007 is deferred to Arc04. The reason is that the finding combines public
byte-slice input ergonomics with crate-root re-export design. The byte-slice
part is mechanically small, but explicit re-export narrowing is a public API
compatibility decision better handled during the whole-codebase cohesion pass.

Re-entry condition: after Arc03 behavior and Cargo policy repairs land, Arc04
should decide the intended public API surface, add compile/integration coverage
for byte-slice callers and intended public imports, then make the API changes
consistently if accepted.

LSP harness deferrals from Slice02 were not dropped:

- Non-file URI diagnostics, `didSave` filesystem failures, unreadable or
  missing sibling-domain files, and no-domain-found behavior are assigned to
  Slice06.
- Diagnostic `RwLock` contention is assigned to Slice07, with the runtime-test
  re-entry condition that the code either drops the read guard before awaited
  work or exposes a narrow test hook for controlled awaited work.

## Ledger Walk

- C3-1: Done. Slice03 changed only planning artifacts; the feature worktree still shows only the pre-existing untracked workbench audit files.
- C3-2: Done. `fix-map.md` covers RUST-001 through RUST-008.
- C3-3: Done. Every finding has exactly one disposition.
- C3-4: Done. Each Arc03 repair names a target slice and PR grouping.
- C3-5: Done. RUST-007 and LSP harness deferrals have destinations, rationale, and re-entry conditions.
- C3-6: Done. Slice02 baselines are mapped to the repairs that will update them.
- C3-7: Done. LSP harness limitations are carried forward to Slice06 and Slice07.
- C3-8: Done. `arc-plan.md` replaces the placeholder focused-fixes slice with concrete Slice04-Slice08 rows.
- C3-9: Done. Slice04-Slice08 each have complete open sets and no premature close-set artifacts.
- C3-10: Done. `project-plan.md` records the new Arc03 sequencing, PR grouping, and RUST-007 Arc04 responsibility.
- C3-11: Done. Final planning diff is whitespace-clean.
- C3-12: Done. This report bubbles up opened repair slices, sequencing changes, and deferrals.

## Bubble-Up

Arc03 now proceeds through Slice04-Slice08 instead of a placeholder
`slice04-plus-focused-fixes` bucket. Project-level PR grouping is:

1. CLI error exits.
2. Structured parser and transform errors.
3. LSP robustness, split into error-boundary/metadata and diagnostic lock-scope slices.
4. Cargo reproducibility policy.

Arc04 inherits RUST-007 as public API cohesion work after Arc03 repairs land.
