# Slice01: Workflow Scaffold

Date: 2026-08-25
Branch: `feature/add-ci`
Arc: `arc01-github-actions-ci`

## Goal

Create the first GitHub Actions workflow skeleton for HDDL-Parser without yet
packing it with every gate. This slice establishes triggers, checkout,
toolchain setup, and cache/concurrency conventions.

## In Scope

- `.github/workflows/ci.yml`
- PR and push triggers for `main`
- Linux and macOS runner matrix unless a local constraint proves one unusable
- Stable Rust toolchain setup with required components for later slices
- Cargo cache strategy that is conventional and low-risk

## Out Of Scope

- Edition migration
- Code fixes beyond what is required to make the workflow syntactically valid
- Release publishing or artifact upload

## Verification Approach

Run local static inspection of the workflow file and, if available, `actionlint`.
The workflow must be readable enough for later slices to append gates without
restructuring it.

## Exit Criteria

- Workflow file exists at `.github/workflows/ci.yml`.
- Workflow triggers on PRs and pushes to `main`.
- Workflow defines a Rust toolchain setup and runner matrix.
- The PR remains focused on CI scaffolding.
