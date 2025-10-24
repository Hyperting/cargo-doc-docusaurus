import React from 'react';
import { useLocation } from '@docusaurus/router';
import DocSidebarOriginal from '@theme-original/DocSidebar';
import { rustSidebars } from '../../../sidebars-rust';
import type { Props } from '@theme/DocSidebar';

/**
 * Dynamic Rust documentation sidebar component.
 * 
 * This component provides rustdoc-like behavior where each module shows its own
 * contextual sidebar instead of a single flat sidebar for all documentation.
 * 
 * How it works:
 * 1. Detects the current page path
 * 2. Looks up the corresponding sidebar in rustSidebars
 * 3. If found, renders that specific sidebar with navigation header
 * 4. Otherwise, uses the default sidebar
 */
export default function RustDocSidebar(props: Props): JSX.Element {
  const location = useLocation();
  const { sidebar } = props;
  
  // Extract the doc path from the current location
  // Handles both module pages (/docs/.../module/index) and item pages (/docs/.../struct.Item)
  const getDocPath = (): string | null => {
    const pathname = location.pathname;
    
    // Remove /docs/ prefix and any trailing /
    const docPath = pathname.replace(/^\/docs\//, '').replace(/\/$/, '');
    
    // Remove the final page name to get the module path
    const parts = docPath.split('/');
    
    // If we're on a specific doc page (struct, enum, etc.), remove it to get the module path
    if (parts.length > 0 && parts[parts.length - 1].match(/^(struct|enum|fn|trait|type|constant|macro)\./)) {
      parts.pop();
    }
    
    // Also handle index pages
    if (parts.length > 0 && parts[parts.length - 1] === 'index') {
      parts.pop();
    }
    
    return parts.join('/');
  };
  
  // Find the best matching sidebar for the current path
  const findBestSidebar = (): string | null => {
    const docPath = getDocPath();
    if (!docPath) return null;
    
    // Count the path segments to determine depth
    const segments = docPath.split('/');
    
    // Only use custom sidebar if we're at least 4 segments deep (base/path/crate/module)
    // This preserves the root sidebar for:
    // - /docs/base/path (2 segments) 
    // - /docs/base/path/crate (3 segments) <- crate root, should use rootRustSidebar
    if (segments.length < 4) {
      return null; // Use default sidebar
    }
    
    // Try exact match first
    if (rustSidebars[docPath]) {
      return docPath;
    }
    
    // Try progressively shorter paths (parent modules)
    for (let i = segments.length - 1; i >= 4; i--) {
      const partialPath = segments.slice(0, i).join('/');
      if (rustSidebars[partialPath]) {
        return partialPath;
      }
    }
    
    return null;
  };
  
  const sidebarKey = findBestSidebar();

  
  // If we found a specific Rust sidebar for this path, use it
  if (sidebarKey && rustSidebars[sidebarKey]) {
    const rustItems = rustSidebars[sidebarKey];
    
    // Ensure rustItems is an array
    if (Array.isArray(rustItems)) {
      // Add a breadcrumb/back link at the top
      const moduleName = sidebarKey.split('/').pop() || sidebarKey;
      const parentPath = sidebarKey.split('/').slice(0, -1).join('/');
      
      const sidebarWithHeader = [
        {
          type: 'link',
          label: '← Go back',
          href: `/docs/${parentPath}`,
          className: 'rust-sidebar-back-link',
        },
        {
          type: 'html',
          value: `<div class="rust-sidebar-module-title">${moduleName}</div>`,
          defaultStyle: true,
        },
        ...rustItems,
      ];
      
      return <DocSidebarOriginal {...props} sidebar={sidebarWithHeader} />;
    }
  }
  
  // Otherwise, use the default sidebar (which includes rootRustSidebar for root paths)
  return <DocSidebarOriginal {...props} />;
}
