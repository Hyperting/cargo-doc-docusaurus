# Example Docusaurus Site

This is a test Docusaurus site for developing and testing `cargo-doc-md`.

## Setup

```bash
# Install dependencies
npm install

# Start development server
npm start
```

## Generate Documentation

To regenerate the example crate documentation:

```bash
# From the cargo-doc-md root directory
cd example-crate
cargo +nightly rustdoc --lib -- -Z unstable-options --output-format json

cd ..
cargo-doc-docusaurus --base-path "/docs" example-crate/target/doc/example_crate.json -o example-docs/docs
```

## Development Workflow

1. Make changes to converter or templates
2. Rebuild: `cargo build --release && cargo install --path .`
3. Regenerate docs (see above)
4. View changes: Keep `npm start` running for hot reload
