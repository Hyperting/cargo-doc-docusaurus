# DocSidebarItemLink - Swizzled Component

This is a swizzled version of the `DocSidebarItem/Link` component that adds support for custom Rust crate title rendering.

## Installation

### 1. Copy Components

```bash
# Copy the RustCrateLink component
cp -r templates/RustCrateLink src/components/

# Copy the swizzled DocSidebarItemLink component
cp -r templates/DocSidebarItemLink src/theme/DocSidebarItem/Link/
```

### 2. Rename Files

Remove the `.txt` extension from the copied files:

```bash
# In src/components/RustCrateLink/
mv index.tsx.txt index.tsx
mv styles.module.css.txt styles.module.css

# In src/theme/DocSidebarItem/Link/
mv index.tsx.txt index.tsx
mv styles.module.css.txt styles.module.css
```

## How It Works

The component checks for `customProps.rustCrateTitle` in sidebar items:

```typescript
{
  type: 'doc',
  id: 'runtime/rust/my_crate/index',
  label: 'my_crate',
  customProps: {
    rustCrateTitle: true,
    crateName: 'my_crate',
    version: '0.1.0'
  }
}
```

When this prop is present, it renders the crate name and version using the `RustCrateTitle` component instead of the default link rendering.

## Visual Result

```
┌─────────────────┐
│ my_crate        │
│ 0.1.0           │  ← Version smaller and faded
├─────────────────┤
│ Overview        │
│ ...             │
└─────────────────┘
```

## Customization

Edit `src/components/RustCrateLink/styles.module.css` to customize the appearance.

## Automatic Generation

When you run `cargo-doc-docusaurus`, it will automatically generate sidebar items with the correct `customProps` for crate root links in module sidebars.
