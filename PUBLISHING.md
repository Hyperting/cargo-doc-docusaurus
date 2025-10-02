# Publishing Guide

This document explains how to publish `rustdoc-to-markdown` to crates.io and set up the GitHub repository.

## Prerequisites

1. **Cargo account**: Create an account at https://crates.io
2. **GitHub account**: For hosting the repository
3. **API token**: Get from https://crates.io/me

## Step 1: Set Up GitHub Repository

```bash
# Initialize GitHub repo (if not already done)
gh repo create rustdoc-to-markdown --public --source=. --remote=origin

# Or manually:
# 1. Create repo on GitHub: https://github.com/new
# 2. Add remote:
git remote add origin https://github.com/YOUR_USERNAME/rustdoc-to-markdown.git

# Update Cargo.toml with correct repository URL
# Replace OWNER with your GitHub username in:
# - repository = "https://github.com/OWNER/rustdoc-to-markdown"
# - homepage = "https://github.com/OWNER/rustdoc-to-markdown"

# Push to GitHub
git push -u origin master
```

## Step 2: Prepare for Publishing

```bash
# Verify package builds
cargo build --release

# Run tests
cargo test

# Check package contents
cargo package --list

# Do a dry-run of publishing
cargo package --allow-dirty

# This creates a .crate file in target/package/
# You can inspect it to make sure everything is included
```

## Step 3: Publish to crates.io

```bash
# Login to crates.io (one-time setup)
cargo login

# Publish!
cargo publish

# If you need to allow uncommitted changes:
cargo publish --allow-dirty
```

## Step 4: Post-Publication

### Update README on GitHub

Make sure the repository URL in README.md is correct (replace OWNER with your username).

### Add Repository Topics

On GitHub, add topics to help discovery:
- `rustdoc`
- `markdown`
- `documentation`
- `llm`
- `ai-generated`
- `rust`
- `cli`

### Create a Release

```bash
# Tag the release
git tag -a v0.1.0 -m "Initial release: MVP with impl block support"
git push origin v0.1.0

# Or use GitHub releases UI:
# https://github.com/YOUR_USERNAME/rustdoc-to-markdown/releases/new
```

## Step 5: Announce

Consider announcing on:
- **Reddit**: r/rust (with "AI-generated" flair if available)
- **This Week in Rust**: Submit via https://github.com/rust-lang/this-week-in-rust
- **Rust Users Forum**: https://users.rust-lang.org/

**Important**: Be transparent that this is an AI-generated project! The ATTRIBUTION.md file should be prominently linked.

## Version Updates

For future updates:

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md` (create one if needed)
3. Commit changes
4. Tag release: `git tag -a v0.X.Y -m "Release message"`
5. Push: `git push && git push --tags`
6. Publish: `cargo publish`

## Troubleshooting

### "crate name already taken"

If the name is taken, you'll need to:
1. Choose a different name (e.g., `rustdoc-md-converter`, `rdoc-to-md`, etc.)
2. Update in `Cargo.toml`
3. Update all documentation references
4. Try again

### "package size too large"

The package includes only source files, not test artifacts:
- Make sure `.gitignore` excludes `target/`, test outputs, etc.
- Check with `cargo package --list`
- Add exclusions to `Cargo.toml` if needed:
  ```toml
  exclude = ["test-crate/", "test-output/", "*.md.backup"]
  ```

### "missing fields"

All required fields should be present in `Cargo.toml`:
- `name`, `version`, `authors`, `edition`
- `license` or `license-file`
- `description`
- `repository` (recommended)

## Maintenance

As the AI model that created this, I won't be maintaining it long-term. The human collaborator may:
- Accept PRs from the community
- Update dependencies as needed
- Add features based on user feedback
- Pass maintainership to interested contributors

## Questions?

If you're the human collaborator and have questions about this process:
1. Check Cargo documentation: https://doc.rust-lang.org/cargo/
2. See crates.io publishing guide: https://doc.rust-lang.org/cargo/reference/publishing.html
3. Ask in #cargo on Rust Discord or users.rust-lang.org

---

**Note**: Remember to update the repository URLs in Cargo.toml before publishing!
