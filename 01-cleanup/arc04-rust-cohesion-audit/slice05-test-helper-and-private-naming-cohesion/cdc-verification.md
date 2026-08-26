# Slice05 CDC Verification: Test Helper And Private Naming Cohesion

Date: 2026-08-26
Feature branch: `fix/test-private-naming-cohesion`
Feature commit: `7e2d8a7 test: consolidate helper and private naming cohesion`
Planning branch: `planning`

## Verdict

CDC verification passes. Slice05 is verified as a focused test-helper and
private/test-only naming cohesion repair with no public API spelling changes.

The implementation diff is limited to the expected test/private naming surface:

- `src/semantic_analyzer/tests/tdg_tests.rs`
- `src/syntactic_analyzer/domain_parser/action_parser.rs`
- `src/transpiler/transformations/mod.rs`
- `src/transpiler/transformations/quantifier_elimination.rs`
- `tests/integration_flawed.rs`

The old private module path
`src/transpiler/transformations/qunatifier_elimintation.rs` is a pure rename to
`quantifier_elimination.rs`.

## Commands Reproduced

Feature worktree:

```bash
git status --short --branch --untracked-files=all
git log --oneline --decorate -5
git diff --name-status d17b41f..HEAD
git diff --stat d17b41f..HEAD
git diff d17b41f..HEAD -- src/output/errors/generic.rs src/transpiler/transformations/transform.rs
rg -n "QuantifierElimintation|Lexiacal|qunatifier|elimintation|parantheses|satelite" src tests -g '*.rs'
cargo fmt --check
cargo check --locked --all-targets
RUSTFLAGS="-D rust-2024-compatibility" cargo check --locked --all-targets
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked --test flawed
cargo test --locked --all-targets
cargo build --locked --release --bins
./target/release/hddl_analyzer --help
actionlint .github/workflows/ci.yml
git diff --check
```

Planning worktree:

```bash
git diff --check
```

Cargo commands that write `target/` were rerun with the sandbox approval path
because the HDDL-Parser worktree is outside the current writable root.

## Reproduced Results

- Feature status was clean on `fix/test-private-naming-cohesion`.
- `git diff --name-status d17b41f..HEAD` matched the expected five-file
  boundary: two test/comment files, one module declaration, one private module
  rename, and `tests/integration_flawed.rs`.
- Public API files `src/output/errors/generic.rs` and
  `src/transpiler/transformations/transform.rs` had no diff.
- Public spellings `ParsingError::Lexiacal` and
  `Transformation::QuantifierElimintation` remain present and covered by
  characterization tests.
- Private/test-only typo spellings covered by the Slice05 scope no longer
  appear in lowercase grep form.
- Focused flawed-domain tests passed: 21 passed, 2 ignored.
- Full locked all-target tests passed. Ignored-test count was preserved:
  library tests: 111 passed, 1 ignored; flawed integration tests: 21 passed, 2
  ignored; IPC integration tests: 1 ignored; JSON integration tests: 8 passed,
  1 ignored; LSP tests: 6 passed.
- Release binary help smoke passed and listed `convert`, `verify`, `metadata`,
  `format`, and `help`.
- `actionlint`, feature whitespace check, and planning whitespace check passed.

## Row Walk

| Row | CDC disposition | Evidence |
|-----|-----------------|----------|
| T4-1 | reproduced | Branch is `fix/test-private-naming-cohesion`; prior Slice03/Slice04 CDC verification files exist. |
| T4-2 | reproduced | `tests/integration_flawed.rs` centralizes result handling while each call site still asserts exact variants and fields; focused and full tests pass. |
| T4-3 | reproduced | Full test output preserves the inherited ignored-test pattern: 1 library, 2 flawed, 1 IPC, and 1 JSON ignored. |
| T4-4 | reproduced | Private module rename and private/test-only spelling fixes are present; lowercase typo grep finds no private/test drift. |
| T4-5 | reproduced | Public API enum files have no diff; public misspelled variants remain present and characterized. |
| T4-6 | reproduced | Full local gate passed with locked Cargo checks, Rust 2024 compatibility, Clippy `-D warnings`, tests, release build, binary help smoke, actionlint, and whitespace checks. |
| T4-7 | reproduced | Closing report walks all seven rows and bubbles up the remaining public API deferrals. |

## Bubble-Up Check

Slice05 delivers the final planned Arc04 repair piece: test-helper cohesion and
private/test-only naming cleanup. The silent-drop diff is clean for the slice:
helper consolidation landed, private/test names were repaired, ignored-test
status was preserved, and public API spelling changes stayed deliberately out
of scope.

This slice does require an Arc04 plan update: all planned Arc04 repair slices
are now CDC-verified, A4-4 and A4-5 can close, and Arc04 is ready for arc-level
closure. Public API/error/AST compatibility work remains deferred behind
explicit operator GO or a future public API contract arc.

## What Worked

Keeping the helper extraction limited to parse/verify result handling made the
review crisp: every test still owns its exact semantic or syntactic assertion.

The public/private spelling boundary held cleanly. The diff fixes private and
test-only drift while leaving downstream-visible enum variants pinned for a
separate compatibility decision.
