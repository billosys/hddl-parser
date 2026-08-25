# Arc04: Rust Cohesion Audit

Version: 0.1 placeholder
Date: 2026-08-25
Expected branch: `audit/rust-cohesion` or smaller focused branches

## Capability

Perform the final whole-codebase Rust cohesion pass after the best-practices
audit/fix work has settled. This arc checks not only whether individual Rust
choices are technically correct, but whether the project uses a coherent set of
idioms everywhere those choices apply.

The desired end state is a codebase that feels intentionally engineered from a
unified front: similar problems use similar error shapes, traversal patterns,
ownership conventions, parser/test structure, module boundaries, naming, and
helper abstractions unless there is an explicit local reason to diverge.

## Relationship To Arc03

Arc03 remains the first Rust quality arc: it finds and fixes correctness,
soundness, API, error-handling, testing, performance, and maintainability
issues using the collaboration-framework code-audit discipline and the
Rust-guidelines substrate.

Arc04 is deliberately later and narrower in a different direction. It assumes
Arc03 has already handled the obvious defects and then asks a project-level
cohesion question: are the idioms that remain consistently chosen across the
entire codebase?

## Audit Emphasis

Use the collaboration-framework `CODE-AUDIT.md` shape, augmented with an
explicit consistency lens:

- Compare like-with-like across modules, not just each function in isolation.
- Treat inconsistent but individually-valid choices as findings when they make
  the project harder to understand, extend, or review.
- Prefer documenting a single project convention, or applying it consistently,
  over locally clever one-off rewrites.
- Distinguish intentional divergence from accidental drift. Intentional
  divergence needs a named reason in code, tests, or the audit report.
- Preserve upstream contribution friendliness: split findings/fixes into small
  PRs when one consistency theme touches too many files.

Concrete consistency areas to audit:

- Error propagation and recoverability boundaries: `Result`, panics,
  `unwrap`/`expect`, and diagnostics.
- Parser control flow: token lookahead, router/delegation shape, recursive
  parsing patterns, and EOF/error handling.
- AST and domain model ownership: borrowed vs owned values, clone patterns,
  boxed formulas, and traversal APIs.
- Iterator and collection idioms: `into_iter`, `iter`, `iter_mut`, `map`,
  `filter_map`, accumulation, and early-return style.
- Module organization and visibility: public exports, private helpers, test
  modules, naming, and file layout.
- Test style: assertion precision, fixture helpers, round-trip tests,
  integration-test structure, ignored tests, and naming.
- CLI/library boundary: stdout/stderr behavior, exit codes, library errors
  versus app aggregation, and binary smoke coverage.
- Concurrency/runtime style in the language server: lock ownership, await
  boundaries, document-map access, and async helper boundaries.

## Deferred Breakdown

Detailed slice planning is intentionally deferred until Arc03 closes. Expected
work:

- Run a diagnosis-only cohesion audit and write reports under `workbench/` using
  the `CODE-AUDIT.md` report shape, with an added section for project-wide Rust
  pattern consistency.
- Group findings by consistency theme, not merely by file, so systemic drift is
  visible.
- Decide which themes become upstream PRs and which are documented as accepted
  local variation.
- Apply only one cohesive pattern family per fix slice/PR unless the operator
  approves a broader cleanup.
- Re-run the Arc01/Arc02 quality gates after each fix slice.

## Placeholder Ledger

Definition of done: the final cleanup arc has an explicit whole-codebase Rust
cohesion lens that survives into the later audit and fix work.

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
|----|-----------|--------|--------------|--------|--------|----------|-------|
| A4-1 | Arc04 records that cohesion means consistent Rust idiom choices across the entire codebase, not only local best-practice compliance. | `rg -n "whole-codebase|cohesion|consistently chosen|unified front" 01-cleanup/arc04-rust-cohesion-audit/arc-plan.md` | serious | operator-follow-up | open | | |
| A4-2 | Arc04 remains deferred until Arc03 fixes settle, so consistency is audited against the repaired codebase. | `rg -n "deferred until Arc03 closes|Arc03 has already handled" 01-cleanup/arc04-rust-cohesion-audit/arc-plan.md` | correctness | project-management | open | | |
| A4-3 | Arc04 uses the collaboration-framework code-audit shape with an explicit consistency augmentation. | `rg -n "CODE-AUDIT.md|consistency lens|Compare like-with-like" 01-cleanup/arc04-rust-cohesion-audit/arc-plan.md` | serious | collaboration-framework | open | | |
| A4-4 | Arc04 preserves upstream-friendly scope by splitting broad consistency themes into focused PRs. | `rg -n "small|focused|PRs|one cohesive pattern family" 01-cleanup/arc04-rust-cohesion-audit/arc-plan.md` | serious | issue-4 | open | | |

## Version History

### v0.1 - 2026-08-25

Placeholder opened as the fourth and final cleanup arc. Operator emphasis added:
audit the project-wide consistency of Rust idiom choices, so correct local code
also coheres into a deliberately unified codebase.
