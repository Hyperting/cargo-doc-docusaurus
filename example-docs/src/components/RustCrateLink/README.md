# RustCrateLink Component

A custom React component for displaying Rust crate names with versions in Docusaurus sidebars, matching rustdoc's style.

## Installation

Copy the entire `RustCrateLink` directory to your Docusaurus project:

```bash
cp -r templates/RustCrateLink src/components/
```

## Usage in Sidebar Configuration

Instead of using plain doc items for crate links, use the custom component:

### Before (plain sidebar item)
```typescript
{
  type: 'doc',
  id: 'runtime/rust/my_crate/index',
  label: 'my_crate 0.1.0',
  className: 'rust-sidebar-crate-link'
}
```

### After (using custom component)
```typescript
{
  type: 'html',
  value: '<RustCrateLink to="/docs/runtime/rust/my_crate" crateName="my_crate" version="0.1.0" />',
  defaultStyle: true
}
```

## Component Props

- `to` (string, required): The link destination path
- `crateName` (string, required): The name of the crate to display
- `version` (string, optional): The version number to display below the crate name
- `className` (string, optional): Additional CSS classes to apply

## Visual Style

The component displays:
```
my_crate
0.1.0
─────────
```

With:
- Crate name in bold, larger font
- Version in smaller, lighter font below
- Border bottom separator
- Hover effect with background highlight

## Customization

Edit `styles.module.css.txt` to customize:
- Font sizes
- Colors
- Spacing
- Border styles
- Hover effects
