# rustdoc-json-to-markdown

> **🤖 AI-Generated Project**: Created entirely by Claude (Anthropic AI). See [ATTRIBUTION.md](ATTRIBUTION.md).

Generate markdown documentation for Rust crates and their dependencies. Perfect for LLM context and offline viewing.

## Installation

```bash
cargo install rustdoc-json-to-markdown
```

## Usage

### Document Dependencies (Main Use Case)

```bash
# Document all dependencies
rustdoc-json-to-markdown --all-deps

# Document specific dependencies
rustdoc-json-to-markdown --deps tokio,axum,serde

# Custom output directory
rustdoc-json-to-markdown --all-deps -o docs/
```

**Output:**
```
docs/deps/
  tokio/index.md
  axum/index.md
  serde/index.md
```

Perfect for:
- 📚 LLM context with comprehensive API docs
- 🔍 Understanding dependencies without leaving the terminal
- 📝 Offline documentation for your entire stack

### Document Your Own Crate

```bash
# Generate rustdoc JSON (requires nightly)
cargo +nightly rustdoc -- --output-format=json -Z unstable-options

# Convert to markdown
rustdoc-json-to-markdown target/doc/your_crate.json -o docs/
```

## Features

- **Module organization** - Hierarchical structure with full paths (`crate::module::Type`)
- **Dependency automation** - Auto-discovers and documents dependencies
- **Complete type info** - Structs, enums, functions, traits, with full signatures
- **Clean tables** - Field tables, variant tables, method listings
- **Smart handling** - Gracefully skips deps that fail to build
- **Multi-version support** - Handles multiple versions of the same dependency

## CLI Reference

```bash
rustdoc-json-to-markdown [INPUT] [OPTIONS]

Options:
  -o, --output <DIR>        Output directory [default: docs]
      --all-deps            Document all direct dependencies
      --deps <CRATES>       Document specific dependencies (comma-separated)
      --include-private     Include private items
  -h, --help               Show help
```

**Note:** Requires Rust nightly to generate rustdoc JSON.

## License

MIT or Apache-2.0
