# Slice08: Cargo Reproducibility Policy Ledger

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
|----|-----------|--------|--------------|--------|--------|----------|-------|
| C8-1 | The slice diff is limited to Cargo reproducibility policy and generated lockfile state. | `git diff --name-only main..HEAD` and inspect changed files. | serious | RUST-006 | open | | Expected files are `Cargo.toml`, `.gitignore`, and `Cargo.lock` if tracked. |
| C8-2 | Wildcard dependency requirements are replaced with explicit compatible semver requirements. | `rg -n 'petgraph = "\\*"|version = "1\\.\\*"|serde_json = "1\\.\\*"' Cargo.toml` expecting no matches, plus inspect replacements. | serious | RUST-006 | open | | |
| C8-3 | Lockfile policy is resolved for this package with binaries. | `git ls-files Cargo.lock`; inspect `.gitignore`; inspect closing-report rationale. | serious | RUST-006 | open | | Prefer tracking `Cargo.lock` for binary reproducibility. |
| C8-4 | Cargo regenerates or validates dependency resolution without manual lockfile edits. | `cargo check --all-targets` after lockfile/policy changes. | correctness | cargo-guidelines | open | | Do not edit `Cargo.lock` by hand. |
| C8-5 | No runtime behavior, source cleanup, LSP, CLI, or public API change is mixed in. | `git diff --name-only main..HEAD` and inspect non-Cargo files. | correctness | slice03-fix-map | open | | |
| C8-6 | The full local workflow-equivalent gate passes. | `cargo fmt --check`; `cargo check --all-targets`; `RUSTFLAGS="-D rust-2024-compatibility" cargo check --all-targets`; `cargo clippy --all-targets -- -D warnings`; `cargo test --all-targets`; `cargo build --release --bins`; `./target/release/hddl_analyzer --help`; `actionlint .github/workflows/ci.yml`; `git diff --check` | serious | arc-plan | open | | |
| C8-7 | Slice08 closing report walks every row and states the final dependency/lockfile policy. | Inspect `closing-report.md` for C8-1 through C8-7. | correctness | ledger-discipline | open | | |

## What Worked

_(At slice close.)_

## Closure

_(At slice close.)_
