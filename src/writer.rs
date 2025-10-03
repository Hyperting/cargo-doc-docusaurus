//! Markdown file writer.

use anyhow::{Context, Result};
use std::fs;
use std::path::Path;
use crate::converter::MarkdownOutput;

/// Write markdown content to a file in the specified directory.
pub fn write_markdown(output_dir: &Path, content: &str) -> Result<()> {
    fs::create_dir_all(output_dir)
        .with_context(|| format!("Failed to create output directory: {}", output_dir.display()))?;

    let output_file = output_dir.join("index.md");

    fs::write(&output_file, content)
        .with_context(|| format!("Failed to write file: {}", output_file.display()))?;

    Ok(())
}

/// Write multi-file markdown output to the specified directory.
pub fn write_markdown_multifile(output_dir: &Path, output: &MarkdownOutput) -> Result<()> {
    fs::create_dir_all(output_dir)
        .with_context(|| format!("Failed to create output directory: {}", output_dir.display()))?;

    for (file_path, content) in &output.files {
        let full_path = output_dir.join(file_path);

        // Create parent directories if needed
        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create directory: {}", parent.display()))?;
        }

        fs::write(&full_path, content)
            .with_context(|| format!("Failed to write file: {}", full_path.display()))?;
    }

    Ok(())
}
