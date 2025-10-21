# cargo-doc-docusaurus

> **🤖 AI-Generated Project**: Created entirely by Claude (Anthropic AI), Qwen 3 Coder (Alibaba Cloud) and Grok Code Fast 1 (xAI). See [ATTRIBUTION.md](ATTRIBUTION.md).

A Cargo subcommand that generates Docusaurus-compatible markdown documentation for Rust crates and their dependencies. Like `cargo doc`, but outputs markdown instead of HTML.

> **📝 Based on**: This project is a fork of [cargo-doc-md](https://github.com/Crazytieguy/cargo-doc-md) by [Yoav Tzfati](https://github.com/Crazytieguy), modified to generate Docusaurus-compatible documentation structure with React component integration.

## Installation

```bash
cargo install cargo-doc-docusaurus
```

## Usage

```bash
# Document current crate + all dependencies (like cargo doc)
cargo doc-docusaurus

# Document only dependencies
cargo doc-docusaurus --all-deps

# Document specific dependencies
cargo doc-docusaurus --deps tokio,axum

# Custom output directory
cargo doc-docusaurus -o docs/

# Docusaurus integration with base path
cargo doc-docusaurus --base-path "/docs/runtime/rust"

# Multi-crate workspace with internal links
cargo doc-docusaurus \
  --base-path "/docs/runtime/rust" \
  --workspace-crates "my_core,my_utils,my_server" \
  target/doc/my_core.json \
  -o apps/docs/docs/runtime/rust
```

### Output Structure

The tool generates a file structure optimized for Docusaurus integration:

```
docs/
  my_crate/
    index.md                           # Crate overview with module tree
    module1/
      index.md                         # Module overview
      struct.MyStruct.md               # Individual struct documentation
      enum.MyEnum.md                   # Individual enum documentation
      fn.my_function.md                # Individual function documentation
    nested/
      submodule/
        index.md
        trait.MyTrait.md               # Individual trait documentation
```

Each file includes:
- **Frontmatter** with `title` for Docusaurus
- **React component imports** (`RustCode` for syntax highlighting)
- **Type signatures** with clickable links to related types
- **Trait implementations** with full method signatures
- **Documentation comments** from source code

### Use Cases

- **Docusaurus documentation sites** - Generate browsable API docs for your Rust projects
- **Multi-crate workspaces** - Document all workspace crates with cross-references
- **Provide API documentation to LLMs** as context for code generation and analysis
- **Version-controlled documentation** - Markdown files can be committed and reviewed
- **Offline documentation** - Self-contained docs that work without internet
- **Custom documentation workflows** - Integrate with static site generators beyond Docusaurus

> **Note**: This tool follows the same conventions as `cargo doc`, including visibility rules, module structure, and re-export handling. The key difference is the output format: while `cargo doc` generates HTML, this tool generates Docusaurus-compatible markdown with React components.

## Features

- **Cargo doc compatible** - follows the same visibility and module structure rules as `cargo doc`
- **Multi-file output** with one markdown file per item (struct, enum, trait, function)
- **Docusaurus-compatible** with React component integration
- **Breadcrumb navigation** showing module hierarchy
- **Module index pages** with item summaries and counts
- **Complete type signatures** for all items with clickable links
- **Base path support** for proper routing in Docusaurus (e.g., `/docs/my-section/rust`)
- **Workspace-aware linking** - internal links for workspace crates instead of docs.rs
- **Enum variant fields** - displays full signatures like `MyEnumVariant(Item, SecondItem)`
- **Re-export handling** - items are documented where they're re-exported, just like `cargo doc`
- **Private items support** - optionally include private items with `--include-private`
- **Trait implementations** - shows all trait impls with full method signatures

## Options

```
cargo doc-docusaurus [OPTIONS] [INPUT]

Arguments:
  [INPUT]  Path to rustdoc JSON file (omit to auto-document current crate + all deps)

Options:
  -o, --output <DIR>              Output directory [default: target/doc-md]
      --include-private           Include private items in documentation
      --base-path <PATH>          Base path for links (e.g., '/docs/runtime/rust' for Docusaurus)
      --workspace-crates <CRATES> Workspace crates (comma-separated) - generates internal links
                                  instead of docs.rs for these crates
      --all-deps                  Document only dependencies (exclude current crate)
      --deps <CRATES>             Document specific dependencies (comma-separated)
  -h, --help                      Show help
```

Run `cargo doc-docusaurus --help` for detailed information.

Requires Rust nightly.

## Docusaurus Integration

This tool is designed to work seamlessly with Docusaurus. The generated markdown files use a custom `RustCode` React component for syntax highlighting and clickable type links.

### Template Files and Setup

Template files for Docusaurus integration (React component, CSS styles, and documentation) are provided in the `templates/` directory. For detailed installation instructions and customization options, see [templates/README.md](templates/README.md).

### Basic Setup

1. Generate rustdoc JSON files:
```bash
RUSTC_BOOTSTRAP=1 RUSTDOCFLAGS="-Z unstable-options --output-format json" \
  cargo doc --no-deps --document-private-items
```

2. Convert to Docusaurus-compatible markdown:
```bash
cargo doc-docusaurus \
  --include-private \
  --base-path "/docs/runtime/rust" \
  target/doc/my_crate.json \
  -o docs/runtime/rust
```

3. The `--base-path` ensures all internal links are routed correctly through Docusaurus.

### Multi-Crate Workspace

For projects with multiple crates in a workspace, use `--workspace-crates` to generate internal links between workspace crates instead of external docs.rs links:

```bash
# List all workspace crates that will be documented
WORKSPACE_CRATES="crate_a,crate_b,crate_c"

# Generate docs for each crate with workspace awareness
for crate in crate_a crate_b crate_c; do
  cargo doc-docusaurus \
    --include-private \
    --base-path "/docs/runtime/rust" \
    --workspace-crates "$WORKSPACE_CRATES" \
    target/doc/${crate}.json \
    -o docs/runtime/rust
done
```

This ensures that when `crate_a` references a type from `crate_b`, the link points to `/docs/runtime/rust/crate_b/...` instead of `https://docs.rs/crate_b/...`.

### Example Justfile

```just
WORKSPACE_CRATES := "my_core,my_server,my_client"

gen-docs: gen-rustdoc-json gen-docusaurus-md

gen-rustdoc-json:
  RUSTC_BOOTSTRAP=1 RUSTDOCFLAGS="-Z unstable-options --output-format json" \
    cargo doc --no-deps --document-private-items

gen-docusaurus-md:
  cargo doc-docusaurus --include-private \
    --base-path "/docs/runtime/rust" \
    --workspace-crates "{{WORKSPACE_CRATES}}" \
    target/doc/my_core.json -o docs/runtime/rust
  cargo doc-docusaurus --include-private \
    --base-path "/docs/runtime/rust" \
    --workspace-crates "{{WORKSPACE_CRATES}}" \
    target/doc/my_server.json -o docs/runtime/rust
  cargo doc-docusaurus --include-private \
    --base-path "/docs/runtime/rust" \
    --workspace-crates "{{WORKSPACE_CRATES}}" \
    target/doc/my_client.json -o docs/runtime/rust
```

## Development

This project uses snapshot testing to ensure output quality and consistency. Run tests with:

```bash
cargo test
```

When making changes to the output format, review snapshot changes with:

```bash
cargo insta review
```

See [tests/README.md](tests/README.md) for more information about the test suite.

## License

MIT or Apache-2.0
