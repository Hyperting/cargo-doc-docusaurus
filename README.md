# rustdoc-json-to-markdown

> **🤖 AI-Generated Project**: Created entirely by Claude (Anthropic AI). See [ATTRIBUTION.md](ATTRIBUTION.md).

Generate markdown documentation for Rust crates and their dependencies. Perfect for LLM context and offline viewing.

## Installation

```bash
cargo install rustdoc-json-to-markdown
```

## Usage

### One Command for Everything (like `cargo doc`)

```bash
# Document your crate + all dependencies
rustdoc-json-to-markdown

# Custom output directory
rustdoc-json-to-markdown -o docs/
```

**Output:**
```
docs/
  index.md           # Your crate
  deps/
    tokio/index.md
    axum/index.md
    serde/index.md
```

Perfect for:
- 📚 LLM context with comprehensive API docs
- 🔍 Understanding your entire stack without leaving the terminal
- 📝 Offline documentation for crate + dependencies

### Document Only Dependencies

```bash
# All dependencies
rustdoc-json-to-markdown --all-deps

# Specific dependencies
rustdoc-json-to-markdown --deps tokio,axum
```

## Features

- **One-command workflow** - Automatically generates JSON and converts to markdown
- **Dependency automation** - Auto-discovers and documents dependencies
- **Module organization** - Hierarchical structure with full paths (`crate::module::Type`)
- **Complete type info** - Structs, enums, functions, traits, with full signatures
- **Clean tables** - Field tables, variant tables, method listings
- **Multi-version support** - Handles multiple versions of the same dependency
- **Smart handling** - Gracefully skips deps that fail to build

## CLI Reference

```bash
rustdoc-json-to-markdown [INPUT] [OPTIONS]

# Default (no args): Documents current crate + all dependencies
# With --all-deps or --deps: Documents only dependencies
# With INPUT file: Converts that JSON file

Options:
  -o, --output <DIR>        Output directory [default: docs]
      --all-deps            Document only all dependencies
      --deps <CRATES>       Document only specific dependencies
      --include-private     Include private items
  -h, --help               Show help
```

**Note:** Requires Rust nightly.

## License

MIT or Apache-2.0
