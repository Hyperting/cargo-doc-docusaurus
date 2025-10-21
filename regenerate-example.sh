#!/bin/bash
# Regenerate example documentation

set -e

echo "🔨 Building cargo-doc-docusaurus..."
cargo build --release
cargo install --path .

echo ""
echo "📚 Generating rustdoc JSON for test_crate..."
cd tests/fixtures/test_crate
cargo +nightly rustdoc --lib -- -Z unstable-options --output-format json
cp target/doc/test_crate.json ../test_crate.json

echo ""
echo "📝 Converting to Docusaurus..."
cd ../../..
cargo-doc-docusaurus --base-path "/docs" tests/fixtures/test_crate.json -o example-docs/docs

echo ""
echo "📦 Copying template files to example-docs..."
# Copy RustCode component (removing .txt extension)
mkdir -p example-docs/src/components/RustCode
cp templates/RustCode/index.tsx.txt example-docs/src/components/RustCode/index.tsx
cp templates/RustCode/styles.module.css.txt example-docs/src/components/RustCode/styles.module.css

# Copy CSS to static folder (removing .txt extension)
mkdir -p example-docs/static/css
cp templates/rust-documentation.css.txt example-docs/static/css/rust-documentation.css

# Copy custom.css template to src/css (removing .txt extension)
mkdir -p example-docs/src/css
cp templates/custom.css.txt example-docs/src/css/custom.css

echo ""
echo "✅ Done! Documentation generated in example-docs/docs/test_crate/"
echo ""
echo "To view: cd example-docs && npm start"
