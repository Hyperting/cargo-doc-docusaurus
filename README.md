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

- ✅ Converts all rustdoc item types (structs, enums, functions, traits, etc.)
- ✅ Preserves documentation comments (already markdown)
- ✅ Formats type signatures as code blocks
- ✅ Generates table of contents
- ✅ Cross-references between items
- 🚧 Multi-file output (one file per module)
- 🚧 External crate links
- 🚧 Trait implementation listings

**Legend:** ✅ Implemented | 🚧 Planned

## Example Output

```markdown
# my_crate

## Module: utils

### Function: add

\`\`\`rust
pub fn add(a: i32, b: i32) -> i32
\`\`\`

Adds two numbers together.

**Parameters:**
- `a` - First number
- `b` - Second number

**Returns:** The sum of `a` and `b`
```

## Project Status

**Early Development** - This project is under active development. The rustdoc JSON format itself is unstable and requires nightly Rust.

See [PLAN.md](PLAN.md) for implementation details and roadmap.

## Requirements

- Rust nightly (to generate JSON output from rustdoc)
- Cargo (to build this tool)

## Dependencies

- `rustdoc-json-types` - Vendored from rust-lang/rust (unstable)
- `serde_json` - JSON parsing
- `anyhow` - Error handling
- `clap` - CLI interface

## Contributing

This is a personal project exploring rustdoc JSON output. Contributions welcome once the MVP is complete.

## License

MIT or Apache-2.0 (same as Rust)

## Related Projects

- [rustdoc JSON RFC](https://rust-lang.github.io/rfcs/2963-rustdoc-json.html)
- [rustdoc-types](https://github.com/rust-lang/rust/tree/master/src/rustdoc-json-types)
- [Tracking issue for rustdoc JSON](https://github.com/rust-lang/rust/issues/76578)
