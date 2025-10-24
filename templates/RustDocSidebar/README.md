# RustDoc Dynamic Sidebar Component

This component provides rustdoc-like sidebar behavior in Docusaurus, where each module shows its own contextual sidebar.

## Installation

1. Copy this component to your Docusaurus project:
   ```bash
   cp -r templates/RustDocSidebar src/theme/DocSidebar
   ```

2. Rename the file:
   ```bash
   mv src/theme/DocSidebar/index.tsx.txt src/theme/DocSidebar/index.tsx
   ```

## How it works

The component:
1. Detects the current page path (e.g., `/docs/runtime/rust/a2a/agent_card`)
2. Extracts the module path (`runtime/rust/a2a/agent_card`)
3. Looks up the corresponding sidebar in `rustSidebars`
4. If found, renders that specific sidebar
5. If not found, tries parent module sidebars
6. Falls back to default sidebar if no Rust sidebar matches

## Example

When viewing `/docs/runtime/rust/a2a/agent_card/struct.AgentCard`:
- The component detects you're in the `agent_card` module
- It loads `rustSidebars['runtime/rust/a2a/agent_card']`
- Shows only the items relevant to that module

This creates a navigation experience similar to rustdoc, where the sidebar changes based on your current location in the documentation tree.

## Requirements

- Docusaurus v3+
- Generated `sidebars-rust.ts` file with `rustSidebars` export
- Theme swizzling enabled (component uses theme overrides)
