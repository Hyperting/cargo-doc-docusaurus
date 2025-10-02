use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

mod parser;
mod converter;
mod writer;

#[derive(Parser)]
#[command(name = "rustdoc-to-markdown")]
#[command(about = "Convert rustdoc JSON output to markdown format", long_about = None)]
struct Cli {
    #[arg(help = "Path to rustdoc JSON file")]
    input: PathBuf,

    #[arg(short, long, default_value = "docs", help = "Output directory for markdown files")]
    output: PathBuf,

    #[arg(long, default_value = "false", help = "Include private items")]
    include_private: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let crate_data = parser::load_rustdoc_json(&cli.input)?;

    let markdown = converter::convert_to_markdown(&crate_data, cli.include_private)?;

    writer::write_markdown(&cli.output, &markdown)?;

    println!("✓ Conversion complete! Output written to: {}", cli.output.display());

    Ok(())
}
