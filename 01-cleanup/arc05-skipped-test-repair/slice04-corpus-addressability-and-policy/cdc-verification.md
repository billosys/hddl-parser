# Slice04 CDC Verification: Corpus Addressability And Policy

Date: 2026-08-26
Feature branch: `policy/corpus-addressability`
Feature base: `c6e4907 tools: add corpus measurement utility`
Planning branch: `planning`

## Verdict

CDC verification passes. Slice04 makes the corpus measurement runner
policy-ready with named fast/full selections, custom manifest support, explicit
JSON assertion modes, and clean policy/input failures while leaving CI and the
two inherited corpus `#[ignore]` annotations unchanged.

The feature implementation remains uncommitted at verification time and is
limited to:

- `tools/corpus_measure.rs`
- `tests/ipc/corpus-selections/README.md`
- `tests/ipc/corpus-selections/fast.txt`

## Commands Reproduced

Feature worktree:

```bash
git status --short --branch
git log --oneline --decorate -3
sed -n '1,900p' tools/corpus_measure.rs
sed -n '1,220p' tests/ipc/corpus-selections/README.md
sed -n '1,120p' tests/ipc/corpus-selections/fast.txt
rg -n "name = \"corpus_measure\"|path = \"tools/corpus_measure.rs\"|#\\[ignore" Cargo.toml src tests -g '*.rs'
test -f tools/corpus_measure.rs
test -f tests/ipc/corpus-selections/fast.txt
test -f tests/ipc/corpus-selections/README.md
rg -v '^\\s*(#|$)' tests/ipc/corpus-selections/fast.txt
rg -v '^\\s*(#|$)' tests/ipc/corpus-selections/fast.txt | wc -l
rg -v '^\\s*(#|$)' tests/ipc/corpus-selections/fast.txt | sort | uniq -d
rg -v '^\\s*(#|$)' tests/ipc/corpus-selections/fast.txt | cut -d/ -f1 | sort -u | wc -l
rg -n 'Minecraft-Player/p-013-013-013-013|Minecraft-Regular/p-050-050-050-050|Minecraft-Regular/p-9-9-9-50|Minecraft-Regular/p-5-5-5-50|Minecraft-Player/p-012-012-012-012|Minecraft-Regular/p-045-045-045-045|Minecraft-Regular/p-9-9-9-45|Minecraft-Regular/p-5-5-5-45|Minecraft-Player/p-011-011-011-011|Minecraft-Regular/p-040-040-040-040' tests/ipc/corpus-selections/fast.txt
git diff -- .github/workflows tests/integration_ipc.rs tests/integration_json.rs
HDDL_CORPUS_LIMIT=5 cargo run --locked --example corpus_measure
HDDL_CORPUS_SELECTION=fast HDDL_CORPUS_ASSERT=both cargo run --locked --example corpus_measure
HDDL_CORPUS_MANIFEST=tests/ipc/corpus-selections/fast.txt HDDL_CORPUS_ASSERT=both cargo run --locked --example corpus_measure
HDDL_CORPUS_SELECTION=fast HDDL_CORPUS_ASSERT=none cargo run --locked --example corpus_measure
HDDL_CORPUS_SELECTION=fast HDDL_CORPUS_ASSERT=structural cargo run --locked --example corpus_measure
HDDL_CORPUS_SELECTION=fast HDDL_CORPUS_ASSERT=string cargo run --locked --example corpus_measure
printf 'AssemblyHierarchical/genericLinearProblem_depth01.hddl\nAssemblyHierarchical/genericLinearProblem_depth01.hddl\n' > /private/tmp/hddl-parser-duplicate-corpus-selection.txt
printf 'NoSuchDomain/nope.hddl\n' > /private/tmp/hddl-parser-unknown-corpus-selection.txt
HDDL_CORPUS_SELECTION=wat cargo run --locked --example corpus_measure
HDDL_CORPUS_ASSERT=wat cargo run --locked --example corpus_measure
HDDL_CORPUS_MANIFEST=/private/tmp/hddl-parser-missing-corpus-selection.txt cargo run --locked --example corpus_measure
HDDL_CORPUS_MANIFEST=/private/tmp/hddl-parser-duplicate-corpus-selection.txt cargo run --locked --example corpus_measure
HDDL_CORPUS_MANIFEST=/private/tmp/hddl-parser-unknown-corpus-selection.txt cargo run --locked --example corpus_measure
cargo fmt --check
cargo check --locked --all-targets
cargo test --locked --all-targets
RUSTFLAGS="-D rust-2024-compatibility" cargo check --locked --all-targets
cargo clippy --locked --all-targets -- -D warnings
cargo build --release --locked --bins
./target/release/hddl_analyzer --help
actionlint .github/workflows/ci.yml
git diff --check
```

Planning worktree:

```bash
git diff --check
```

## Reproduced Results

- Branch `policy/corpus-addressability` is based at
  `c6e4907 tools: add corpus measurement utility`, with Slice04 present as
  unstaged feature changes.
- `tools/corpus_measure.rs` exists, and `Cargo.toml` still declares the
  explicit example surface with `name = "corpus_measure"` and
  `path = "tools/corpus_measure.rs"`.
- `HDDL_CORPUS_LIMIT=5 cargo run --locked --example corpus_measure` reported
  `selection=full`, `discovered_cases=900`, `selected_cases=5`,
  `completed=5`, `failures=0`, `json_equality_disagreements=0`, and
  `json_assertion_failures=0`.
- `tests/ipc/corpus-selections/fast.txt` and `README.md` exist. The fast
  selection has 43 non-comment IDs, 43 represented domains, and no duplicate
  IDs.
- The known Slice03 slow-tail Minecraft cases searched during verification are
  absent from `fast.txt`.
- `HDDL_CORPUS_SELECTION=fast HDDL_CORPUS_ASSERT=both cargo run --locked --example corpus_measure`
  reported `selected_cases=43`, `completed=43`, `failures=0`,
  `json_equality_disagreements=0`, and `json_assertion_failures=0`.
- `HDDL_CORPUS_MANIFEST=tests/ipc/corpus-selections/fast.txt HDDL_CORPUS_ASSERT=both cargo run --locked --example corpus_measure`
  selected the same 43 cases and completed with zero failures, zero equality
  disagreements, and zero assertion failures.
- Fast selection runs passed with `HDDL_CORPUS_ASSERT=none`, `structural`,
  `string`, and `both`.
- Invalid policy inputs failed cleanly with process exit 1 and actionable
  `error:` messages:
  - invalid selection: `HDDL_CORPUS_SELECTION must be one of: full, fast`
  - invalid assertion: `HDDL_CORPUS_ASSERT must be one of: none, structural, string, both`
  - missing manifest: `failed to read corpus selection`
  - duplicate manifest ID: duplicate case ID with line number
  - unknown manifest ID: unknown corpus case ID
- `rg -n "#\\[ignore" src tests -g '*.rs'` reports only
  `tests/integration_ipc.rs:7` and `tests/integration_json.rs:17`.
- `git diff -- .github/workflows tests/integration_ipc.rs
  tests/integration_json.rs` is empty.
- `cargo fmt --check`, locked all-target check, locked all-target tests,
  strict Rust 2024 compatibility check, strict Clippy, release binary build,
  binary help smoke, `actionlint`, feature whitespace check, and planning
  whitespace check all passed.

## Row Walk

| Row | CDC disposition | Evidence |
|-----|-----------------|----------|
| CP5-1 | reproduced | Branch head is the Slice03 measurement commit with Slice04 unstaged; `tools/corpus_measure.rs` exists; `Cargo.toml` keeps the explicit `corpus_measure` example; bounded full run completed 5 of 5 selected cases. |
| CP5-2 | reproduced | `fast.txt` and `README.md` exist; fast manifest has 43 non-comment IDs, 43 unique represented domains, no duplicates, and resolves through the manifest runner with zero failures. |
| CP5-3 | reproduced | Named `fast`, named `full` with limit, and explicit manifest selection all run successfully. README documents manifest-over-selection precedence, followed by filter and limit. |
| CP5-4 | reproduced | Invalid selection, invalid assertion mode, missing manifest, duplicate ID, and unknown ID all fail non-zero with useful `error:` text and no panic. |
| CP5-5 | reproduced | Source defines `AssertionMode::{None, Structural, String, Both}`; fast selection passes with `none`, `structural`, `string`, and `both`. |
| CP5-6 | reproduced | `fast.txt` documents one fastest measured case per discovered domain; verification counted 43 cases across 43 domains and confirmed representative fast runs have zero failures/disagreements while excluding searched slow-tail Minecraft IDs. |
| CP5-7 | reproduced | Default locked all-target tests pass; only the two corpus ignores remain; workflow and inherited corpus test diffs are empty. |
| CP5-8 | reproduced | Full local gate passed: fmt, locked check, locked tests, Rust 2024 compatibility, Clippy `-D warnings`, release build, help smoke, actionlint, and whitespace checks; closing report walks CP5-1 through CP5-8 and includes Bubble-Up. |

## Bubble-Up Check

Slice04 delivers its assigned Arc05 piece: an addressable corpus policy
substrate now exists before CI wiring or ignore removal.

The silent-drop diff is clean. The slice adds named fast/full selections,
custom manifest loading, checked-in fast corpus policy, and explicit JSON
assertion modes. It does not change GitHub Actions, does not remove the two
corpus `#[ignore]` annotations, does not optimize parser/transpiler behavior,
and does not introduce a custom test framework.

This verification does not close Arc05. It does change the next planning
boundary: the next slice can now route or replace the inherited corpus tests
through the verified named policy commands, with CI policy split into a
separate slice if the diff becomes large enough for a cleaner review boundary.

## What Worked

Keeping corpus addressability as a Cargo example plus checked-in manifests
gave the project deterministic coverage controls without turning test
infrastructure into its own subsystem. The explicit JSON assertion mode is also
the right future-proofing: branch, PR, and post-merge policy can choose the
same runner with different gates rather than changing the tool again.
