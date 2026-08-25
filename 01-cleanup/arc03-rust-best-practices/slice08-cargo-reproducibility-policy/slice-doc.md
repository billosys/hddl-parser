# Slice08: Cargo Reproducibility Policy

Version: 1.0
Date: 2026-08-25
Arc: `arc03-rust-best-practices`
Expected implementation branch: `fix/cargo-reproducibility-policy`

## Goal

Fix RUST-006 by making dependency resolution reproducible for this package's binaries.

## Scope

In scope:

- Replacing wildcard dependency requirements in `Cargo.toml` with explicit compatible semver requirements.
- Deciding and implementing `Cargo.lock` tracking policy for a package with binaries.
- Updating `.gitignore` only as needed for the lockfile policy.
- Regenerating `Cargo.lock` through Cargo if tracking it.

Out of scope:

- Runtime behavior repairs.
- README or release automation changes unless the lockfile policy requires a short documented note in a planning report.
- MSRV policy unless required by dependency resolution.

## Expected Behavior

CI and local builds resolve repeatable dependency versions. If `Cargo.lock` remains untracked, the slice must document why this binary package is intentionally treated as a library-only resolution surface.

## Verification Approach

Inspect manifest requirements, lockfile tracking state, and `.gitignore`; run the full local workflow-equivalent gate after any lockfile generation.

## Exit Criteria

- RUST-006 is fixed or explicitly reclassified with a documented project rationale.
- Dependency requirements are no longer wildcard-only.
- Lockfile policy is observable through tracked files and `git ls-files Cargo.lock`.
- The full local workflow-equivalent gate passes.
