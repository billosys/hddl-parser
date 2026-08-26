# Arc04 Slice02 Fix Map

Date: 2026-08-25
Planning branch: planning
Implementation branch under review: audit/rust-cohesion
Input audit: `workbench/2026.08.25-cohesion-audit-results-rust.md`

## Triage Decision

The Slice01 grouping hypothesis is mostly accepted with two adjustments:

- A pure test-only baseline cannot prove the desired future `&[u8]` caller
  behavior because those calls do not compile until production signatures
  change. Slice03 should characterize the current public surface and current
  panic/error behavior; Slice04 should add the new slice-caller tests as part
  of the parser API repair.
- Public API breakage is not approved by this map. Root export narrowing,
  public error enum changes, public formula contract changes, and public enum
  spelling repairs require operator GO or a later public API design arc.

## Primary Finding Map

| Finding | Severity | Theme | Disposition | Target slice or deferral destination | Rationale | Public API compatibility risk | Operator GO required before implementation | Expected behavior or API change | Existing baseline or characterization action | Proposed upstream PR grouping |
|---------|----------|-------|-------------|--------------------------------------|-----------|-------------------------------|--------------------------------------------|---------------------------------|--------------------------------------------|-------------------------------|
| COHESION-001 / RUST-007 | Medium | Public API cohesion | Focused repair slice with operator-gated export policy | `slice04-parser-api-and-error-boundary`; root export narrowing is gated for operator GO or later public API deferral | Borrowed byte inputs are a compatibility-improving Rust API repair; crate-root glob export narrowing may remove public names and needs policy approval. | Low for changing `&Vec<u8>` arguments to `&[u8]` at function call sites; medium/high for narrowing root exports because downstream imports may break. | Yes for removing or narrowing public re-exports; no for accepting `&[u8]` where `&Vec<u8>` currently works through deref coercion. | Parser/transpiler/LSP helpers accept byte slices; export surface is either explicitly narrowed after GO or documented as accepted variation/no-op. | Existing tests cover Vec-backed calls. `slice03-characterization-baselines` pins current public imports and Vec behavior; Slice04 adds passing `&[u8]` caller tests during repair. | Parser API cohesion PR; export narrowing, if approved, should be a separate public API PR or a clearly called-out section of the same PR. |
| COHESION-002 | Medium | Parser error boundary cohesion | Focused repair slice | `slice04-parser-api-and-error-boundary` | The malformed problem parser panic is a recoverable input error boundary, and a parser helper can make error construction more consistent. | Low/medium behavior change: malformed input that used to panic should return `ParsingError::Syntactic`. Public type shape can remain stable. | No, unless the repair changes public error enum shape beyond returning the existing syntactic variant. | Problem parser returns a structured syntactic error instead of panicking on unexpected top-level tokens. | `slice03-characterization-baselines` pins the current panic with `catch_unwind`; Slice04 changes that expectation to non-panic `Err`. | Parser API cohesion PR with the borrowed-input repair. |
| COHESION-003 | Medium | Error API cohesion | Later-arc deferral with baseline retained | Destination: future public API/error taxonomy arc; re-entry before any 1.0-style API stabilization, crate publication, or operator-approved breaking cleanup | Typed transformation errors and `Lexiacal` spelling affect public error matching. Arc04 can preserve evidence, but should not redesign the error API without an explicit public compatibility decision. | High: replacing `Transformation(String)` or renaming `ParsingError::Lexiacal` changes downstream pattern matches. | Yes. | No Arc04 production change by default. Future change would add typed transformation/classification errors and/or rename public variants with a compatibility plan. | `slice03-characterization-baselines` pins current variants and messages so future policy work can choose break, alias, deprecate, or no-op deliberately. | Separate public error API PR after operator GO; otherwise excluded from Arc04 repair PRs. |
| COHESION-004 | Medium | AST traversal and transformation cohesion | Later-arc deferral with baseline retained | Destination: future public AST/API contract arc; re-entry if `Formula` remains public after export policy, before public AST stabilization, or before adding downstream AST consumers | `Formula::to_dnf` and `to_nnf` panic contracts are public only because AST internals are broadly exported. The correct repair depends on the public export policy: hide internals, document panics, or add fallible APIs. | Medium/high: changing public panic behavior to `Result` or restricting visibility is an API change. | Yes for visibility changes or replacing public methods with fallible APIs. | No Arc04 production change by default. Future change either documents panic contracts, adds `try_to_*` APIs, or makes panic-only helpers crate-private. | `slice03-characterization-baselines` pins current panic contracts for equality, non-NNF, and probabilistic formula paths. | Separate public AST/API PR after operator GO; otherwise excluded from Arc04 parser/test cleanup PRs. |
| COHESION-005 | Low | Test cohesion | Focused repair slice | `slice05-test-helper-and-private-naming-cohesion` | Test helper drift is not product behavior, but shared helpers will make Arc04 repairs easier to maintain after behavior baselines are present. | None for runtime API; low review risk because assertions can accidentally weaken if helper consolidation is careless. | No. | No behavior/API change; test assertions become more consistent and retain current coverage. | Existing tests are the baseline. Slice05 must preserve assertion precision and keep the full suite green. | Test cohesion PR; can pair with private/test-only naming cleanup. |
| COHESION-006 | Low | Naming consistency | Focused repair slice for private/test names; public spelling repairs deferred behind operator GO | `slice05-test-helper-and-private-naming-cohesion` for private modules/test names; public enum variants go to future public API/naming policy re-entry | Private typos are low-risk maintenance cleanup. Public variants such as `QuantifierElimintation` and `Lexiacal` are downstream API and should not be silently renamed. | None/low for private modules and test names; high for public enum variants. | No for private/test names; yes for public enum variant spelling repairs. | Private module/test names can be corrected. Public names remain unchanged unless a later operator-approved compatibility plan says otherwise. | `slice03-characterization-baselines` pins public misspelled variants; Slice05 may update only private/test-only names. | Test/private naming PR; public spelling repair belongs with a future public API PR after GO. |

## Downstream Slices Opened

### slice03-characterization-baselines

Test-only baseline slice. It pins current public imports, current Vec-backed
parser/transpiler inputs, malformed problem-parser panic behavior, stringly
transformation/classification error variants, formula normalization panic
contracts, and public misspelled variants.

### slice04-parser-api-and-error-boundary

Focused repair slice. It changes borrowed byte inputs from `&Vec<u8>` to
`&[u8]`, adds the new slice-caller behavior tests, and converts the problem
parser panic path to a structured syntactic error. Public export narrowing is
not approved; the slice must stop for operator GO before removing public
re-exports.

### slice05-test-helper-and-private-naming-cohesion

Focused cleanup slice. It consolidates duplicated test assertion helpers and
repairs private/test-only naming drift after behavior baselines exist. It must
not rename public enum variants without operator GO.

## Operator GO Gates

- Root public export narrowing: re-entry after Slice03 shows which public names
  are currently reachable and before any `pub use` narrowing lands.
- Public error taxonomy changes: re-entry after Slice03 variant/message baseline
  and before replacing `Transformation(String)` or renaming `ParsingError`
  variants.
- Public transformation variant spelling: re-entry before renaming
  `Transformation::QuantifierElimintation`.
- Public formula API contract: re-entry after export-policy decision if
  `Formula` remains public, or before replacing panic contracts with fallible
  APIs.

## Later-Arc Deferrals

Destination: a future public API/error/AST contract arc if the operator wants a
breaking or compatibility-managed public API cleanup.

Reason: Arc04 can safely perform compatibility-improving parser input changes,
recoverable parser-error repairs, test helper consolidation, and private naming
cleanup. It cannot silently approve public API breaks.

Re-entry condition: open the public API contract arc before any 1.0-style API
stabilization, crate publication, downstream API support commitment, or
operator-approved breaking cleanup PR.

## Accepted Variation / No-Op Decisions

- Broad public exports are not declared accepted variation yet. They are held
  behind an operator GO gate because narrowing them may break downstream imports.
- Public misspelled variants are not declared no-op. They are deferred until the
  project chooses a public API compatibility policy.
- Formula normalization panic contracts are not declared no-op. They are
  deferred until `Formula` visibility is intentionally settled.
- Lexer `Cell` state, boxed recursive formulas, bounded `Box::leak` for
  synthesized identifiers, and LSP document-byte cloning remain accepted
  variation from Slice01; no new repair slice is opened for those local reasons.

## PR Grouping

- PR 1: Arc04 characterization baselines. Test-only, no production behavior
  changes.
- PR 2: Parser API and recoverable parser boundary. Includes `&[u8]` inputs and
  structured malformed-problem errors.
- PR 3: Test helper and private naming cohesion. No public API changes.
- Future PR: public API/error/AST naming cleanup only after operator GO and a
  compatibility plan.
