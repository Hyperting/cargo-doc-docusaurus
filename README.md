# cargo-doc-docusaurus

> **🤖 AI-Generated Project**: Created entirely by Claude (Anthropic AI), Qwen 3 Coder (Alibaba Cloud) and Grok Code Fast 1 (xAI). See [ATTRIBUTION.md](ATTRIBUTION.md).
>
> **📝 Based on**: Fork of [cargo-doc-md](https://github.com/Crazytieguy/cargo-doc-md) by [Yoav Tzfati](https://github.com/Crazytieguy)

Generate Docusaurus-compatible markdown documentation from Rust crates. Like `cargo doc`, but outputs markdown with React components instead of HTML.

## Installation

```bash
cargo install cargo-doc-docusaurus
```

## Quick Start

```bash
# 1. Setup Docusaurus components (one-time)
cargo doc-docusaurus components init /path/to/docusaurus

# 2. Generate rustdoc JSON
RUSTDOCFLAGS="-Z unstable-options --output-format json" \
  cargo +nightly doc --no-deps

# 3. Convert to markdown
cargo doc-docusaurus target/doc/my_crate.json -o docs/api
```

## Features

- **One-command setup** - Install all components automatically
- **Multi-file output** - One markdown per item (struct, enum, trait, etc.)
- **Workspace-aware** - Internal links between workspace crates
- **React components** - Syntax highlighting, crate badges, clickable types
- **Auto-generated sidebar** - Complete navigation structure
- **Private items** - Optional with `--include-private`

## Commands

### Component Management

```bash
# Install components (React, CSS, theme overrides)
cargo doc-docusaurus components init <PATH>

# Update components
cargo doc-docusaurus components sync <PATH>

# List components and status
cargo doc-docusaurus components list [PATH]

# Get CSS variables for customization
cargo doc-docusaurus components css-vars [--css-only]
```

### Documentation Generation

```bash
cargo doc-docusaurus <INPUT.json> [OPTIONS]
```

| Option | Description | Example |
|--------|-------------|---------|
| `-o, --output <DIR>` | Output directory | `--output docs/api` |
| `--base-path <PATH>` | Base URL path for links | `--base-path "/docs/api"` |
| `--workspace-crates <CRATES>` | Comma-separated workspace crates for internal linking | `--workspace-crates "core,utils"` |
| `--include-private` | Include private items | `--include-private` |
| `--sidebar-output <PATH>` | Custom sidebar location | `--sidebar-output sidebars-rust.ts` |
| `--sidebar-root-link <URL>` | Back link in sidebar | `--sidebar-root-link "/docs"` |
| `--sidebarconfig-collapsed` | Generate collapsed sidebar | `--sidebarconfig-collapsed` |

## Examples

### Single Crate

```bash
RUSTDOCFLAGS="-Z unstable-options --output-format json" \
  cargo +nightly doc --no-deps

cargo doc-docusaurus target/doc/my_crate.json \
  -o docs/api \
  --base-path "/docs/api"
```

### Multi-Crate Workspace

```bash
WORKSPACE_CRATES="core,utils,server"

RUSTDOCFLAGS="-Z unstable-options --output-format json" \
  cargo +nightly doc --no-deps

for crate in core utils server; do
  cargo doc-docusaurus target/doc/${crate}.json \
    -o docs/api \
    --base-path "/docs/api" \
    --workspace-crates "$WORKSPACE_CRATES"
done
```

**Result:** Cross-references between crates use internal links (`/docs/api/utils/...`) instead of docs.rs.

## Documentation

- **[SETUP.md](SETUP.md)** - Complete setup guide with Docusaurus integration
- **[templates/CSS_GUIDE.md](templates/CSS_GUIDE.md)** - Customize colors and styles

## Development

```bash
cargo test              # Run all tests
cargo insta review      # Review snapshot changes
```

See [tests/README.md](tests/README.md) for details.

## License

MIT or Apache-2.0
