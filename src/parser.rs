use anyhow::{Context, Result};
use rustdoc_types::Crate;
use std::path::Path;

pub fn load_rustdoc_json(path: &Path) -> Result<Crate> {
    let contents = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read file: {}", path.display()))?;

    let crate_data: Crate = serde_json::from_str(&contents)
        .with_context(|| format!("Failed to parse JSON from: {}", path.display()))?;

    println!("Loaded crate: {} (format version: {})",
             crate_data.index.get(&crate_data.root)
                 .and_then(|item| item.name.as_deref())
                 .unwrap_or("unknown"),
             crate_data.format_version);

    Ok(crate_data)
}
