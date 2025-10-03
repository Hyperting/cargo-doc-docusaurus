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

**Output Structure:**
```
docs/
  index.md                    # Master index of all crates
  your_crate/
    index.md                  # Crate overview + module list
    module1.md                # Each module in its own file
    module2.md
    sub/
      nested_module.md        # Nested modules create subdirectories
  deps/
    tokio/
      index.md                # Tokio overview
      io.md                   # Each tokio module
      net.md
      sync.md
    axum/
      index.md
      ...
```

Perfect for:
- 📚 LLM context with comprehensive API docs
- 🔍 Understanding your entire stack without leaving the terminal
- 📝 Offline documentation for crate + dependencies
- 🧭 Easy navigation with multi-file output (no more 14k-line files!)

### Document Only Dependencies

```bash
# All dependencies
rustdoc-json-to-markdown --all-deps

# Specific dependencies
rustdoc-json-to-markdown --deps tokio,axum
```

## Features

**Navigation & Organization:**
- **Master index** - Single entry point listing all documented crates
- **Multi-file output** - One file per module (no more giant files!)
- **Breadcrumb navigation** - Know where you are: `backend > db > models`
- **Module summaries** - See counts at a glance: *27 structs, 2 enums, 1 type alias*
- **Organized contents** - Items grouped by type (Structs, Enums, Functions, etc.)

**Documentation Quality:**
- **Complete type info** - Full signatures for structs, enums, functions, traits
- **Clean tables** - Field tables, variant tables, method listings
- **Module organization** - Hierarchical structure with full paths (`crate::module::Type`)

**Automation:**
- **One-command workflow** - Automatically generates JSON and converts to markdown
- **Dependency discovery** - Auto-discovers and documents all dependencies
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
