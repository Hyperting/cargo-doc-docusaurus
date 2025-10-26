//! Convert rustdoc JSON output to Docusaurus-compatible markdown documentation.
//!
//! This library provides functionality to parse rustdoc's JSON output and convert it
//! to well-formatted markdown files with React component integration for Docusaurus sites.
//!
//! # Example
//!
//! ```no_run
//! use cargo_doc_docusaurus::{convert_json_file, ConversionOptions};
//! use std::path::Path;
//!
//! let options = ConversionOptions {
//!     input_path: Path::new("target/doc/my_crate.json"),
//!     output_dir: Path::new("docs"),
//!     include_private: false,
//!     base_path: "",
//!     workspace_crates: &[],
//!     sidebarconfig_collapsed: false,
//!     sidebar_output: None,
//!     sidebar_root_link: None,
//! };
//!
//! convert_json_file(&options).expect("Conversion failed");
//! ```

pub mod converter;
pub mod parser;
pub mod writer;

pub use rustdoc_types;

use anyhow::Result;
use std::path::Path;

/// Options for converting rustdoc JSON to markdown.
pub struct ConversionOptions<'a> {
    /// Path to the input rustdoc JSON file
    pub input_path: &'a Path,
    /// Directory where markdown files will be written
    pub output_dir: &'a Path,
    /// Whether to include private items in the output
    pub include_private: bool,
    /// Base path for links (e.g., "/docs/runtime/rust" for Docusaurus routing)
    pub base_path: &'a str,
    /// List of workspace crate names - external crates in this list will use internal links
    pub workspace_crates: &'a [String],
    /// Whether to generate sidebar categories as collapsed
    pub sidebarconfig_collapsed: bool,
    /// Custom path for the sidebar configuration file
    pub sidebar_output: Option<&'a Path>,
    /// URL for the 'Go back' link in root crate sidebars
    pub sidebar_root_link: Option<&'a str>,
}

/// Convert a rustdoc JSON file to markdown (multi-file output).
///
/// This is the main entry point for library usage. Generates one file per module.
///
/// # Arguments
///
/// * `options` - Configuration for the conversion
///
/// # Returns
///
/// Returns `Ok(())` on success, or an error if the conversion fails.
///
/// # Example
///
/// ```no_run
/// use cargo_doc_docusaurus::{convert_json_file, ConversionOptions};
/// use std::path::Path;
///
/// let options = ConversionOptions {
///     input_path: Path::new("target/doc/my_crate.json"),
///     output_dir: Path::new("docs"),
///     include_private: false,
///     base_path: "",  // Optional: use "/docs/runtime/rust" for Docusaurus routing
///     workspace_crates: &[],
///     sidebarconfig_collapsed: false,
///     sidebar_output: None,
///     sidebar_root_link: None,
/// };
///
/// convert_json_file(&options).expect("Conversion failed");
/// ```
pub fn convert_json_file(options: &ConversionOptions) -> Result<()> {
    let crate_data = parser::load_rustdoc_json(options.input_path)?;
    let output = converter::convert_to_markdown_multifile(
        &crate_data, 
        options.include_private, 
        options.base_path,
        options.workspace_crates,
        options.sidebarconfig_collapsed,
        options.sidebar_root_link,
    )?;

    // Write to crate-specific subdirectory
    let crate_output_dir = options.output_dir.join(&output.crate_name);
    writer::write_markdown_multifile_with_sidebar_path(&crate_output_dir, &output, options.sidebar_output)?;
    Ok(())
}

/// Convert rustdoc JSON data (already loaded) to markdown.
///
/// Use this if you want more control over the loading and writing process.
///
/// # Arguments
///
/// * `json_data` - The rustdoc JSON as a string
/// * `include_private` - Whether to include private items
///
/// # Returns
///
/// Returns the markdown as a String, or an error.
pub fn convert_json_string(json_data: &str, include_private: bool) -> Result<String> {
    let crate_data: rustdoc_types::Crate = serde_json::from_str(json_data)?;
    converter::convert_to_markdown(&crate_data, include_private)
}
