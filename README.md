# rustdoc-to-markdown

Convert rustdoc's JSON output to markdown format for LLM consumption and convenient documentation viewing.

## Motivation

Rustdoc generates excellent HTML documentation, but for LLM context or text-based viewing, markdown is more convenient. This tool converts rustdoc's JSON output (available via `--output-format=json`) into clean, structured markdown files.

## Installation

```bash
cargo install --path .
```

## Usage

### Step 1: Generate rustdoc JSON

```bash
# For a single file
rustdoc --output-format=json -Z unstable-options src/lib.rs

# For a Cargo project
cargo rustdoc -- --output-format=json -Z unstable-options
```

This creates a JSON file at `target/doc/<crate_name>.json`

### Step 2: Convert to Markdown

```bash
rustdoc-to-markdown target/doc/my_crate.json -o docs/
```

## Features

- ✅ Converts all major rustdoc item types
  - Structs (with field tables and type information)
  - Enums (with variant tables showing kinds: Unit, Tuple, Struct)
  - Functions (with complete type signatures)
  - Traits (with method listings)
  - Type aliases and constants
  - Modules
- ✅ Preserves documentation comments (already markdown)
- ✅ Full type formatting for signatures and fields
- ✅ Generates table of contents with links
- ✅ Markdown tables for struct fields and enum variants
- ✅ Generic parameter support
- 🚧 Multi-file output (one file per module)
- 🚧 Cross-reference links between items
- 🚧 External crate links
- 🚧 Trait implementation listings

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
rustdoc-to-markdown <INPUT> [OPTIONS]

Arguments:
  <INPUT>  Path to rustdoc JSON file

Options:
  -o, --output <OUTPUT>              Output directory for markdown files [default: docs]
      --include-private              Include private items
  -h, --help                         Print help
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
