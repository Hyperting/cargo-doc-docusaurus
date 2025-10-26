# CSS Guide for Rust Documentation Styling

This guide explains the CSS architecture for styling Rust documentation in Docusaurus.

## Architecture

Two CSS files work together:

1. **`rust-documentation.css`** - All Rust-specific styles (colors, syntax highlighting, sidebar)
2. **`custom.css`** - Docusaurus theme + optional color overrides

## CSS Variables

### Default Colors (Light Mode)

| Item Type | Color | Usage |
|-----------|-------|-------|
| Module | Gold `rgb(173, 124, 0)` | Module links |
| Struct | Cyan `rgb(0, 125, 119)` | Struct/enum links |
| Trait | Purple `rgb(110, 79, 201)` | Trait links |
| Function | Blue `rgb(56, 115, 173)` | Function links |

### Default Colors (Dark Mode)

| Item Type | Color | Usage |
|-----------|-------|-------|
| Module | Light Gold `rgb(253, 214, 135)` | Module links |
| Struct | Light Cyan `rgb(0, 168, 160)` | Struct/enum links |
| Trait | Light Purple `rgb(183, 140, 255)` | Trait links |
| Function | Light Blue `rgb(88, 166, 255)` | Function links |

## Customization

Add to `src/css/custom.css`:

```css
:root {
  /* Light mode */
  --rust-color-module: rgb(255, 140, 0);      /* Orange modules */
  --rust-color-struct: rgb(34, 139, 34);      /* Forest green structs */
  --rust-color-trait: rgb(220, 20, 60);       /* Crimson traits */
  --rust-color-function: rgb(70, 130, 180);   /* Steel blue functions */
}

[data-theme='dark'] {
  /* Dark mode - use lighter versions */
  --rust-color-module: rgb(255, 200, 100);
  --rust-color-struct: rgb(144, 238, 144);
  --rust-color-trait: rgb(255, 105, 180);
  --rust-color-function: rgb(135, 206, 250);
}
```

### Available Variables

- `--rust-color-module` / `--rust-color-module-hover`
- `--rust-color-struct` / `--rust-color-struct-hover`
- `--rust-color-trait` / `--rust-color-trait-hover`
- `--rust-color-function` / `--rust-color-function-hover`
- `--rust-item-spacing` (default: `0.35em`)
- `--rust-field-indent` (default: `1.5em`)

## Troubleshooting

#### Colors Not Showing

**Problem**: Sidebar or content links are default blue instead of colored

**Checklist**:
1. ✅ `rust-documentation.css` loaded (check `docusaurus.config.ts` or `custom.css`)
2. ✅ CSS file copied to correct location (`static/css/` or `src/css/`)
3. ✅ Elements have correct classes (e.g., `rust-mod`, `rust-struct`)
4. ✅ `DocSidebarItem/Link` component applies `className` to links

**Debug in browser:**
```javascript
// Check if element has class
document.querySelector('a.rust-mod')

// Check computed color  
getComputedStyle(document.querySelector('a.rust-mod')).color
// Should be rgb(173, 124, 0), not rgb(0, 0, 255)
```

### Dark Mode Colors Wrong

Check `[data-theme='dark']` block exists with proper colors. Dark mode needs lighter colors for contrast.

### Hover Not Working

### Hover Not Working

Verify hover variables are defined:
```css
:root {
  --rust-color-module-hover: rgb(200, 140, 0);
}
```

## Best Practices

## Customization Examples

### Custom Color Scheme

Edit CSS variables in `rust-documentation.css`:

```css
[data-theme='light'] {
  --rust-color-module: rgb(255, 140, 0);      /* Orange modules */
  --rust-color-module-hover: rgb(255, 165, 0);
  --rust-color-struct: rgb(34, 139, 34);      /* Forest green structs */
  --rust-color-struct-hover: rgb(50, 205, 50);
  --rust-color-trait: rgb(220, 20, 60);       /* Crimson traits */
  --rust-color-trait-hover: rgb(255, 69, 96);
  --rust-color-function: rgb(70, 130, 180);   /* Steel blue functions */
  --rust-color-function-hover: rgb(100, 149, 237);
}

[data-theme='dark'] {
  /* Lighter versions for dark background */
  --rust-color-module: rgb(255, 200, 100);
  --rust-color-module-hover: rgb(255, 220, 150);
  /* ... etc ... */
}
```

## Best Practices

- Use CSS variables (don't hard-code colors)
- Update both light and dark themes
- Test contrast (WCAG AA: 4.5:1 ratio minimum)
- Provide hover feedback

## References

- [Docusaurus Styling](https://docusaurus.io/docs/styling-layout)
- [CSS Variables](https://developer.mozilla.org/en-US/docs/Web/CSS/Using_CSS_custom_properties)
- [WCAG Contrast](https://www.w3.org/WAI/WCAG21/Understanding/contrast-minimum.html)
