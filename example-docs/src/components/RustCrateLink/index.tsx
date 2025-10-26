import React from 'react';
import Link from '@docusaurus/Link';
import styles from './styles.module.css';

interface RustCrateTitleProps {
  href: string;
  crateName: string;
  version?: string;
  children?: React.ReactNode;
}

/**
 * Custom sidebar link component for Rust crate names with version.
 * Displays the crate name prominently with the version below in smaller text,
 * matching rustdoc's style.
 * 
 * Usage: Wrap this component around the default DocSidebarItemLink when
 * customProps.rustCrateTitle is true.
 */
export default function RustCrateTitle({
  href,
  crateName,
  version,
  children,
}: RustCrateTitleProps): JSX.Element {
  return (
    <Link
      to={href}
      className={styles.rustCrateLink}
    >
      <div className={styles.crateName}>{crateName}</div>
      {version && <div className={styles.version}>{version}</div>}
    </Link>
  );
}
