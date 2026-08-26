# IPC Corpus Selections

`corpus_measure` supports named and manifest-based corpus selections for local
validation and later CI wiring:

```bash
cargo run --locked --example corpus_measure
HDDL_CORPUS_SELECTION=fast HDDL_CORPUS_ASSERT=both cargo run --locked --example corpus_measure
HDDL_CORPUS_SELECTION=full HDDL_CORPUS_ASSERT=both cargo run --locked --example corpus_measure
HDDL_CORPUS_MANIFEST=tests/ipc/corpus-selections/fast.txt HDDL_CORPUS_ASSERT=both cargo run --locked --example corpus_measure
```

Stable case IDs use `<domain-dir>/<problem-file>`.

Selection precedence:

1. Discover all corpus cases under `tests/ipc`, sorted by stable case ID.
2. If `HDDL_CORPUS_MANIFEST` is set, load that newline-oriented case-ID file.
   Blank lines and full-line `#` comments are ignored. Duplicate IDs, unknown
   IDs, empty manifests, and unreadable paths fail non-zero.
3. Otherwise, apply `HDDL_CORPUS_SELECTION`. Supported values are `full` and
   `fast`; the default is `full`.
4. Apply `HDDL_CORPUS_FILTER` and `HDDL_CORPUS_LIMIT` after the manifest or
   named selection. These remain local iteration controls rather than policy.

JSON assertion policy is explicit through `HDDL_CORPUS_ASSERT`:

- `none`: measure and report equality results without failing on mismatches.
- `structural`: fail when `serde_json::Value` equality fails.
- `string`: fail when exact string equality fails.
- `both`: fail when either structural or exact string equality fails.

`both` is the recommended validation mode because the Slice03 full corpus
measurement found zero structural/string disagreements across 900 cases, and
the inherited JSON corpus test asserted exact string equality.

## Default Tests

The default locked test suite uses `fast.txt` for enabled IPC verification and
JSON round-trip corpus coverage:

```bash
cargo test --locked --test ipc
cargo test --locked --test json
cargo test --locked --all-targets
```

Those tests report stable case IDs in failure messages. The full corpus remains
explicit through the `HDDL_CORPUS_SELECTION=full` command above.

## Fast Selection

`fast.txt` is one fastest measured case per discovered corpus domain from the
2026-08-26 Slice03 measurement CSV. This gives a broad 43-domain smoke set for
branch-push use while avoiding the measured multi-second tail concentrated in
larger Minecraft cases.

The full corpus remains all 900 discovered cases and should be routed by a
later slice into whatever PR or post-merge policy the project chooses.
