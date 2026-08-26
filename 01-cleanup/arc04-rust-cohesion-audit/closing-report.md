# Arc04 Closing Report: Rust Cohesion Audit And Fixes

Date: 2026-08-26
Arc: `arc04-rust-cohesion-audit`
Final feature branch: `fix/test-private-naming-cohesion`
Final feature commit: `7e2d8a7 test: consolidate helper and private naming cohesion`
Planning branch: `planning`

## Verdict

Arc04 is closed locally.

The arc delivered its capability: after the Arc03 best-practices repairs, the
project received a whole-codebase Rust cohesion audit, a triaged fix map,
behavior-preserving characterization coverage, focused parser API/error-boundary
repair, and focused test-helper/private naming repair. Similar problems now use
more consistent byte-input, parser-error, test-helper, private-module naming,
and public-API deferral patterns.

No public API breaking change landed in Arc04. Root export narrowing, public
error taxonomy redesign, public formula API contract changes,
`ParsingError::Lexiacal`, and `Transformation::QuantifierElimintation` remain
explicitly deferred behind operator GO or a future public API/error/AST contract
arc.

## Slice Walk

| Slice | Outcome | Evidence |
|-------|---------|----------|
| slice01-cohesion-diagnosis-audit | delivered | `slice01-cohesion-diagnosis-audit/cdc-verification.md` verifies the read-only cohesion audit, six findings, seven negative checks, and RUST-007 re-entry. |
| slice02-triage-and-fix-map | delivered | `slice02-triage-and-fix-map/cdc-verification.md` verifies every finding was mapped to a focused repair, accepted variation, or later-arc public API deferral. |
| slice03-characterization-baselines | delivered | `slice03-characterization-baselines/cdc-verification.md` verifies the test-only characterization baseline before production repair. |
| slice04-parser-api-and-error-boundary | delivered | `slice04-parser-api-and-error-boundary/cdc-verification.md` verifies the `&[u8]` byte-input repair and malformed-problem structured syntactic error. |
| slice05-test-helper-and-private-naming-cohesion | delivered | `slice05-test-helper-and-private-naming-cohesion/cdc-verification.md` verifies test-helper consolidation, private/test naming cleanup, ignored-test preservation, and no public API spelling changes. |

Slice count matches the Arc04 slice breakdown. No planned Arc04 slice is missing.

## Composition Check

The slices compose into the Arc04 capability.

- The diagnosis pass identified cohesion findings without changing code.
- The fix map converted those findings into reviewable repair slices and
  explicit public API gates.
- The characterization baseline pinned current public/API behavior before
  production changes.
- The parser repair improved borrowed byte-input cohesion and replaced a
  reachable parser panic with structured `ParsingError::Syntactic`.
- The final repair consolidated flawed-domain test helpers and corrected
  private/test-only naming drift while preserving public compatibility.

The final Arc04 feature state was independently reproduced at `7e2d8a7`:

```bash
cargo fmt --check
cargo check --locked --all-targets
RUSTFLAGS="-D rust-2024-compatibility" cargo check --locked --all-targets
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked --all-targets
cargo build --locked --release --bins
./target/release/hddl_analyzer --help
actionlint .github/workflows/ci.yml
git diff --check
```

All commands passed. `cargo test --locked --all-targets` preserved the inherited
ignored-test inventory that Arc05 now owns: one ignored library test, two
ignored flawed-domain integration tests, one ignored IPC integration test, and
one ignored JSON integration test.

## Arc Ledger Walk

| Row | Final status | Evidence |
|-----|--------------|----------|
| A4-1 | done | Slice01 CDC verification confirms the read-only cohesion diagnosis audit from Arc03 final feature state. |
| A4-2 | done | Slice02 CDC verification confirms every finding was dispositioned and Slice03-Slice05 were opened with public API gates. |
| A4-3 | done | Slice02 fix map records accepted variations, local rationale, and later-arc deferrals; Slice05 verification confirms public misspelled variants remain deliberate deferrals. |
| A4-4 | done | Slice04 and Slice05 production repair slices both have CDC verification and focused behavior-preservation stories. |
| A4-5 | done | The final feature state at `7e2d8a7` passes the full locked local gate, release build, binary smoke, actionlint, and whitespace checks. |
| A4-6 | done | This closing report records the arc composition check and project bubble-up. |

## Accumulated Plan Changes

Arc04 changed as the work revealed the right boundaries:

- v1.1 accepted the Slice01 diagnosis and moved to planning-only triage.
- v1.3 and v1.4 replaced the placeholder repair plan with Slice03
  characterization, Slice04 parser API/error-boundary repair, and Slice05
  test-helper/private naming cohesion.
- v1.5 through v1.9 recorded the verified baselines and repairs.
- v1.10 closes the arc and routes inherited ignored-test debt to Arc05.

The main constraint preserved throughout the arc was the public API gate:
compatibility-improving changes could land, but root export narrowing, public
error taxonomy redesign, formula API changes, and public spelling repairs
remained out of scope without explicit operator approval.

## Bubble-Up To The Project

Arc04 delivered the project roadmap capability: a final whole-codebase Rust
cohesion pass that makes the repaired codebase feel more deliberately unified
without smuggling in public API breaks.

The arc surfaced one project-level follow-on: inherited ignored tests should not
remain unexplained. That work is not part of the Arc04 cohesion repair PR
because the skipped tests may require test-only repair, code-and-test repair,
code-and-test rewrite, or a slow/corpus policy. Arc05 now owns that remediation
through an investigation-first slice.

Project-plan change required: yes. Mark Arc04 closed locally and make Arc05
Slice01 the active follow-on stream.

## Silent-Drop Diff

Arc04 promised:

- A diagnosis-only cohesion audit.
- A mapped repair plan.
- Behavior-preserving baselines before repair.
- Focused production repairs for approved cohesion findings.
- Explicit treatment of public API compatibility boundaries.
- A final local gate on the composed feature state.

Delivered:

- All planned slices are CDC-verified.
- All approved Arc04 production repairs landed.
- Public API breaking work is explicitly deferred, not silently dropped.
- The final local gate passed at the composed feature state.
- Inherited ignored-test debt is not hidden; it is routed to Arc05.

No Arc04-scoped capability is silently dropped.

## What Worked

The read-only diagnosis followed by a planning-only fix map kept the arc from
turning into a broad cleanup sweep. Characterization before production repair
made the public API compatibility decisions visible, and the private/public
naming split kept the final repair small enough to verify cleanly.
