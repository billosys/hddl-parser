# Slice04: Binary Smoke And Status

Date: 2026-08-25
Branch: `feature/add-ci`
Arc: `arc01-github-actions-ci`

## Goal

Add CI evidence that the shipped binaries build and that the command-line tool
can be invoked. Add minimal project status polish if it remains review-friendly.

## In Scope

- `cargo build --release --bins`
- `hddl_analyzer --help` smoke check
- Optional README CI badge if it points to the new workflow and does not
  distract from the CI PR

## Out Of Scope

- Language-server integration testing
- Packaging, release uploads, installers, or binary artifacts
- CLI behavior fixes

## Verification Approach

Run the release build locally and invoke the release `hddl_analyzer --help`.
Inspect any README badge/link for the upstream repository path.

## Exit Criteria

- CI builds both binaries in release mode.
- CI invokes `hddl_analyzer --help`.
- Any README status change is minimal and accurate.
