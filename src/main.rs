use anyhow::Result;
use clap::Parser;
use rustdoc_to_markdown::ConversionOptions;
use std::path::PathBuf;

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

    let options = ConversionOptions {
        input_path: &cli.input,
        output_dir: &cli.output,
        include_private: cli.include_private,
    };

    rustdoc_to_markdown::convert_json_file(&options)?;

    println!("✓ Conversion complete! Output written to: {}", cli.output.display());

    Ok(())
}
