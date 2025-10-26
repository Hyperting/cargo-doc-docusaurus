# RustModuleTitle Component

A custom React component for rendering Rust module titles in Docusaurus sidebars with rustdoc-style formatting.

## Purpose

This component displays module names prominently with extra vertical spacing, matching rustdoc's style for module titles in the sidebar. Unlike `RustCrateTitle`, it doesn't show version information.

## Usage

This component is used by the swizzled `DocSidebarItem/Link` component when it detects `customProps.rustModuleTitle`:

```typescript
{
  type: 'doc',
  id: 'runtime/rust/my_crate/my_module/index',
  label: 'my_module',
  customProps: {
    rustModuleTitle: true,
    moduleName: 'my_module'
  }
}
```

## Props

- `href` (string, required): The link URL
- `moduleName` (string, required): The module name to display
- `children` (ReactNode, optional): Additional content

## Styling

The component uses CSS modules from `styles.module.css`:

- `.rustModuleLink`: Container with padding, margins, and hover effects
- `.moduleName`: Module name text styling (bold, slightly larger font)

## Installation

See the main [DocSidebarItemLink README](../DocSidebarItemLink/README.md) for installation instructions.

## Customization

Edit `styles.module.css` to customize:
- Font size and weight
- Padding and margins
- Border and hover effects
- Dark/light theme colors

## Example

```jsx
<RustModuleTitle
  href="/docs/rust/my_crate/my_module/index"
  moduleName="my_module"
/>
```

This renders as:

```
┌─────────────────┐
│ my_module       │  ← Bold, slightly larger
└─────────────────┘
```
