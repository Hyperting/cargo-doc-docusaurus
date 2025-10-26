import React from 'react';
import Link from '@docusaurus/Link';
import styles from './styles.module.css';

interface RustModuleTitleProps {
  href: string;
  moduleName: string;
  children?: React.ReactNode;
}

/**
 * Custom sidebar link component for Rust module names.
 * Displays the module name prominently with extra vertical spacing,
 * matching rustdoc's style for module titles in the sidebar.
 * 
 * Usage: Wrap this component around the default DocSidebarItemLink when
 * customProps.rustModuleTitle is true.
 */
export default function RustModuleTitle({
  href,
  moduleName,
  children,
}: RustModuleTitleProps): JSX.Element {
  return (
    <Link
      to={href}
      className={styles.rustModuleLink}
    >
      <div className={styles.moduleName}>{moduleName}</div>
    </Link>
  );
}
