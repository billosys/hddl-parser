# Arc04: Rust Cohesion Audit And Fixes

Version: 1.1
Date: 2026-08-26
Expected audit branch: `audit/rust-cohesion`
Expected repair branches: smaller `fix/...` branches as Slice02 requires

## Capability

Perform the final whole-codebase Rust cohesion pass after the best-practices
audit/fix work has settled. This arc checks not only whether individual Rust
choices are technically correct, but whether the project uses a coherent set of
idioms everywhere those choices apply.

The desired end state is a codebase that feels intentionally engineered from a
unified front: similar problems use similar error shapes, traversal patterns,
ownership conventions, parser/test structure, module boundaries, naming, and
helper abstractions unless there is an explicit local reason to diverge.

Arc04 is not a broad rewrite permission slip. It is a final cleanup arc that
turns an already-improved codebase into a more cohesive upstream contribution
set. Every production change after the diagnosis pass must be justified by a
specific cohesion finding, a behavior-preservation baseline, and a focused
review boundary.

## Relationship To Arc03

Arc03 handled the concrete Rust best-practices findings discovered by the first
whole-codebase audit. It fixed CLI error exits, structured parser/transform
panic paths, LSP error boundaries and metadata, diagnostic lock scope, and Cargo
reproducibility policy. Its final local feature commit is `d820065 Fix Cargo
reproducibility policy`.

Arc04 starts from that settled Arc03 local feature state. It should not reopen
Arc03 repairs unless the cohesion audit finds either a new concrete defect or a
cross-cutting consistency theme that makes the repaired code harder to maintain.

Arc03 explicitly routed RUST-007 here: public parser API cohesion. Arc04 must
re-enter that topic and decide whether it needs a focused repair slice, an
accepted-variation note, or a later API-design arc.

## Slice Breakdown

### slice01-cohesion-diagnosis-audit

Run a diagnosis-only, read-only cohesion audit across the Arc03-repaired Rust
codebase. Produce non-overwriting workbench reports using filenames that include
`cohesion`, identify cohesion findings by theme, record accepted variations, and
capture RUST-007 disposition options. No production, test, manifest, workflow,
README, or behavior changes are allowed in this slice. Status: CDC-verified;
delivered six cohesion findings, seven negative checks, and concrete Slice02
handoff items.

### slice02-triage-and-fix-map

Turn Slice01 findings into a repair map. Each finding must be assigned to one of
these outcomes: focused repair slice, no-op accepted variation, later-arc
deferral with a concrete re-entry condition, or duplicate of an already-fixed
Arc03 issue. This slice opens the repair slices required by the map.

### slice03-plus-focused-cohesion-repairs

Placeholder for the focused repair slices opened by Slice02. Each repair slice
must handle one cohesive pattern family and must first add or identify any
missing behavior-preservation tests needed for the change.

## Dependencies

- Arc01 local CI gate and workflow-equivalent command set.
- Arc02 Rust 2024 edition migration and strict compatibility gate.
- Arc03 final local feature state at `d820065`.
- Arc03 RUST-007 public parser API cohesion deferral.
- Arc03 Slice07 C7-5 runtime contention coverage deferral as a visibility item,
  not an automatic Arc04 implementation requirement.

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
- Reuse the dominant project convention when it is sound. Where the dominant
  convention is weak, propose a single replacement convention and explain why it
  should become the project norm.

Concrete consistency areas to audit:

- Error propagation and recoverability boundaries: `Result`, panics,
  `unwrap`/`expect`, and diagnostics.
- Parser control flow: token lookahead, router/delegation shape, recursive
  parsing patterns, parse-error construction, and EOF/error handling.
- AST and domain model ownership: borrowed vs owned values, clone patterns,
  boxed formulas, and traversal APIs.
- Iterator and collection idioms: `into_iter`, `iter`, `iter_mut`, `map`,
  `filter_map`, accumulation, allocation, and early-return style.
- Module organization and visibility: public exports, private helpers, test
  modules, naming, and file layout.
- Test style: assertion precision, fixture helpers, round-trip tests,
  integration-test structure, ignored tests, and naming.
- CLI/library boundary: stdout/stderr behavior, exit codes, library errors
  versus app aggregation, and binary smoke coverage.
- Concurrency/runtime style in the language server: lock ownership, await
  boundaries, document-map access, and async helper boundaries.
- Public API cohesion: whether top-level parser/transform entry points expose a
  coherent shape for callers, especially after Arc03 structured-error repairs.

## Operating Rules

- Slice01 is read-only. It creates only workbench audit reports and planning
  close evidence.
- Do not overwrite Arc03 workbench files. Use filenames such as
  `workbench/<DATE>-cohesion-audit-index.md` and
  `workbench/<DATE>-cohesion-audit-results-rust.md`.
- Before any production repair, either identify existing tests that preserve the
  behavior being changed or open a test-only characterization slice first.
- Do not add `#[allow(...)]` suppressions as a substitute for a cohesion repair.
- Do not change public behavior for cosmetic consistency alone.
- Keep upstream review units small: one pattern family per repair slice unless
  the operator explicitly approves a broader cleanup.

## Arc Ledger

Definition of done: the final cleanup arc audits and, where justified, repairs
whole-codebase Rust cohesion so similar problems use similar idioms with clear
intentional variation.

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
|----|-----------|--------|--------------|--------|--------|----------|-------|
| A4-1 | Slice01 runs as a diagnosis-only, read-only cohesion audit from Arc03 final feature state. | `test -f 01-cleanup/arc04-rust-cohesion-audit/slice01-cohesion-diagnosis-audit/closing-report.md` and inspect row walk. | serious | collaboration-framework | done | `slice01-cohesion-diagnosis-audit/cdc-verification.md` verifies 12/12 rows, workbench-only implementation diff, six cohesion findings, seven negative checks, RUST-007 re-entry, and full locked quality gate reproduction. | Slice02 is the next planning-only fix-map slice. |
| A4-2 | Slice02 maps every Slice01 finding to a focused repair slice, accepted variation, later-arc deferral, or duplicate/no-op. | `test -f 01-cleanup/arc04-rust-cohesion-audit/slice02-triage-and-fix-map/fix-map.md` and inspect mapping. | serious | project-management | open | | |
| A4-3 | Accepted Rust idiom variations are explicitly documented with local reasons instead of silently drifting. | `rg -n "accepted variation|intentional divergence|local reason" 01-cleanup/arc04-rust-cohesion-audit` | correctness | operator-follow-up | open | | |
| A4-4 | Every production repair slice opened by Slice02 closes with CDC verification and a focused behavior-preservation story. | `find 01-cleanup/arc04-rust-cohesion-audit -path '*/closing-report.md' -print` and inspect repair rows. | serious | collaboration-framework | open | | |
| A4-5 | Final Arc04 feature state passes the full local gate, including locked Cargo verification. | `cargo fmt --check && cargo check --locked --all-targets && RUSTFLAGS="-D rust-2024-compatibility" cargo check --locked --all-targets && cargo clippy --locked --all-targets -- -D warnings && cargo test --locked --all-targets && cargo build --locked --release --bins && ./target/release/hddl_analyzer --help && actionlint .github/workflows/ci.yml && git diff --check` | serious | arc01-arc03 | open | | |
| A4-6 | Arc04 close report bubbles up whether HDDL-Parser cleanup can close or needs another remediation arc. | `test -f 01-cleanup/arc04-rust-cohesion-audit/closing-report.md` and inspect final decision. | serious | project-management | open | | |

## Version History

### v1.1 - 2026-08-26

Slice01 CDC verification landed. The cohesion diagnosis audit produced six
findings, seven negative checks, and concrete Slice02 handoff items while
preserving the read-only implementation boundary.

### v1.0 - 2026-08-26

Promoted Arc04 from placeholder to active arc after Arc03 local close. Slice01
opened as a read-only cohesion diagnosis audit against Arc03 final feature state
`d820065`, preserving the operator's emphasis on consistent Rust idiom choices
across the whole codebase.

### v0.1 - 2026-08-25

Placeholder opened as the fourth and final cleanup arc. Operator emphasis added:
audit the project-wide consistency of Rust idiom choices, so correct local code
also coheres into a deliberately unified codebase.
