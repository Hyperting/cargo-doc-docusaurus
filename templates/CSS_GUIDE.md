# CSS Customization Guide

Customize Rust documentation colors in Docusaurus.

## Quick Start

### 1. Get CSS Variables

```bash
# Show all available variables
cargo doc-docusaurus components css-vars

# Or append to custom.css
cargo doc-docusaurus components css-vars --css-only >> src/css/custom.css
```

## Available Variables

```css
/* Item colors (base + hover) */
--rust-color-module
--rust-color-module-hover
--rust-color-struct
--rust-color-struct-hover
--rust-color-trait
--rust-color-trait-hover
--rust-color-function
--rust-color-function-hover
--rust-color-constant
--rust-color-constant-hover
--rust-color-macro
--rust-color-macro-hover

/* Spacing */
--rust-item-spacing: 0.35em
--rust-field-indent: 1.5em
```

## Default Colors

| Type | Light Mode | Dark Mode |
|------|------------|-----------|
| Module | `rgb(173, 124, 0)` | `rgb(253, 214, 135)` |
| Struct | `rgb(0, 125, 119)` | `rgb(0, 168, 160)` |
| Trait | `rgb(110, 79, 201)` | `rgb(183, 140, 255)` |
| Function | `rgb(56, 115, 173)` | `rgb(88, 166, 255)` |
| Constant | `rgb(139, 69, 19)` | `rgb(210, 180, 140)` |
| Macro | `rgb(184, 33, 148)` | `rgb(255, 120, 220)` |

## Examples

### Monochrome Theme

```css
:root {
  --rust-color-module: rgb(80, 80, 80);
  --rust-color-struct: rgb(60, 60, 60);
  --rust-color-function: rgb(100, 100, 100);
}

[data-theme='dark'] {
  --rust-color-module: rgb(200, 200, 200);
  --rust-color-struct: rgb(220, 220, 220);
  --rust-color-function: rgb(180, 180, 180);
}
```

### High Contrast Theme

```css
:root {
  --rust-color-module: rgb(255, 0, 0);
  --rust-color-struct: rgb(0, 255, 0);
  --rust-color-function: rgb(0, 0, 255);
}
```
