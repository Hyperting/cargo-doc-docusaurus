# cargo-doc-docusaurus

> **🤖 AI-Generated Project**: Created entirely by Claude (Anthropic AI), Qwen 3 Coder (Alibaba Cloud) and Grok Code Fast 1 (xAI). See [ATTRIBUTION.md](ATTRIBUTION.md).
>
> **📝 Based on**: Fork of [cargo-doc-md](https://github.com/Crazytieguy/cargo-doc-md) by [Yoav Tzfati](https://github.com/Crazytieguy), modified to generate Docusaurus-compatible documentation.

A Cargo subcommand that generates Docusaurus-compatible markdown documentation for Rust crates. Like `cargo doc`, but outputs markdown files with React components instead of HTML.

## Installation

```bash
cargo install cargo-doc-docusaurus
```

## Quick Start

```bash
# 1. Install Docusaurus components (one-time setup)
cd your-docusaurus-site
cargo doc-docusaurus components init .

# 2. Generate rustdoc JSON (requires Rust nightly)
RUSTDOCFLAGS="-Z unstable-options --output-format json" \
  cargo +nightly doc --no-deps

# 3. Convert to Docusaurus markdown
cargo doc-docusaurus target/doc/my_crate.json \
  --output docs/api \
  --base-path "/docs/api"

# 4. Build your Docusaurus site
npm start
```

## Features

- **Automated component installation** - Install React components and CSS with one command
- **Multi-file output** - One markdown file per item (struct, enum, trait, function, module)
- **Docusaurus-ready** - Includes frontmatter, React components, and proper routing
- **Workspace-aware linking** - Internal links between workspace crates instead of docs.rs
- **Complete type signatures** - All items documented with clickable type links
- **Trait implementations** - Shows all trait impls with full method signatures
- **Module hierarchy** - Breadcrumb navigation and nested module structure
- **Re-export handling** - Items documented where re-exported, like `cargo doc`
- **Private items support** - Optionally include private items with `--include-private`

## Commands

### Component Management

Install and manage Docusaurus components (React, CSS, theme overrides):

```bash
# Install all components in a Docusaurus project
cargo doc-docusaurus components init <DOCUSAURUS_PATH>

# Update existing components to latest version
cargo doc-docusaurus components sync <DOCUSAURUS_PATH>

# List all available components and their installation status
cargo doc-docusaurus components list [DOCUSAURUS_PATH]
```

**Installed components:**
- `RustCode` - Syntax highlighting with clickable type links
- `RustCrateLink` - Crate titles with badges
- `RustModuleTitle` - Module titles for sidebar
- `DocSidebarItem/Link` - Theme override for colored sidebar links
- `rust-documentation.css` - All Rust documentation styles

### CSS Customization

Get customizable CSS variables for your `custom.css`:

```bash
# Show all available CSS variables with explanation
cargo doc-docusaurus components css-vars

# Output only CSS (for appending to file)
cargo doc-docusaurus components css-vars --css-only >> src/css/custom.css
```

This shows all color variables you can customize for light/dark mode, including:
- Module colors (gold)
- Struct/Enum colors (cyan)
- Trait colors (purple)
- Function colors (blue)
- Constant colors (brown)
- Macro colors (orange-red)

### JSON to Markdown Conversion

Convert rustdoc JSON files to Docusaurus markdown:

```bash
cargo doc-docusaurus <INPUT.json> [OPTIONS]
```

**Required:**
- `<INPUT.json>` - Path to rustdoc JSON file (e.g., `target/doc/my_crate.json`)

**Options:**

| Option | Description | Example |
|--------|-------------|---------|
| `-o, --output <DIR>` | Output directory | `--output docs/api` |
| `--base-path <PATH>` | Base URL path for links | `--base-path "/docs/api"` |
| `--include-private` | Include private items | `--include-private` |
| `--workspace-crates <CRATES>` | Comma-separated workspace crates for internal linking | `--workspace-crates "core,utils,server"` |
| `--sidebar-output <PATH>` | Custom sidebar file location | `--sidebar-output sidebars-rust.ts` |
| `--sidebar-root-link <URL>` | "Go back" link in crate sidebar | `--sidebar-root-link "/docs"` |
| `--sidebarconfig-collapsed` | Generate collapsed sidebar categories | `--sidebarconfig-collapsed` |

**Default values:**
- Output: `target/doc-md`
- Base path: none (uses relative links)
- Private items: excluded
- Workspace crates: none (all external links to docs.rs)
- Sidebar: auto-generated in output directory

## Usage Examples

### Single Crate

```bash
# Generate JSON
RUSTDOCFLAGS="-Z unstable-options --output-format json" \
  cargo +nightly doc --no-deps

# Convert to markdown
cargo doc-docusaurus target/doc/my_crate.json \
  -o docs/api \
  --base-path "/docs/api"
```

**Generated structure:**
```
docs/api/
  my_crate/
    index.md                    # Crate overview
    struct.MyStruct.md          # Struct documentation
    enum.MyEnum.md              # Enum documentation
    fn.my_function.md           # Function documentation
    module1/
      index.md                  # Module overview
      trait.MyTrait.md          # Trait documentation
    sidebars-rust.ts            # Auto-generated sidebar config
```

### Multi-Crate Workspace

For workspaces with multiple crates that reference each other:

```bash
# Define workspace crates
WORKSPACE_CRATES="crate_a,crate_b,crate_c"

# Generate JSON for all crates
RUSTDOCFLAGS="-Z unstable-options --output-format json" \
  cargo +nightly doc --no-deps

# Convert each crate
for crate in crate_a crate_b crate_c; do
  cargo doc-docusaurus target/doc/${crate}.json \
    --output docs/api \
    --base-path "/docs/api" \
    --workspace-crates "$WORKSPACE_CRATES"
done
```

**Result:** When `crate_a` references a type from `crate_b`, the link will be `/docs/api/crate_b/struct.MyType` instead of `https://docs.rs/crate_b/...`.

### Including Private Items

```bash
# Generate JSON with private items
RUSTDOCFLAGS="-Z unstable-options --output-format json" \
  cargo +nightly doc --no-deps --document-private-items

# Convert including private items
cargo doc-docusaurus target/doc/my_crate.json \
  -o docs/api \
  --include-private \
  --base-path "/docs/api"
```

### Custom Sidebar Location

```bash
# Generate sidebar in custom location
cargo doc-docusaurus target/doc/my_crate.json \
  -o docs/api \
  --sidebar-output sidebars-api.ts \
  --sidebar-root-link "/docs"
```

Then import in `docusaurus.config.ts`:

```typescript
import { rustApiDocumentation } from './sidebars-api';

export default {
  themeConfig: {
    navbar: {
      items: [
        {
          type: 'docSidebar',
          sidebarId: 'rustApiDocumentation',
          position: 'left',
          label: 'Rust API',
        },
      ],
    },
  },
  presets: [
    [
      'classic',
      {
        docs: {
          sidebarPath: './sidebars-api.ts',
        },
      },
    ],
  ],
};
```

### Automation with Justfile

Create a `justfile` for repeatable documentation builds:

```just
# Configuration
WORKSPACE_CRATES := "core,utils,server,client"
DOCS_BASE := "/docs/api"
DOCS_OUTPUT := "docusaurus-site/docs/api"

# Generate all documentation
docs: json markdown

# Generate rustdoc JSON
json:
  RUSTDOCFLAGS="-Z unstable-options --output-format json" \
    cargo +nightly doc --no-deps --document-private-items

# Convert to Docusaurus markdown
markdown:
  cargo doc-docusaurus target/doc/core.json \
    -o {{DOCS_OUTPUT}} \
    --base-path {{DOCS_BASE}} \
    --workspace-crates {{WORKSPACE_CRATES}} \
    --include-private
  cargo doc-docusaurus target/doc/utils.json \
    -o {{DOCS_OUTPUT}} \
    --base-path {{DOCS_BASE}} \
    --workspace-crates {{WORKSPACE_CRATES}} \
    --include-private
  cargo doc-docusaurus target/doc/server.json \
    -o {{DOCS_OUTPUT}} \
    --base-path {{DOCS_BASE}} \
    --workspace-crates {{WORKSPACE_CRATES}} \
    --include-private
  cargo doc-docusaurus target/doc/client.json \
    -o {{DOCS_OUTPUT}} \
    --base-path {{DOCS_BASE}} \
    --workspace-crates {{WORKSPACE_CRATES}} \
    --include-private

# Clean generated files
clean:
  rm -rf target/doc/*.json
  rm -rf {{DOCS_OUTPUT}}
```

Run with:
```bash
just docs      # Generate everything
just json      # Only generate JSON
just markdown  # Only convert to markdown
just clean     # Clean generated files
```

## Output Format

Each markdown file includes:

**Frontmatter:**
```yaml
---
title: "Struct MyStruct"
---
```

**React Component Imports:**
```jsx
import RustCode from '@site/src/components/RustCode';
import RustCrateLink from '@site/src/components/RustCrateLink';
```

**Type Signatures:**
```jsx
<RustCode>
pub struct MyStruct<T> {
    pub field: T,
}
</RustCode>
```

**Clickable Links:**
All type references are clickable links to their documentation pages.

## Setup Guide

For complete setup instructions including:
- Creating a new Docusaurus site
- Component installation details
- CSS customization
- Troubleshooting

See **[SETUP.md](SETUP.md)**.

## Development

This project uses snapshot testing with `insta`:

```bash
# Run all tests
cargo test

# Review snapshot changes
cargo insta review
```

See [tests/README.md](tests/README.md) for more information.

## Use Cases

- **API documentation sites** - Professional documentation for Rust libraries
- **Multi-crate workspaces** - Document entire project with cross-references
- **LLM context** - Provide API documentation as context for AI code generation
- **Version control** - Markdown files can be committed and reviewed in PRs
- **Offline documentation** - Self-contained docs that work without internet
- **Custom workflows** - Integrate with any static site generator

## License

MIT or Apache-2.0
