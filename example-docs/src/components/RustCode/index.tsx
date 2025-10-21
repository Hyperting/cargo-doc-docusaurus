import React, { useState, useEffect } from "react";
import { Highlight, themes } from "prism-react-renderer";
import Link from "@docusaurus/Link";
import styles from "./styles.module.css";

interface TypeLink {
  text: string;
  href: string;
}

interface RustCodeProps {
  code: string;
  links?: TypeLink[];
  inline?: boolean;
}

export default function RustCode({ code, links = [], inline = false }: RustCodeProps): React.JSX.Element {
  const [isDarkTheme, setIsDarkTheme] = useState(false);

  useEffect(() => {
    // Check initial theme
    const htmlElement = document.documentElement;
    setIsDarkTheme(htmlElement.getAttribute("data-theme") === "dark");

    // Watch for theme changes
    const observer = new MutationObserver(() => {
      setIsDarkTheme(htmlElement.getAttribute("data-theme") === "dark");
    });

    observer.observe(htmlElement, {
      attributes: true,
      attributeFilter: ["data-theme"],
    });

    return () => observer.disconnect();
  }, []);

  const theme = isDarkTheme ? themes.dracula : themes.github;

  // If no links, use standard highlighting
  if (links.length === 0) {
    return (
      <Highlight code={code} language="rust" theme={theme}>
        {({ className, style, tokens, getLineProps, getTokenProps }) => {
          // Inline mode: render as code block but without background
          if (inline) {
            return (
              <code className={styles.inlineCode}>
                {tokens.map((line, i) => (
                  <div key={i}>
                    {line.map((token, key) => (
                      <span key={key} {...getTokenProps({ token })} />
                    ))}
                  </div>
                ))}
              </code>
            );
          }

          // Block mode: render as pre block
          return (
            <pre className={className} style={style}>
              <code>
                {tokens.map((line, i) => (
                  <div key={i} {...getLineProps({ line })}>
                    {line.map((token, key) => (
                      <span key={key} {...getTokenProps({ token })} />
                    ))}
                  </div>
                ))}
              </code>
            </pre>
          );
        }}
      </Highlight>
    );
  }

  // Create a map of type names to their links
  const linkMap = new Map(links.map((link) => [link.text, link.href]));

  // With links: apply syntax highlighting and inject links
  return (
    <Highlight code={code} language="rust" theme={theme}>
      {({ className, style, tokens, getLineProps, getTokenProps }) => {
        // Inline mode with links
        if (inline) {
          return (
            <code className={styles.inlineCode}>
              {tokens.map((line, i) => (
                <div key={i}>
                  {line.map((token, key) => {
                    const tokenText = token.content;
                    const href = linkMap.get(tokenText);

                    // If this token matches a link, wrap it in a Link component
                    // We only link tokens that are types (class-name)
                    if (href && token.types.includes("class-name")) {
                      const tokenProps = getTokenProps({ token });
                      // Extract the color from the token's style
                      const tokenColor = tokenProps.style?.color;

                      return (
                        <Link key={key} href={href} className={styles.typeLink} style={{ color: tokenColor }}>
                          {token.content}
                        </Link>
                      );
                    }

                    // Otherwise, render normal highlighted token
                    return <span key={key} {...getTokenProps({ token })} />;
                  })}
                </div>
              ))}
            </code>
          );
        }

        // Block mode with links
        return (
          <pre className={className} style={style}>
            <code>
              {tokens.map((line, i) => (
                <div key={i} {...getLineProps({ line })}>
                  {line.map((token, key) => {
                    const tokenText = token.content;
                    const href = linkMap.get(tokenText);

                    // If this token matches a link, wrap it in a Link component
                    // We only link tokens that are types (class-name)
                    if (href && token.types.includes("class-name")) {
                      const tokenProps = getTokenProps({ token });
                      // Extract the color from the token's style
                      const tokenColor = tokenProps.style?.color;

                      return (
                        <Link key={key} href={href} className={styles.typeLink} style={{ color: tokenColor }}>
                          {token.content}
                        </Link>
                      );
                    }

                    // Otherwise, render normal highlighted token
                    return <span key={key} {...getTokenProps({ token })} />;
                  })}
                </div>
              ))}
            </code>
          </pre>
        );
      }}
    </Highlight>
  );
}
