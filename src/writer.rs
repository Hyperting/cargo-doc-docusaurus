//! Markdown file writer.

use crate::converter::MarkdownOutput;
use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

/// Write markdown content to a file in the specified directory.
pub fn write_markdown(output_dir: &Path, content: &str) -> Result<()> {
    fs::create_dir_all(output_dir).with_context(|| {
        format!(
            "Failed to create output directory: {}",
            output_dir.display()
        )
    })?;

    let output_file = output_dir.join("index.md");

    fs::write(&output_file, content)
        .with_context(|| format!("Failed to write file: {}", output_file.display()))?;

    Ok(())
}

/// Write multi-file markdown output to the specified directory.
pub fn write_markdown_multifile(output_dir: &Path, output: &MarkdownOutput) -> Result<()> {
    write_markdown_multifile_with_sidebar_path(output_dir, output, None)
}

/// Write multi-file markdown output with custom sidebar path.
pub fn write_markdown_multifile_with_sidebar_path(
    output_dir: &Path, 
    output: &MarkdownOutput,
    custom_sidebar_path: Option<&Path>
) -> Result<()> {
    fs::create_dir_all(output_dir).with_context(|| {
        format!(
            "Failed to create output directory: {}",
            output_dir.display()
        )
    })?;

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

    // Write sidebar configuration if present
    if let Some(sidebar_content) = &output.sidebar {
        let sidebar_path = if let Some(custom_path) = custom_sidebar_path {
            custom_path.to_path_buf()
        } else {
            // Default behavior: Write to docs parent directory (project root for Docusaurus)
            // If output_dir is "example-docs/docs/test_crate", parent.parent gives us "example-docs"
            if let Some(parent) = output_dir.parent() {
                if let Some(grandparent) = parent.parent() {
                    grandparent.join("sidebars-rust.ts")
                } else {
                    parent.join("sidebars-rust.ts")
                }
            } else {
                output_dir.join("sidebars-rust.ts")
            }
        };
        
        // Check if sidebar already exists for append mode
        let final_content = if sidebar_path.exists() {
            merge_sidebar_content(&sidebar_path, sidebar_content, &output.crate_name)?
        } else {
            sidebar_content.clone()
        };
        
        // Create parent directories if needed
        if let Some(parent) = sidebar_path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create directory: {}", parent.display()))?;
        }
        
        fs::write(&sidebar_path, final_content)
            .with_context(|| format!("Failed to write sidebar file: {}", sidebar_path.display()))?;
        
        println!("✓ Generated sidebar configuration: {}", sidebar_path.display());
        println!("  Import it in your sidebars.ts file:");
        println!("  import {{rustApiCategory}} from './sidebars-rust';");
    }

    Ok(())
}

/// Merge sidebar content when appending to existing sidebar
fn merge_sidebar_content(existing_path: &Path, new_content: &str, _crate_name: &str) -> Result<String> {
    // With the new multiple-sidebar format (rustSidebars object), we need to merge
    // the new sidebars into the existing object
    let existing_content = fs::read_to_string(existing_path)
        .with_context(|| format!("Failed to read existing sidebar: {}", existing_path.display()))?;
    
    // Extract rustSidebars from both files and merge them
    merge_rust_sidebars(&existing_content, new_content)
}

/// Merge rustSidebars objects from existing and new content
fn merge_rust_sidebars(existing: &str, new_content: &str) -> Result<String> {
    // Find the rustSidebars object in existing content
    let existing_start = "export const rustSidebars: Record<string, any[]> = {";
    let existing_end = "};";
    
    // Extract existing entries
    let existing_entries = if let Some(start_pos) = existing.find(existing_start) {
        let start = start_pos + existing_start.len();
        if let Some(end_pos) = existing[start..].find(existing_end) {
            &existing[start..start + end_pos]
        } else {
            ""
        }
    } else {
        ""
    };
    
    // Extract new entries
    let new_start = "export const rustSidebars: Record<string, any[]> = {";
    let new_end = "};";
    
    let new_entries = if let Some(start_pos) = new_content.find(new_start) {
        let start = start_pos + new_start.len();
        if let Some(end_pos) = new_content[start..].find(new_end) {
            &new_content[start..start + end_pos]
        } else {
            anyhow::bail!("Could not find rustSidebars object end in new content");
        }
    } else {
        anyhow::bail!("Could not find rustSidebars object in new content");
    };
    
    // Extract the header and footer from new_content (everything before and after rustSidebars)
    let header_end = new_content.find(new_start).unwrap() + new_start.len();
    let header = &new_content[..header_end];
    
    // Find where the footer starts (after the rustSidebars closing brace)
    let footer_start = header_end + new_entries.len() + new_end.len();
    let footer = &new_content[footer_start..];
    
    // Merge: combine existing entries with new entries, avoiding duplicates
    // Parse entries into a map to avoid duplicates
    let mut entries_map: std::collections::HashMap<String, String> = std::collections::HashMap::new();
    
    // Helper function to parse sidebar entries
    let parse_entries = |content: &str| -> Vec<(String, String)> {
        let mut results = Vec::new();
        let mut current_key = String::new();
        let mut current_value = String::new();
        let mut brace_depth = 0;
        let mut in_key = false;
        let mut in_value = false;
        
        for line in content.lines() {
            let trimmed = line.trim();
            
            // Start of a new entry
            if trimmed.starts_with('\'') && trimmed.contains("': [") {
                // Save previous entry if exists
                if !current_key.is_empty() {
                    results.push((current_key.clone(), current_value.clone()));
                }
                
                // Extract key
                if let Some(end_quote) = trimmed[1..].find('\'') {
                    current_key = trimmed[1..=end_quote].to_string();
                    current_value = line.to_string() + "\n";
                    in_value = true;
                    brace_depth = 1; // Count the opening [
                }
            } else if in_value {
                current_value.push_str(line);
                current_value.push('\n');
                
                // Count braces to detect end of entry
                for ch in line.chars() {
                    match ch {
                        '[' | '{' => brace_depth += 1,
                        ']' | '}' => {
                            brace_depth -= 1;
                            if brace_depth == 0 {
                                // Entry complete
                                results.push((current_key.clone(), current_value.clone()));
                                current_key.clear();
                                current_value.clear();
                                in_value = false;
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        
        // Save last entry if exists
        if !current_key.is_empty() {
            results.push((current_key, current_value));
        }
        
        results
    };
    
    // Parse existing entries
    for (key, value) in parse_entries(existing_entries) {
        entries_map.insert(key, value);
    }
    
    // Parse and add/overwrite with new entries
    for (key, value) in parse_entries(new_entries) {
        entries_map.insert(key, value); // This will overwrite duplicates
    }
    
    // Sort keys and build merged content
    let mut sorted_keys: Vec<_> = entries_map.keys().collect();
    sorted_keys.sort();
    
    let mut merged_entries = String::new();
    for (i, key) in sorted_keys.iter().enumerate() {
        if let Some(value) = entries_map.get(*key) {
            merged_entries.push_str(value);
            // Add comma between entries (but not after the last one)
            if i < sorted_keys.len() - 1 && !value.trim_end().ends_with(',') {
                merged_entries.push(',');
            }
        }
    }
    
    // Generate rootRustSidebar with all workspace crates
    let root_sidebar = generate_root_sidebar(&merged_entries);
    
    // Construct the final output
    let mut result = String::new();
    result.push_str(header);
    result.push_str(&merged_entries);
    result.push_str(new_end);
    result.push_str(footer);
    
    // Append the root sidebar export
    result.push_str("\n\n// Root sidebar with links to all crates (for main navigation)\n");
    result.push_str("export const rootRustSidebar = [\n");
    result.push_str(&root_sidebar);
    result.push_str("];\n");
    
    Ok(result)
}

/// Generate a root sidebar that includes all crates with their content
/// This function parses the merged sidebar content to extract root crate information
fn generate_root_sidebar(merged_entries: &str) -> String {
    let mut output = String::new();
    
    // Find all lines with rustCrateTitle: true to identify root crates
    // These lines have the format:
    // { type: 'doc', id: 'runtime/rust/crate_name/index', label: 'crate_name', customProps: { rustCrateTitle: true, ... } }
    
    use std::collections::HashSet;
    let mut seen_crates: HashSet<String> = HashSet::new();
    let mut crate_entries: Vec<(String, String)> = Vec::new(); // (doc_id, label)
    
    for line in merged_entries.lines() {
        let trimmed = line.trim();
        
        // Look for lines containing rustCrateTitle: true
        if trimmed.contains("rustCrateTitle: true") && trimmed.contains("type: 'doc'") {
            // Extract the doc id and label from the line
            // Format: { type: 'doc', id: 'path/to/crate/index', label: 'crate_name', customProps: ...
            
            if let Some(id_start) = trimmed.find("id: '") {
                if let Some(id_end) = trimmed[id_start + 5..].find('\'') {
                    let doc_id = &trimmed[id_start + 5..id_start + 5 + id_end];
                    
                    // Extract label
                    if let Some(label_start) = trimmed.find("label: '") {
                        if let Some(label_end) = trimmed[label_start + 8..].find('\'') {
                            let label = &trimmed[label_start + 8..label_start + 8 + label_end];
                            
                            // Only add each crate once (in case it appears in multiple sidebars)
                            if seen_crates.insert(doc_id.to_string()) {
                                crate_entries.push((doc_id.to_string(), label.to_string()));
                            }
                        }
                    }
                }
            }
        }
    }
    
    // Sort by doc_id for consistent output
    crate_entries.sort_by(|a, b| a.0.cmp(&b.0));
    
    for (doc_id, label) in crate_entries {
        output.push_str(&format!(
            "  {{ type: 'doc', id: '{}', label: '{}', className: 'rust-mod' }},\n",
            doc_id, label
        ));
    }
    
    output
}
