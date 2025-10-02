# rustdoc-json-to-markdown

> **🤖 AI-Generated Project**: This tool was created entirely by Claude (Anthropic AI). See [ATTRIBUTION.md](ATTRIBUTION.md) for details.

Convert rustdoc's JSON output to markdown format for LLM consumption and convenient documentation viewing.

## Motivation

Rustdoc generates excellent HTML documentation, but for LLM context or text-based viewing, markdown is more convenient. This tool converts rustdoc's JSON output (available via `--output-format=json`) into clean, structured markdown files.

## Installation

### From crates.io (recommended)

```bash
cargo install rustdoc-json-to-markdown
```

### From source

```bash
git clone https://github.com/Crazytieguy/rustdoc-json-to-markdown
cd rustdoc-json-to-markdown
cargo install --path .
```

## Usage

### Quick Start

```bash
# 1. Generate rustdoc JSON (requires nightly Rust)
cargo +nightly rustdoc -- --output-format=json -Z unstable-options

# 2. Convert to markdown
rustdoc-json-to-markdown target/doc/your_crate.json -o docs/

# 3. View the generated markdown
cat docs/index.md
```

### Document Dependencies

**NEW in v0.3.0**: Automatically generate documentation for your project's dependencies!

```bash
# Document specific dependencies
rustdoc-json-to-markdown --deps tokio,axum,serde -o docs/

# Document all direct dependencies
rustdoc-json-to-markdown --all-deps -o docs/

# Dependencies are organized in subdirectories
# Output structure:
#   docs/deps/tokio/index.md
#   docs/deps/axum/index.md
#   docs/deps/serde/index.md
```

This is perfect for:
- 📚 Creating comprehensive documentation for LLM context
- 🔍 Understanding your dependencies' APIs without leaving your terminal
- 📝 Generating offline documentation for your entire stack
- 🎯 Including dependency docs in your project's documentation

The tool automatically:
- Discovers dependencies from Cargo.toml
- Generates rustdoc JSON for each dependency
- Converts to organized markdown
- Gracefully handles failures (some deps may not build)

### Detailed Usage

**Step 1: Generate rustdoc JSON**

```bash
# For a Cargo project (most common)
cargo +nightly rustdoc -- --output-format=json -Z unstable-options

# For a single file
rustdoc +nightly --output-format=json -Z unstable-options src/lib.rs

# For a specific dependency
cargo +nightly rustdoc -p dependency_name -- --output-format=json -Z unstable-options
```

This creates a JSON file at `target/doc/<crate_name>.json`

**Step 2: Convert to Markdown**

```bash
rustdoc-json-to-markdown target/doc/my_crate.json -o docs/

# With options
rustdoc-json-to-markdown target/doc/my_crate.json \
  --output docs/ \
  --include-private  # Include private items
```

## Features

- ✅ **Module organization** (v0.2.0)
  - Items grouped by module with clear headers
  - Hierarchical table of contents
  - Full paths for all items (e.g., `backend::db::Account`)
  - Eliminates confusion from duplicate names
- ✅ **Dependency documentation** (v0.3.0)
  - Document all project dependencies with `--all-deps`
  - Document specific dependencies with `--deps <names>`
  - Automatic dependency discovery via cargo metadata
  - Organized output in `docs/deps/<crate>/` subdirectories
- ✅ Converts all major rustdoc item types
  - Structs (with field tables and type information)
  - Enums (with variant tables showing kinds: Unit, Tuple, Struct)
  - Functions (with complete type signatures)
  - Traits (with method listings)
  - Type aliases and constants
  - Modules
- ✅ Preserves documentation comments (already markdown)
- ✅ Full type formatting for signatures and fields
- ✅ Hierarchical table of contents organized by module
- ✅ Markdown tables for struct fields and enum variants
- ✅ Generic parameter support
- ✅ Trait implementation listings (inherent and user-defined traits)
- 🚧 Multi-file output (one file per module)
- 🚧 Cross-reference links between items

**Legend:** ✅ Implemented | 🚧 Planned

## Example Output

The tool generates clean, structured markdown with:

**Table of Contents:**
```markdown
## Table of Contents

- [Point](#point)
- [Color](#color)
- [add](#add)
```

**Struct with Field Table:**
```markdown
## Point

**Type:** Struct

Represents a 2D point in space.

**Fields:**

| Name | Type | Description |
|------|------|-------------|
| `x` | `i32` | The x coordinate |
| `y` | `i32` | The y coordinate |
```

**Enum with Variant Table:**
```markdown
## Color

**Type:** Enum

**Variants:**

| Variant | Kind | Description |
|---------|------|-------------|
| `Red` | Unit | Red color |
| `Rgb` | Tuple(u8, u8, u8) | Custom RGB color |
```

**Function with Type Signatures:**
```markdown
## add

**Type:** Function

Adds two numbers together.

\`\`\`rust
fn add(a: i32, b: i32) -> i32
\`\`\`
```

## CLI Options

```bash
rustdoc-json-to-markdown [INPUT] [OPTIONS]

Arguments:
  [INPUT]  Path to rustdoc JSON file (optional if using --deps or --all-deps)

Options:
  -o, --output <OUTPUT>              Output directory for markdown files [default: docs]
      --include-private              Include private items
      --deps <DEPS>                  Document specific dependencies (comma-separated)
      --all-deps                     Document all direct dependencies
  -h, --help                         Print help
  -V, --version                      Print version
```

**Examples:**

```bash
# Convert a single JSON file
rustdoc-json-to-markdown target/doc/my_crate.json -o docs/

# Document specific dependencies
rustdoc-json-to-markdown --deps tokio,serde -o docs/

# Document all dependencies
rustdoc-json-to-markdown --all-deps -o docs/
```

## Project Status

**Phase 1 Complete (MVP)** - The core functionality is implemented and tested with real-world crates:
- Successfully tested with anyhow, serde_json, and custom crates
- Generates well-formatted markdown with complete type information
- Produces structured documentation suitable for LLM consumption and viewing

**Note:** The rustdoc JSON format itself is unstable and requires nightly Rust.

See [PLAN.md](PLAN.md) for implementation details and Phase 2 roadmap.

## Requirements

- Rust nightly (to generate JSON output from rustdoc)
- Cargo (to build this tool)

## Dependencies

- `rustdoc-types` v0.56 - Type definitions for rustdoc JSON (from crates.io)
- `serde_json` - JSON parsing
- `anyhow` - Error handling
- `clap` - CLI interface with derive macros

## Contributing

This is a personal project exploring rustdoc JSON output. Contributions welcome once the MVP is complete.

## License

MIT or Apache-2.0 (same as Rust)

## Related Projects

- [rustdoc JSON RFC](https://rust-lang.github.io/rfcs/2963-rustdoc-json.html)
- [rustdoc-types](https://github.com/rust-lang/rust/tree/master/src/rustdoc-json-types)
- [Tracking issue for rustdoc JSON](https://github.com/rust-lang/rust/issues/76578)
