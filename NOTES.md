# Development Notes

## Initial Setup

Project created with:
- `cargo new rustdoc-to-markdown --bin`
- Dependencies: serde_json, anyhow, clap (with derive feature)
- rustdoc-json-types from git (rust-lang/rust repo)

## rustdoc-json-types Dependency Issue

**Warning:** The first build will be slow because it clones the entire rust-lang/rust repository (~1GB+).

### Alternatives to Consider:

1. **Use a specific commit/tag** (recommended once you test)
   ```toml
   rustdoc-json-types = { git = "https://github.com/rust-lang/rust.git", rev = "42b384ec" }
   ```
   This pins to a specific version and makes subsequent builds faster.

2. **Vendor locally** once the dependency downloads
   ```bash
   cargo vendor
   ```
   Then use: `rustdoc-json-types = { path = "vendor/rustdoc-json-types" }`

3. **Wait for crates.io**
   The rustdoc team plans to publish this crate once JSON output stabilizes.

4. **Alternative: Parse JSON manually**
   You could define just the types you need and skip the dependency entirely.
   This gives you more control but requires maintaining type definitions.

## Next Steps

When continuing implementation:

1. Start with `src/main.rs` - basic CLI that loads a JSON file
2. Test with a simple crate's JSON output
3. Implement converters one item type at a time
4. Focus on structs/enums/functions first (most common)
5. Add tests as you go

## Testing the Tool

To generate test JSON files:

```bash
# Create a simple test crate
cargo new --lib test-crate
cd test-crate

# Generate JSON
cargo rustdoc -- --output-format=json -Z unstable-options

# JSON will be at: target/doc/test_crate.json
```

## Format Version

Current rustdoc JSON format version: **56** (as of Oct 2025)

Your tool should check `crate.format_version` and warn if incompatible.
