# Rustdoc-to-Markdown Implementation Plan

## Project Goal
Convert rustdoc's JSON output to markdown format for convenient LLM consumption and documentation viewing.

## Architecture Overview

```
rustdoc JSON → Parser → Converter → Markdown Writer → Files
```

## Key Components

### 1. CLI Interface (`main.rs`)
- Parse command-line arguments:
  - Input: JSON file path (required)
  - Output: directory path (default: `./docs`)
  - Format options: single-file vs multi-file, table-of-contents, etc.
- Load and validate JSON file
- Orchestrate conversion process

**CLI Usage:**
```bash
rustdoc-to-markdown <input.json> -o <output-dir>
rustdoc-to-markdown --help
```

### 2. JSON Parser (`parser.rs`)
- Load rustdoc JSON using `rustdoc-json-types::Crate`
- Validate format version compatibility
- Extract items from the index

### 3. Markdown Converter (`converter.rs`)
- Core conversion logic for each item type:
  - **Module** → Section with sub-items
  - **Struct** → Heading + fields table + methods
  - **Enum** → Heading + variants list
  - **Function** → Signature + docs
  - **Trait** → Heading + required methods
  - **Type Alias** → Definition
  - **Const/Static** → Declaration + docs

### 4. Formatters (`formatters/`)
- `signature.rs` - Convert type signatures to markdown code blocks
- `docs.rs` - Handle doc comments (already markdown, minimal processing)
- `links.rs` - Convert cross-references to markdown links
- `code.rs` - Format Rust code with proper fencing

### 5. Writer (`writer.rs`)
- File organization strategies:
  - **Single file**: All items in one `docs.md`
  - **Per-module**: Mirror crate structure
  - **Per-item**: One file per struct/enum/etc.
- Generate table of contents
- Handle file I/O

## Implementation Phases

### Phase 1: MVP (Minimal Viable Product)
**Goal:** Basic working converter for simple items

1. Parse JSON into `rustdoc-json-types::Crate`
2. Iterate through crate.index
3. Convert simple items (functions, structs, enums) to markdown
4. Output to single file
5. Basic CLI with input/output paths

**Success Criteria:**
- Can convert a simple crate to readable markdown
- Includes doc comments and basic signatures

### Phase 2: Enhanced Formatting
**Goal:** Better structure and readability

1. Implement proper heading hierarchy
2. Format type signatures cleanly
3. Add tables for struct fields
4. Generate table of contents
5. Support multi-file output

### Phase 3: Cross-References
**Goal:** Working internal links

1. Build ID→path mapping from crate.paths
2. Convert rustdoc IDs to markdown links
3. Handle external crate references
4. Add "See also" sections

### Phase 4: Advanced Features
**Goal:** Production-ready tool

1. Format version compatibility checking
2. External crate links (using html_root_url)
3. Trait implementations section
4. Source code links (if available)
5. Filter by visibility (pub vs private)
6. Configuration file support

## Data Structures

### Key Types from rustdoc-json-types
```rust
Crate {
    root: Id,
    index: HashMap<Id, Item>,
    paths: HashMap<Id, ItemSummary>,
    external_crates: HashMap<u32, ExternalCrate>,
    ...
}

Item {
    id: Id,
    name: Option<String>,
    docs: Option<String>,  // Already markdown!
    kind: ItemKind,
    ...
}
```

### Internal Types (to define)
```rust
struct MarkdownOutput {
    path: PathBuf,
    content: String,
}

struct ConversionContext {
    crate_data: Crate,
    id_to_path: HashMap<Id, String>,  // For cross-refs
}
```

## Testing Strategy

1. **Unit tests**: Individual converters for each item type
2. **Integration tests**: Full JSON → Markdown conversion
3. **Fixtures**: Sample JSON files from real crates
4. **Golden files**: Expected markdown output for comparison

## Future Enhancements

- Custom templates (user-defined markdown structure)
- Mermaid diagrams for trait hierarchies
- Search index generation
- HTML fallback for complex items
- Watch mode for live updates
- Cargo subcommand: `cargo markdown-doc`

## File Structure

```
rustdoc-to-markdown/
├── src/
│   ├── main.rs              # CLI entry point
│   ├── parser.rs            # JSON loading
│   ├── converter.rs         # Core conversion logic
│   ├── writer.rs            # File output
│   ├── formatters/
│   │   ├── mod.rs
│   │   ├── signature.rs     # Type signature formatting
│   │   ├── docs.rs          # Doc comment processing
│   │   ├── links.rs         # Cross-reference handling
│   │   └── code.rs          # Code block formatting
│   └── lib.rs               # Public API
├── tests/
│   ├── fixtures/            # Sample JSON files
│   └── integration.rs
├── Cargo.toml
├── PLAN.md                  # This file
└── README.md
```

## Dependencies

- `rustdoc-json-types` - Type definitions for rustdoc JSON
- `serde_json` - JSON parsing
- `anyhow` - Error handling
- `clap` - CLI argument parsing

## Open Questions

1. **File organization**: Single vs multi-file? User preference?
2. **Link format**: `[Struct](path/to/struct.md)` or `[Struct](#struct)`?
3. **Code highlighting hints**: Preserve `rust` fence language?
4. **Visibility filtering**: Include private items by default?

## Timeline Estimate

- Phase 1 (MVP): 2-3 days
- Phase 2 (Formatting): 2-3 days
- Phase 3 (Links): 2-3 days
- Phase 4 (Polish): 3-5 days

**Total:** ~2 weeks for production-ready v1.0
