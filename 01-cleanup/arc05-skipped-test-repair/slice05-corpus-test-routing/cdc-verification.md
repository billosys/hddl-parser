# CDC Verification: Arc05 Slice05 Corpus Test Routing

Date: 2026-08-26
Verifier: Codex
Feature branch: `test/corpus-test-routing`
Verified commit: `af0968b test: route corpus tests through fast selection`
Planning base: `a264af9 docs: close Arc05 Slice05 corpus routing`
Status: verified

## Scope Checked

CDC treated CC's Slice05 close report as proposed-done and independently
checked that the remaining corpus skips were removed by routing IPC and JSON
coverage through the verified fast corpus selection, while preserving the full
900-case corpus command as an explicit policy surface.

No implementation files were edited during verification.

## Result

Slice05 is CDC-verified.

The feature worktree is clean at `af0968b`. The two inherited corpus tests now
run by default over the checked-in fast selection, no Rust `#[ignore]`
annotations remain under `src` or `tests`, workflows are unchanged, and the
full 900-case corpus command passed with zero failures, zero JSON equality
disagreements, and zero JSON assertion failures.

## Evidence

```text
git -C /Users/oubiwann/lab/billosys/hddl-parser/.worktrees/features status --short --branch
## test/corpus-test-routing

git -C /Users/oubiwann/lab/billosys/hddl-parser/.worktrees/features log --oneline --decorate -5
af0968b (HEAD -> test/corpus-test-routing) test: route corpus tests through fast selection
8e36a6a (origin/test/corpus-test-routing, origin/policy/corpus-addressability, origin/main, origin/HEAD, policy/corpus-addressability, main) tools: add corpus selection policy
c6e4907 (origin/measure/corpus-phase-timings, measure/corpus-phase-timings) tools: add corpus measurement utility
e534df2 (origin/fix/fast-ignored-test-repair, fix/fast-ignored-test-repair) fix: repair fast ignored tests
7e2d8a7 (origin/fix/test-private-naming-cohesion, origin/audit/ignored-tests, fix/test-private-naming-cohesion, audit/ignored-tests) test: consolidate helper and private naming cohesion

rg -n "#\\[ignore" src tests -g '*.rs'
# no matches

git diff -- .github/workflows
# no output

actionlint .github/workflows/ci.yml
# passed
```

Focused test evidence:

```text
cargo test --locked --test ipc
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.51s

cargo test --locked --test json
test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.32s
```

Full default gate evidence:

```text
cargo test --locked --all-targets
# passed all targets
# library: 112 passed, 0 ignored
# arc04_characterization: 11 passed, 0 ignored
# current_behavior: 10 passed, 0 ignored
# integration_flawed: 23 passed, 0 ignored
# integration_ipc: 1 passed, 0 ignored
# integration_json: 9 passed, 0 ignored
# lsp_current_behavior: 6 passed, 0 ignored
```

Corpus command evidence:

```text
HDDL_CORPUS_SELECTION=fast HDDL_CORPUS_ASSERT=both cargo run --locked --example corpus_measure
selection=fast, assertion=both, discovered_cases=900, selected_cases=43
summary attempted=43 completed=43 failures=0
json_equality_disagreements=0
json_assertion_failures=0

HDDL_CORPUS_SELECTION=full HDDL_CORPUS_ASSERT=both cargo run --locked --example corpus_measure > /private/tmp/hddl-parser-slice05-full-corpus.log 2>&1
# exit status 0

rg -n "selection=|summary attempted=|json_equality_disagreements=|json_assertion_failures=" /private/tmp/hddl-parser-slice05-full-corpus.log
3:selection=full, assertion=both, discovered_cases=900, selected_cases=900
904:summary attempted=900 completed=900 failures=0
915:json_equality_disagreements=0
916:json_assertion_failures=0
```

Mechanical gate evidence:

```text
cargo fmt --check
cargo check --locked --all-targets
RUSTFLAGS="-D rust-2024-compatibility" cargo check --locked --all-targets
cargo clippy --locked --all-targets -- -D warnings
cargo build --release --locked --bins
./target/release/hddl_analyzer --help
git diff --check
git diff --cached --check
git -C ../planning diff --check
```

All commands above passed.

## Row Assessment

| ID | CDC Assessment |
|----|----------------|
| T5-1 | Verified. Slice05 is committed at `af0968b` on top of the Slice04 policy base. `tools/corpus_measure.rs`, `tests/ipc/corpus-selections/fast.txt`, and the explicit `corpus_measure` example target remain present. |
| T5-2 | Verified. `rg -n "#\\[ignore" src tests -g '*.rs'` returned no matches. |
| T5-3 | Verified. The default IPC corpus test passes and routes through the checked-in fast corpus selection with stable case IDs. |
| T5-4 | Verified. The default JSON corpus test passes and checks exact string plus structural `serde_json::Value` equality over the fast selection. |
| T5-5 | Verified. Full corpus validation remains explicitly addressable through the Slice04 command surface and passed 900/900 cases. |
| T5-6 | Verified. `.github/workflows` has no diff and `actionlint` passed. |
| T5-7 | Verified. The full locked local quality gate passed with no hidden skip debt. |
| T5-8 | Verified. The close report bubbles CI policy to the next slice without silently wiring workflow behavior. |

## Residual Risk

The full corpus command is locally green, but it is still not wired into GitHub
Actions. That is intentional Slice05 scope control. The next slice should
decide the branch-push, PR, scheduled, and post-merge/main policy using the
now-verified command surface.
