# Test Workspace Fixture

This workspace contains two crates to test workspace documentation features.

## Structure

```
test_workspace/
├── Cargo.toml          # Workspace manifest
├── crate_a/            # Depends on crate_b
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs      # Contains StructA, references DataB from crate_b
├── crate_b/            # Base crate with core types
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs      # Contains DataB, ResultB, Processor trait
├── crate_a.json        # Pre-generated rustdoc JSON
└── crate_b.json        # Pre-generated rustdoc JSON
```

## Purpose

This fixture tests:

1. **Cross-crate references**: `crate_a` references types from `crate_b`
2. **Internal linking**: When documenting workspace crates, links between them should be internal (not docs.rs)
3. **Re-exports**: `crate_a` re-exports `DataB` from `crate_b`
4. **Workspace awareness**: The `--workspace-crates` parameter should recognize both crates

## Key Test Cases

- `StructA.data` field is of type `crate_b::DataB` → should link to `/docs/rust/crate_b/struct.DataB`
- `combine()` function returns `crate_b::ResultB` → should link internally
- `pub use crate_b::DataB` re-export → should be documented correctly

## Regenerating

```bash
cd tests/fixtures/test_workspace
RUSTC_BOOTSTRAP=1 RUSTDOCFLAGS="-Z unstable-options --output-format json" \
  cargo +nightly doc --no-deps
cp target/doc/crate_a.json .
cp target/doc/crate_b.json .
```
