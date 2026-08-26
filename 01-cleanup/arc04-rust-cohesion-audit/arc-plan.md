# Arc04: Rust Cohesion Audit And Fixes

Version: 1.10
Date: 2026-08-25
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
Arc03 issue. This slice opens the repair slices required by the map. Status:
CDC-verified. The map opens Slice03, Slice04, and Slice05, and holds public API
breaking changes behind operator GO or later-arc deferral.

### slice03-characterization-baselines

Add test-only characterization coverage for current public imports, Vec-backed
input APIs, malformed problem-parser panic behavior, current error variants,
formula normalization panic contracts, and public misspelled variants. Status:
CDC-verified. The slice added a single integration test file and changed no
production behavior.

### slice04-parser-api-and-error-boundary

Repair parser byte-input cohesion by changing borrowed inputs from `&Vec<u8>` to
`&[u8]` where no Vec behavior is needed, while preserving Vec-backed callers.
Also convert the malformed problem-parser panic path to a structured syntactic
error after Slice03 baselines exist. Status: CDC-verified. Public crate-root
export narrowing was skipped and remains gated behind operator GO or future
public API/error/AST contract work.

### slice05-test-helper-and-private-naming-cohesion

Consolidate duplicated test assertion helpers and repair private/test-only
naming drift after the behavior baselines and parser repair are settled. Status:
CDC-verified. Public enum variant spelling repairs remain out of scope unless
the operator explicitly approves a public API compatibility plan.

### Public API Gates And Deferrals

Root export narrowing, public error taxonomy redesign, public formula API
contract changes, and public enum spelling repairs are not approved by Slice02.
They are deferred to a future public API/error/AST contract arc unless the
operator gives explicit GO after the Slice03 characterization baseline.

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
| A4-2 | Slice02 maps every Slice01 finding to a focused repair slice, accepted variation, later-arc deferral, or duplicate/no-op. | `test -f 01-cleanup/arc04-rust-cohesion-audit/slice02-triage-and-fix-map/fix-map.md` and inspect mapping. | serious | project-management | done | `slice02-triage-and-fix-map/cdc-verification.md` verifies all six cohesion findings are dispositioned, public API gates are explicit, Slice03-Slice05 open sets are complete, and no downstream close artifacts were created. | Slice03 characterization baselines are next. |
| A4-3 | Accepted Rust idiom variations are explicitly documented with local reasons instead of silently drifting. | `rg -n "accepted variation|intentional divergence|local reason" 01-cleanup/arc04-rust-cohesion-audit` | correctness | operator-follow-up | done | `slice02-triage-and-fix-map/fix-map.md` documents accepted variations, local rationale, and later-arc public API deferrals; Slice05 CDC verification confirms public misspelled variants remain deliberately deferred rather than silently renamed. | Remaining public API/error/AST work is gated by operator GO or a future public API contract arc. |
| A4-4 | Every production repair slice opened by Slice02 closes with CDC verification and a focused behavior-preservation story. | `find 01-cleanup/arc04-rust-cohesion-audit -path '*/closing-report.md' -print` and inspect repair rows. | serious | collaboration-framework | done | `slice04-parser-api-and-error-boundary/cdc-verification.md` verifies the parser byte-slice API and malformed problem-parser syntactic-error repair with full local gate reproduction. `slice05-test-helper-and-private-naming-cohesion/cdc-verification.md` verifies the test helper and private/test naming cleanup with no public API changes. | Slice03 characterization baseline plus Slice04/Slice05 repairs are CDC-verified; Arc04 is ready for arc-level closure. |
| A4-5 | Final Arc04 feature state passes the full local gate, including locked Cargo verification. | `cargo fmt --check && cargo check --locked --all-targets && RUSTFLAGS="-D rust-2024-compatibility" cargo check --locked --all-targets && cargo clippy --locked --all-targets -- -D warnings && cargo test --locked --all-targets && cargo build --locked --release --bins && ./target/release/hddl_analyzer --help && actionlint .github/workflows/ci.yml && git diff --check` | serious | arc01-arc03 | done | CDC reproduced the full local gate on `fix/test-private-naming-cohesion` at `7e2d8a7`, including locked check, Rust 2024 compatibility, Clippy `-D warnings`, full tests, release build, binary help smoke, actionlint, and whitespace checks. | Full tests preserve the inherited ignored-test pattern: 1 library ignored, 2 flawed ignored, 1 IPC ignored, and 1 JSON ignored. |
| A4-6 | Arc04 close report bubbles up whether HDDL-Parser cleanup can close or needs another remediation arc. | `test -f 01-cleanup/arc04-rust-cohesion-audit/closing-report.md` and inspect final decision. | serious | project-management | done | `closing-report.md` closes A4-1 through A4-6, confirms all planned Arc04 slices compose into the cohesion capability, and bubbles inherited ignored-test debt to Arc05. | Arc04 is closed locally. |

## Version History

### v1.10 - 2026-08-26

Arc04 closed locally. The arc-level close report verifies that Slice01 through
Slice05 compose into the Rust cohesion capability, records the full locked local
gate passing at `7e2d8a7`, and bubbles inherited ignored-test debt into Arc05
rather than treating it as silent residual work.

### v1.9 - 2026-08-26

Slice05 CDC verification landed. The test helper/private naming cohesion repair
is accepted with public API spellings unchanged and explicitly deferred. All
planned Arc04 repair slices are now CDC-verified, the final Arc04 feature state
passes the full locked local gate, and Arc04 is ready for arc-level closure.

### v1.8 - 2026-08-26

Slice05 locally closed the test helper/private naming cohesion repair. The
flawed-domain integration-test assertion shell is consolidated without
weakening variant or field checks, scoped private/test-only spelling drift is
repaired, and public misspelled enum variants remain deferred behind explicit
operator GO or future public API/error/AST contract work. CDC verification is
pending.

### v1.7 - 2026-08-26

Slice04 CDC verification landed. The parser/transpiler/LSP byte-input repair
and malformed problem-parser structured-error repair are accepted, Vec-backed
caller compatibility remains covered, and public crate-root export narrowing
remains gated. Slice05 is next.

### v1.6 - 2026-08-25

Slice04 locally closed the parser API/error-boundary repair. Borrowed byte-input
APIs now accept `&[u8]`, Vec-backed callers remain covered, the malformed
problem-parser panic path returns `ParsingError::Syntactic`, and public
crate-root export narrowing remains skipped/gated pending explicit operator GO
or future public API/error/AST contract work. CDC verification is pending.

### v1.5 - 2026-08-25

Slice03 CDC verification landed. The characterization baseline is test-only and
pins public imports, Vec-backed parser/transpiler inputs, malformed problem
panic behavior, transformation error messages, public misspelled variants, and
formula panic contracts. Slice04 is now ready to start.

### v1.4 - 2026-08-25

Slice02 CDC verification landed. The fix map is now accepted as the Arc04 repair
sequence: Slice03 characterization baselines, Slice04 parser API/error-boundary
repair, and Slice05 test-helper/private-naming cohesion, with public API breaks
still gated by operator GO or future public API/error/AST contract work.

### v1.3 - 2026-08-25

Slice02 locally closed the triage/fix map and replaced the placeholder repair
entry with concrete downstream slices: Slice03 characterization baselines,
Slice04 parser API/error-boundary repair, and Slice05 test-helper/private-naming
cohesion. Public API breaking changes remain behind operator GO or later-arc
deferral.

### v1.2 - 2026-08-26

Slice02 opened as a planning-only triage/fix-map slice. The open set requires
all six Slice01 cohesion findings to receive explicit dispositions, public API
compatibility gates, baseline-test routing, downstream slice open sets where
concrete, and no implementation changes.

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
