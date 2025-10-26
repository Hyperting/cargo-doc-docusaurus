# Complete Setup Guide for cargo-doc-docusaurus

This guide walks you through setting up `cargo-doc-docusaurus` with Docusaurus from scratch.

## Prerequisites

- **Rust nightly** (required for rustdoc JSON output)
- **Node.js 18+** and npm/yarn
- **Docusaurus 3.x** site (or create a new one)
- **Your Rust project** (single crate or workspace)

```bash
# Check versions
rustc --version  # Should show "nightly"
node --version   # Should be 18.x or higher
```

## Installation

### 1. Install cargo-doc-docusaurus

```bash
cargo install cargo-doc-docusaurus
```

### 2. Verify Installation

```bash
cargo doc-docusaurus --help
```

You should see the help output with all available options.

## Docusaurus Setup

### Option A: Existing Docusaurus Site

Skip to [Component Installation](#component-installation).

### Option B: Create New Docusaurus Site

```bash
npx create-docusaurus@latest my-docs classic
cd my-docs
npm install
```

Test the site:

```bash
npm start
# Open http://localhost:3000
```

## Component Installation

Install all required React components and theme overrides.

### 1. Create Directory Structure

```bash
cd my-docs  # Your Docusaurus root

# Create component directories
mkdir -p src/components/RustCode
mkdir -p src/components/RustCrateLink
mkdir -p src/components/RustModuleTitle
mkdir -p src/theme/DocSidebarItem/Link
mkdir -p src/css
```

### 2. Copy Components from Templates

From your `cargo-doc-docusaurus` repository:

```bash
# Set paths
TEMPLATES="/path/to/cargo-doc-md/templates"
DOCUSAURUS="/path/to/my-docs"

# RustCode component
cp "$TEMPLATES/RustCode/index.tsx.txt" "$DOCUSAURUS/src/components/RustCode/index.tsx"
cp "$TEMPLATES/RustCode/styles.module.css.txt" "$DOCUSAURUS/src/components/RustCode/styles.module.css"

# RustCrateLink component
cp "$TEMPLATES/RustCrateLink/index.tsx.txt" "$DOCUSAURUS/src/components/RustCrateLink/index.tsx"
cp "$TEMPLATES/RustCrateLink/styles.module.css.txt" "$DOCUSAURUS/src/components/RustCrateLink/styles.module.css"

# RustModuleTitle component
cp "$TEMPLATES/RustModuleTitle/index.tsx.txt" "$DOCUSAURUS/src/components/RustModuleTitle/index.tsx"
cp "$TEMPLATES/RustModuleTitle/styles.module.css.txt" "$DOCUSAURUS/src/components/RustModuleTitle/styles.module.css"

# DocSidebarItem/Link (swizzled theme component)
cp "$TEMPLATES/DocSidebarItem/Link/index.tsx.txt" "$DOCUSAURUS/src/theme/DocSidebarItem/Link/index.tsx"
cp "$TEMPLATES/DocSidebarItem/Link/styles.module.css.txt" "$DOCUSAURUS/src/theme/DocSidebarItem/Link/styles.module.css"

# CSS files
cp "$TEMPLATES/rust-documentation.css.txt" "$DOCUSAURUS/src/css/rust-documentation.css"
```

### 3. Verify Structure

```bash
tree src/
```

Expected output:

```
src/
├── components/
│   ├── RustCode/
│   │   ├── index.tsx
│   │   └── styles.module.css
│   ├── RustCrateLink/
│   │   ├── index.tsx
│   │   └── styles.module.css
│   └── RustModuleTitle/
│       ├── index.tsx
│       └── styles.module.css
├── theme/
│   └── DocSidebarItem/
│       └── Link/
│           ├── index.tsx
│           └── styles.module.css
└── css/
    ├── custom.css
    └── rust-documentation.css
```

## CSS Configuration

### Load Rust Documentation CSS

All Rust-specific styles are included in `rust-documentation.css`. You just need to load it in your Docusaurus config.

**Option A: Via docusaurus.config.js (Recommended)**

Edit `docusaurus.config.js` (or `.ts`):

```javascript
module.exports = {
  // ... other config

  stylesheets: [
    {
      href: '/css/rust-documentation.css',
    },
  ],

  // ... rest of config
};
```

Then copy the file to your static directory:

```bash
cp "$TEMPLATES/rust-documentation.css.txt" "$DOCUSAURUS/static/css/rust-documentation.css"
```

**Option B: Via custom.css import**

Alternatively, import in `src/css/custom.css`:

```css
@import "./rust-documentation.css";

/* Your custom Docusaurus styles below */
:root {
  --ifm-color-primary: #2e8555;
  /* ... other Docusaurus variables ... */
}
```

Then copy to src:

```bash
cp "$TEMPLATES/rust-documentation.css.txt" "$DOCUSAURUS/src/css/rust-documentation.css"
```

### Optional: Customize Colors

All colors are defined using CSS variables. To customize, add to your `custom.css`:

```css
:root {
  /* Light mode */
  --rust-color-module: rgb(255, 120, 0);     /* Orange modules */
  --rust-color-struct: rgb(0, 150, 200);     /* Light blue structs */
  --rust-color-trait: rgb(150, 50, 200);     /* Purple traits */
  --rust-color-function: rgb(0, 120, 180);   /* Blue functions */
}

[data-theme='dark'] {
  /* Dark mode */
  --rust-color-module: rgb(255, 180, 100);
  --rust-color-struct: rgb(100, 200, 250);
  --rust-color-trait: rgb(200, 150, 255);
  --rust-color-function: rgb(150, 220, 255);
}
```

See [`templates/CSS_GUIDE.md`](templates/CSS_GUIDE.md) for all available variables.

## Generating Documentation

### 1. Generate Rustdoc JSON

From your Rust project root:

```bash
# Single crate
RUSTC_BOOTSTRAP=1 RUSTDOCFLAGS="-Z unstable-options --output-format json" \
  cargo +nightly doc --no-deps --document-private-items
```

This creates `target/doc/your_crate.json`.

### 2. Convert to Docusaurus Markdown

#### Single Crate

```bash
cargo doc-docusaurus \
  --include-private \
  --base-path "/docs/api" \
  target/doc/your_crate.json \
  -o my-docs/docs/api
```

#### Workspace with Multiple Crates

```bash
# List all workspace crates
WORKSPACE_CRATES="crate_a,crate_b,crate_c"

# Generate docs for each crate
for crate in crate_a crate_b crate_c; do
  cargo doc-docusaurus \
    --include-private \
    --base-path "/docs/api" \
    --workspace-crates "$WORKSPACE_CRATES" \
    target/doc/${crate}.json \
    -o my-docs/docs/api
done
```

### 3. Import Generated Sidebar

The tool creates `sidebars-rust.ts` in your output directory.

**Option A: Auto-merge (Recommended)**

Add to `my-docs/sidebars.ts`:

```typescript
import type {SidebarsConfig} from '@docusaurus/plugin-content-docs';
import {rustSidebars} from './docs/api/sidebars-rust';

const sidebars: SidebarsConfig = {
  // Your existing sidebars
  tutorialSidebar: [{type: 'autogenerated', dirName: '.'}],
  
  // Merge Rust sidebars
  ...rustSidebars,
};

export default sidebars;
```

**Option B: Manual category**

```typescript
const sidebars: SidebarsConfig = {
  mySidebar: [
    {
      type: 'category',
      label: 'API Documentation',
      items: [{type: 'autogenerated', dirName: 'api'}],
    },
  ],
};
```

## Verification

### 1. Start Docusaurus

```bash
cd my-docs
npm start
```

### 2. Check Generated Pages

Navigate to:
- `http://localhost:3000/docs/api/your_crate` - Crate overview
- Click on modules, structs, functions in sidebar
- Verify colors:
  - 🟡 Modules (gold/orange)
  - 🔵 Structs (cyan)
  - 🟣 Traits (purple)
  - 🔵 Functions (blue)

### 3. Test Features

- ✅ Syntax highlighting in code blocks
- ✅ Clickable type links
- ✅ Sidebar navigation with colors
- ✅ Dark/light mode switching
- ✅ Breadcrumb navigation

## Customization

### Custom Sidebar Layout

Edit generated `sidebars-rust.ts` to reorganize items:

```typescript
export const rustSidebars = {
  rustSidebar: [
    {
      type: 'doc',
      id: 'api/my_crate/index',
      label: 'Overview',
    },
    {
      type: 'category',
      label: 'Core',
      items: [
        // Move important items here
      ],
    },
  ],
};
```

### Add Documentation Sections

Mix Rust docs with hand-written guides:

```typescript
const sidebars = {
  mySidebar: [
    {
      type: 'category',
      label: 'Getting Started',
      items: ['intro', 'quickstart'],
    },
    {
      type: 'category',
      label: 'API Reference',
      items: [{type: 'autogenerated', dirName: 'api'}],
    },
  ],
};
```

## Troubleshooting

### Components Not Found

**Error:** `Module not found: Can't resolve '@site/src/components/RustCode'`

**Solution:**
1. Verify files exist: `ls src/components/RustCode/`
2. Check filenames: `index.tsx` (not `.txt`)
3. Restart dev server: `npm start`

### CSS Not Loading

**Error:** Styles not applying, colors not showing

**Solution:**
1. Check `docusaurus.config.js` has `stylesheets: [{href: '/css/rust-documentation.css'}]`
2. Verify file exists: `ls src/css/rust-documentation.css` or `ls static/css/rust-documentation.css`
3. Clear cache: `npm run clear && npm start`

### Sidebar Colors Not Showing

**Solution:**
1. Check `DocSidebarItem/Link` component is installed
2. Verify `className` is set in sidebar config
3. Open DevTools, inspect link, should see `menu__link rust-mod`
4. Check CSS variables are defined in `rust-documentation.css`

### TypeScript Errors

**Error:** `Cannot find module '@docusaurus/Link'`

**Solution:**
```bash
npm install --save-dev @types/react @docusaurus/types @docusaurus/theme-common
```

### Build Fails

**Error:** Build succeeds but `npm run build` fails

**Solution:**
1. Check all imports resolve
2. Verify no missing dependencies
3. Test with `npm run clear && npm run build`

## Next Steps

- Customize colors (see `templates/CSS_GUIDE.md`)
- Set up CI/CD to auto-generate docs
- Deploy your site
