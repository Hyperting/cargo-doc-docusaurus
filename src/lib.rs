//! Convert rustdoc JSON output to clean, LLM-friendly markdown documentation.
//!
//! This library provides functionality to parse rustdoc's JSON output and convert it
//! to well-formatted markdown files suitable for LLM consumption and text-based viewing.
//!
//! # Example
//!
//! ```no_run
//! use cargo_doc_md::{convert_json_file, ConversionOptions};
//! use std::path::Path;
//!
//! let options = ConversionOptions {
//!     input_path: Path::new("target/doc/my_crate.json"),
//!     output_dir: Path::new("docs"),
//!     include_private: false,
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
/// use cargo_doc_md::{convert_json_file, ConversionOptions};
/// use std::path::Path;
///
/// let options = ConversionOptions {
///     input_path: Path::new("target/doc/my_crate.json"),
///     output_dir: Path::new("docs"),
///     include_private: false,
/// };
///
/// convert_json_file(&options).expect("Conversion failed");
/// ```
pub fn convert_json_file(options: &ConversionOptions) -> Result<()> {
    let crate_data = parser::load_rustdoc_json(options.input_path)?;
    let output = converter::convert_to_markdown_multifile(&crate_data, options.include_private)?;

    // Write to crate-specific subdirectory
    let crate_output_dir = options.output_dir.join(&output.crate_name);
    writer::write_markdown_multifile(&crate_output_dir, &output)?;
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
