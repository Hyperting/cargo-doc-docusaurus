use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

pub fn write_markdown(output_dir: &Path, content: &str) -> Result<()> {
    fs::create_dir_all(output_dir)
        .with_context(|| format!("Failed to create output directory: {}", output_dir.display()))?;

    let output_file = output_dir.join("index.md");

    fs::write(&output_file, content)
        .with_context(|| format!("Failed to write file: {}", output_file.display()))?;

    Ok(())
}
