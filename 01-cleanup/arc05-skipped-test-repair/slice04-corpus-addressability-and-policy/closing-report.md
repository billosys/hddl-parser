# Slice04: Corpus Addressability And Policy Closing Report

Date: 2026-08-26
Feature branch: `policy/corpus-addressability`
Feature base: `c6e4907 tools: add corpus measurement utility`
Status: locally closed; CDC verified 2026-08-26

## Summary

Slice04 made the corpus measurement runner policy-ready without changing CI or
removing the two inherited corpus `#[ignore]` annotations.

The runner now supports:

- named `full` and `fast` selections;
- custom newline-oriented manifest files with comments and blank lines;
- duplicate, unknown, empty, unreadable, and invalid-policy input errors;
- explicit JSON assertion modes: `none`, `structural`, `string`, and `both`.

The checked-in fast selection lives under `tests/ipc/corpus-selections/` and
uses one fastest measured case per discovered corpus domain from the Slice03
measurement CSV. The fast command selected 43 cases across 43 domains and
passed with zero failures, zero JSON equality disagreements, and zero assertion
failures.

## Row Walk

### CP5-1

Done, attested. The branch starts from `c6e4907 tools: add corpus measurement
utility`, and `cargo run --locked --example corpus_measure` remains the runner
surface. `tools/corpus_measure.rs` exists, and `Cargo.toml` still declares:

- `name = "corpus_measure"`
- `path = "tools/corpus_measure.rs"`

`HDDL_CORPUS_SELECTION=full HDDL_CORPUS_LIMIT=5 HDDL_CORPUS_ASSERT=both cargo run --locked --example corpus_measure`
passed with `selected_cases=5`, `completed=5`, `failures=0`, and
`json_assertion_failures=0`.

### CP5-2

Done, attested. The checked-in fast selection files exist:

- `tests/ipc/corpus-selections/fast.txt`
- `tests/ipc/corpus-selections/README.md`

The fast file has 43 non-comment case IDs and 43 unique represented domains.
Running it through the manifest path:

`HDDL_CORPUS_MANIFEST=tests/ipc/corpus-selections/fast.txt HDDL_CORPUS_ASSERT=both cargo run --locked --example corpus_measure`

selected 43 known cases and completed with zero failures and zero assertion
failures.

### CP5-3

Done, attested. Named and manifest selections work:

- `HDDL_CORPUS_SELECTION=fast HDDL_CORPUS_ASSERT=both cargo run --locked --example corpus_measure`
  selected 43 cases, completed 43, failures 0.
- `HDDL_CORPUS_SELECTION=full HDDL_CORPUS_LIMIT=5 HDDL_CORPUS_ASSERT=both cargo run --locked --example corpus_measure`
  selected 5 sorted full-corpus cases, completed 5, failures 0.
- `HDDL_CORPUS_MANIFEST=tests/ipc/corpus-selections/fast.txt HDDL_CORPUS_ASSERT=both cargo run --locked --example corpus_measure`
  selected 43 cases, completed 43, failures 0.

`tests/ipc/corpus-selections/README.md` documents precedence: discover sorted
corpus cases, apply manifest if set, otherwise apply named selection, then apply
`HDDL_CORPUS_FILTER` and `HDDL_CORPUS_LIMIT` as local iteration controls.

### CP5-4

Done, attested. Expected input and policy failures return non-zero with useful
messages:

- `HDDL_CORPUS_SELECTION=wat cargo run --locked --example corpus_measure`
  exited 1 with `error: HDDL_CORPUS_SELECTION must be one of: full, fast`.
- `HDDL_CORPUS_ASSERT=wat cargo run --locked --example corpus_measure`
  exited 1 with `error: HDDL_CORPUS_ASSERT must be one of: none, structural, string, both`.
- `HDDL_CORPUS_MANIFEST=/tmp/hddl-parser-bad-corpus-selection.txt cargo run --locked --example corpus_measure`
  exited 1 with `error: failed to read corpus selection`.
- `HDDL_CORPUS_MANIFEST=/private/tmp/hddl-parser-duplicate-corpus-selection.txt cargo run --locked --example corpus_measure`
  exited 1 with a duplicate case ID and line number.
- `HDDL_CORPUS_MANIFEST=/private/tmp/hddl-parser-unknown-corpus-selection.txt cargo run --locked --example corpus_measure`
  exited 1 with the unknown case ID.

No expected policy/input error path panicked.

### CP5-5

Done, attested. `tools/corpus_measure.rs` defines
`AssertionMode::{None, Structural, String, Both}` and applies assertion
failures after measuring all selected cases.

Fast selection passed in every assertion mode:

- `HDDL_CORPUS_ASSERT=none`
- `HDDL_CORPUS_ASSERT=structural`
- `HDDL_CORPUS_ASSERT=string`
- `HDDL_CORPUS_ASSERT=both`

Each run reported `completed=43`, `failures=0`,
`json_equality_disagreements=0`, and `json_assertion_failures=0`.

### CP5-6

Done, attested. The fast selection is measurement-backed. `fast.txt` documents
that it uses one fastest measured case per discovered corpus domain from the
2026-08-26 Slice03 measurement CSV.

Observed fast-command evidence:

- selected cases: 43
- represented domains: 43
- failures: 0
- JSON equality disagreements: 0
- JSON assertion failures: 0

The slowest observed fast representatives were
`Freecell-Learned-ECAI-16/probfreecell-04-5.hddl` and
`Minecraft-Player/p-003-003-003-003.hddl`, both chosen as the fastest available
representatives for their domains. The multi-second Minecraft cases from the
Slice03 slow tail are not in the fast selection.

### CP5-7

Done, attested. Default test behavior is unchanged.

`cargo test --locked --all-targets` passed. The ignored-test scan still reports
only:

- `tests/integration_ipc.rs:7`
- `tests/integration_json.rs:17`

`git diff -- .github/workflows tests/integration_ipc.rs tests/integration_json.rs`
is empty.

### CP5-8

Done, attested. The full local gate passed:

- `cargo fmt --check`
- `cargo check --locked --all-targets`
- `cargo test --locked --all-targets`
- `RUSTFLAGS="-D rust-2024-compatibility" cargo check --locked --all-targets`
- `cargo clippy --locked --all-targets -- -D warnings`
- `cargo build --release --locked --bins`
- `./target/release/hddl_analyzer --help`
- `rg -n "#\\[ignore" src tests -g '*.rs'`
- `git diff -- .github/workflows tests/integration_ipc.rs tests/integration_json.rs`
- `actionlint .github/workflows/ci.yml`
- `git diff --check`
- `git -C ../planning diff --check`

## Bubble-Up To Arc05

Slice04 delivered the Arc05 piece assigned in `arc-plan.md`: the remaining
corpus ignored tests now have an addressable fast/full policy substrate before
CI wiring or ignore removal.

The slice surfaced a clean next boundary. The next Arc05 slice should replace
or route the two inherited ignored corpus tests through the named commands:

- fast validation: `HDDL_CORPUS_SELECTION=fast HDDL_CORPUS_ASSERT=both cargo run --locked --example corpus_measure`
- full validation: `HDDL_CORPUS_SELECTION=full HDDL_CORPUS_ASSERT=both cargo run --locked --example corpus_measure`

CI wiring can either be part of that slice or split into a following slice if
the test replacement/routing diff becomes large enough to deserve its own
review boundary. The policy substrate is now present; the final branch, PR, and
post-merge matrix decision remains intentionally unwired.

Silent-drop diff: no CI files were changed, the two corpus `#[ignore]`
annotations remain in place, parser/verifier/transpiler behavior was not
optimized or rewritten, and no extra test framework or parallel runner was
added.
