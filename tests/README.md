# Snapshot Tests

This directory contains snapshot tests for the cargo-doc-docusaurus tool. Snapshot testing ensures that the markdown output remains consistent across changes.

## Structure

- `fixtures/` - Test data
  - `test_crate/` - A comprehensive test crate covering all Rust documentation features
  - `test_crate.json` - Pre-generated rustdoc JSON for the test crate
  - `test_workspace/` - A workspace with multiple crates for testing workspace features
    - `crate_a/` - A crate that depends on crate_b (demonstrates cross-crate references)
    - `crate_b/` - A base crate with core types
    - `crate_a.json` - Pre-generated rustdoc JSON for crate_a
    - `crate_b.json` - Pre-generated rustdoc JSON for crate_b
- `snapshot_tests.rs` - Snapshot test suite
- `snapshots/` - Stored snapshots of expected output (auto-generated)

## Running Tests

```bash
# Run all snapshot tests
cargo test --test snapshot_tests

# Run all tests
cargo test
```

## Updating Snapshots

When you make intentional changes to the output format:

```bash
# Run tests to see differences
cargo test --test snapshot_tests

# Review and accept changes
cargo insta review

# Or accept all changes
cargo insta accept
```

## What's Tested

The snapshot tests cover:

- **Structs**: Plain, tuple, unit, with generics, with methods
- **Enums**: Simple and complex variants
- **Functions**: Various signatures including generic and async functions
- **Traits**: Trait definitions and implementations
- **Modules**: Nested module hierarchies
- **Constants and Type Aliases**
- **Multi-file output structure**
- **Index generation**
- **Legacy single-file output**
- **Workspace support**: Multi-crate workspaces with cross-crate references
- **Internal linking**: Workspace crates link to each other instead of docs.rs
- **Base path handling**: Correct URL generation for Docusaurus routing

## How Snapshot Tests Help with PRs

Snapshot tests make it easy to see exactly how output has changed:

1. When tests fail, `insta` shows a diff of the changes
2. Reviewers can see the exact markdown output differences in the PR
3. Snapshots are committed to git, so changes are visible in diffs
4. This makes it easy to verify that changes are intentional and correct

## Regenerating Test Fixtures

### Single Crate (test_crate)

If you need to regenerate the test fixture JSON:

```bash
cd tests/fixtures/test_crate
cargo +nightly rustdoc --lib -- --output-format=json -Z unstable-options
cp target/doc/test_crate.json ../test_crate.json
```

### Workspace (test_workspace)

To regenerate the workspace fixture JSONs:

```bash
cd tests/fixtures/test_workspace
RUSTC_BOOTSTRAP=1 RUSTDOCFLAGS="-Z unstable-options --output-format json" \
  cargo +nightly doc --no-deps
cp target/doc/crate_a.json .
cp target/doc/crate_b.json .
```
