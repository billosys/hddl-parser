# CDC Verification: Arc05 Slice06 GitHub Actions Corpus Policy

Date: 2026-08-26
Verifier: Codex
Feature branch: `ci/corpus-policy`
Verified commit: `4f41000 ci: add corpus policy gates`
Planning base: `9249981 docs: open Arc05 Slice06 CI corpus policy`
Status: verified

## Scope Checked

CDC treated CC's Slice06 close report as proposed-done and independently
checked the workflow, documentation, corpus commands, and local Rust gate.

CC's implementation was initially left as an uncommitted two-file feature
diff. CDC reproduced the required gates, then committed the verified feature
diff as `4f41000` so the slice close has a concrete landed commit.

## Result

Slice06 is CDC-verified.

The workflow now runs locked Cargo gates, keeps the existing Linux/macOS
matrix and checkout/toolchain choices, runs explicit fast corpus measurement
for all workflow runs, and runs full corpus measurement for pull requests,
scheduled runs, and pushes to `main` or `master`. The full corpus step remains
inside the existing OS matrix, so qualifying events run it on both Linux and
macOS.

The feature diff is limited to `.github/workflows/ci.yml` and
`tests/ipc/corpus-selections/README.md`. No parser, verifier, transpiler, JSON,
test, corpus-measurement, or fast-selection semantics changed.

## Evidence

```text
git -C /Users/oubiwann/lab/billosys/hddl-parser/.worktrees/features status --short --branch
## ci/corpus-policy

git -C /Users/oubiwann/lab/billosys/hddl-parser/.worktrees/features log --oneline --decorate -3
4f41000 (HEAD -> ci/corpus-policy) ci: add corpus policy gates
af0968b (origin/test/corpus-test-routing, origin/ci/corpus-policy, test/corpus-test-routing) test: route corpus tests through fast selection
8e36a6a (origin/policy/corpus-addressability, origin/main, origin/HEAD, policy/corpus-addressability, main) tools: add corpus selection policy

git diff --name-status HEAD^ HEAD
M       .github/workflows/ci.yml
M       tests/ipc/corpus-selections/README.md
```

Workflow policy evidence:

```text
rg -n "ubuntu-24.04|macos-15|actions/checkout@v7" .github/workflows/ci.yml
33:          - ubuntu-24.04
34:          - macos-15
38:        uses: actions/checkout@v7

rg -n "cargo (check|clippy|test|build).*--locked|cargo run --locked --example corpus_measure" .github/workflows/ci.yml
69:        run: cargo check --locked --all-targets
74:        run: cargo check --locked --all-targets
77:        run: cargo clippy --locked --all-targets -- -D warnings
80:        run: cargo test --locked --all-targets
83:        run: HDDL_CORPUS_SELECTION=fast HDDL_CORPUS_ASSERT=both cargo run --locked --example corpus_measure
87:        run: HDDL_CORPUS_SELECTION=full HDDL_CORPUS_ASSERT=both cargo run --locked --example corpus_measure
90:        run: cargo build --release --locked --bins

rg -n "HDDL_CORPUS_SELECTION=fast|HDDL_CORPUS_SELECTION=full|HDDL_CORPUS_ASSERT=both|pull_request|refs/heads/main|refs/heads/master|schedule|cron" .github/workflows/ci.yml
4:  pull_request:
18:  schedule:
19:    - cron: "17 8 * * 1"
83:        run: HDDL_CORPUS_SELECTION=fast HDDL_CORPUS_ASSERT=both cargo run --locked --example corpus_measure
86:        if: github.event_name == 'pull_request' || github.event_name == 'schedule' || github.ref == 'refs/heads/main' || github.ref == 'refs/heads/master'
87:        run: HDDL_CORPUS_SELECTION=full HDDL_CORPUS_ASSERT=both cargo run --locked --example corpus_measure

actionlint .github/workflows/ci.yml
# passed
```

Skipped-test and local gate evidence:

```text
rg -n "#\\[ignore" src tests -g '*.rs'
# no matches

cargo fmt --check
cargo check --locked --all-targets
RUSTFLAGS="-D rust-2024-compatibility" cargo check --locked --all-targets
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked --all-targets
cargo build --release --locked --bins
./target/release/hddl_analyzer --help
git diff --check
git -C ../planning diff --check
```

All commands above passed. The full locked test gate included:

```text
library: 112 passed, 0 ignored
arc04_characterization: 11 passed, 0 ignored
current_behavior: 10 passed, 0 ignored
integration_flawed: 23 passed, 0 ignored
integration_ipc: 1 passed, 0 ignored
integration_json: 9 passed, 0 ignored
lsp_current_behavior: 6 passed, 0 ignored
```

Corpus command evidence:

```text
HDDL_CORPUS_SELECTION=fast HDDL_CORPUS_ASSERT=both cargo run --locked --example corpus_measure
selection=fast, assertion=both, discovered_cases=900, selected_cases=43
summary attempted=43 completed=43 failures=0
json_equality_disagreements=0
json_assertion_failures=0

HDDL_CORPUS_SELECTION=full HDDL_CORPUS_ASSERT=both cargo run --locked --example corpus_measure > /private/tmp/hddl-parser-slice06-cdc-full-corpus.log 2>&1
# exit status 0

rg -n "selection=|summary attempted=|json_equality_disagreements=|json_assertion_failures=" /private/tmp/hddl-parser-slice06-cdc-full-corpus.log
3:selection=full, assertion=both, discovered_cases=900, selected_cases=900
904:summary attempted=900 completed=900 failures=0
915:json_equality_disagreements=0
916:json_assertion_failures=0
```

## Row Assessment

| ID | CDC Assessment |
|----|----------------|
| T6-1 | Verified. Slice06 is based on committed Slice05 feature state and no Rust `#[ignore]` annotations remain. |
| T6-2 | Verified. The workflow preserves `ubuntu-24.04`, `macos-15`, and `actions/checkout@v7`; direct `rustup` setup remains. |
| T6-3 | Verified. Dependency-resolving Cargo commands in CI use `--locked`; `cargo fmt --check` remains appropriately unlocked. |
| T6-4 | Verified. Branch pushes get the ordinary quality gate and explicit fast corpus measurement; non-default branch pushes do not run the full corpus gate. |
| T6-5 | Verified. Pull requests run the full corpus step inside the Linux/macOS matrix. |
| T6-6 | Verified. Pushes to `main` and `master` run the full corpus step inside the Linux/macOS matrix. |
| T6-7 | Verified. A weekly schedule is present and the full corpus condition includes `schedule`. |
| T6-8 | Verified. The landed feature diff is workflow policy plus corpus-selection documentation only. |
| T6-9 | Verified. Fast and full corpus commands are locally reproducible and passed with zero failures and zero JSON assertion failures. |
| T6-10 | Verified. The full local quality gate passed after workflow policy changes. |
| T6-11 | Verified. The close report bubbles Arc05 toward composition/PR-readiness closure rather than another feature slice. |

## Bubble-Up Check

Slice06 delivered its assigned Arc05 piece: the corpus policy is now explicit
in GitHub Actions while preserving the test and command semantics proven by
Slices04 and 05.

The silent-drop check is clean. Scheduled policy was implemented rather than
omitted, full corpus gates cover both default-branch names, and the full
corpus step is matrixed across Linux and macOS.

No further feature slice is indicated by CDC verification. Arc05 should move
next to arc-level composition and PR-readiness closure, unless the operator
wants to split upstream PR packaging decisions into a separate planning-only
slice.
