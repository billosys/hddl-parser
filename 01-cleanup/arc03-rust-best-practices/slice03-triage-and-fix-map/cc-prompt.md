# CC Prompt: Arc03 Slice03 Triage And Fix Map

You are working in HDDL-Parser on Arc03 Slice03:
`arc03-rust-best-practices/slice03-triage-and-fix-map`.

This is a planning-only slice. Do not edit Rust source, tests, manifests,
workflows, README, or dependency policy in this slice.

Read these files first:

- `/Users/oubiwann/lab/billosys/hddl-parser/.worktrees/planning/01-cleanup/project-plan.md`
- `/Users/oubiwann/lab/billosys/hddl-parser/.worktrees/planning/01-cleanup/arc03-rust-best-practices/arc-plan.md`
- `/Users/oubiwann/lab/billosys/hddl-parser/.worktrees/planning/01-cleanup/arc03-rust-best-practices/slice01-diagnosis-only-audit/cdc-verification.md`
- `/Users/oubiwann/lab/billosys/hddl-parser/.worktrees/planning/01-cleanup/arc03-rust-best-practices/slice02-baseline-characterization-tests/cdc-verification.md`
- `/Users/oubiwann/lab/billosys/hddl-parser/.worktrees/planning/01-cleanup/arc03-rust-best-practices/slice03-triage-and-fix-map/slice-doc.md`
- `/Users/oubiwann/lab/billosys/hddl-parser/.worktrees/planning/01-cleanup/arc03-rust-best-practices/slice03-triage-and-fix-map/ledger.md`
- `workbench/2026.08.25-audit-index.md`
- `workbench/2026.08.25-audit-results-rust.md`

Goal:

Create a concrete fix map for all Arc03 audit findings and open the next
repair slices that are clear enough to hand off without another planning pass.

Required output:

- Create
  `/Users/oubiwann/lab/billosys/hddl-parser/.worktrees/planning/01-cleanup/arc03-rust-best-practices/slice03-triage-and-fix-map/fix-map.md`.
- Cover every finding `RUST-001` through `RUST-008`.
- For each finding, include:
  - severity and category;
  - disposition: fix in Arc03, defer to Arc04, defer outside this cleanup
    project, or no-op;
  - target repair slice or deferral destination;
  - rationale;
  - expected behavior change;
  - Slice02 baseline tests that will be updated, or why no behavior baseline
    is needed;
  - proposed upstream PR grouping.
- Update `arc-plan.md` to replace/refine the placeholder repair slice if you
  open concrete repair slices.
- Create the open set (`slice-doc.md`, `ledger.md`, `cc-prompt.md`) for every
  concrete repair slice you open now.
- Update `project-plan.md` only if the fix map changes project-level status,
  sequencing, upstream PR grouping, or Arc04 responsibilities.

Initial grouping hypothesis to evaluate, not blindly copy:

- RUST-001: CLI recoverable failures should likely be the first focused repair
  because Slice02 has direct process baselines for the current bad exit codes.
- RUST-002 and RUST-003 may belong together if the clean repair is a shared
  structured-error path for parser/transpiler/transform failures; split them if
  the API change makes the diff too large.
- RUST-004, RUST-005, and RUST-008 may belong together as an LSP robustness
  slice, but keep the `RwLock` contention work separate if deterministic tests
  require a larger harness or refactor.
- RUST-006 should probably be a Cargo reproducibility/policy slice, separate
  from behavior repairs.
- RUST-007 might be an Arc03 API cleanup if it can be done safely with the
  current tests, or an Arc04 cohesion item if explicit re-export design needs a
  whole-codebase consistency pass.

Important constraints:

- Do not lose the Slice02 LSP harness deferrals. Carry forward non-file URI,
  `didSave`, unreadable/missing sibling files, no-domain-found behavior, and
  diagnostic `RwLock` contention as explicit repair-slice decisions or
  deferrals with re-entry conditions.
- Do not mark Low findings complete just because they are low severity.
- Do not open close-set files for Slice03 or later repair slices.
- Keep upstream reviewability in view. Prefer focused PR groups with crisp
  behavior changes and tests over a broad "best practices" branch.

Run and record:

```bash
test -f 01-cleanup/arc03-rust-best-practices/slice03-triage-and-fix-map/fix-map.md
rg -n "RUST-001|RUST-002|RUST-003|RUST-004|RUST-005|RUST-006|RUST-007|RUST-008" \
  01-cleanup/arc03-rust-best-practices/slice03-triage-and-fix-map/fix-map.md
rg -n "fix in Arc03|defer to Arc04|defer outside|no-op|PR grouping|baseline|re-entry" \
  01-cleanup/arc03-rust-best-practices/slice03-triage-and-fix-map/fix-map.md
rg -n "tests/current_behavior.rs|tests/lsp_current_behavior.rs|current_behavior|baseline|update" \
  01-cleanup/arc03-rust-best-practices/slice03-triage-and-fix-map/fix-map.md
rg -n "RwLock|contention|non-file|didSave|unreadable|harness|re-entry" \
  01-cleanup/arc03-rust-best-practices/slice03-triage-and-fix-map/fix-map.md
find 01-cleanup/arc03-rust-best-practices -maxdepth 2 -type f | sort
git diff --name-only
git diff --check
```

Closing report requirements:

- Walk every ledger row `C3-1` through `C3-12`; no silent drops.
- State the final disposition for every audit finding.
- State exactly which repair slices were opened and why.
- State any findings deferred to Arc04 or outside this cleanup project, with
  re-entry conditions.
- Bubble up any arc-plan or project-plan changes.
