use anyhow::Result;
use cargo_doc_docusaurus::ConversionOptions;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod components;

#[derive(Parser)]
#[command(name = "cargo-doc-docusaurus")]
#[command(about = "Generate Docusaurus-compatible documentation for Rust crates")]
struct Cli {
  #[command(subcommand)]
  command: Option<Commands>,

  #[arg(help = "Path to rustdoc JSON file")]
  input: Option<PathBuf>,

  #[arg(short, long, default_value = "target/doc-md")]
  output: PathBuf,

  #[arg(long)]
  include_private: bool,

  #[arg(long, default_value = "")]
  base_path: String,

  #[arg(long, value_delimiter = ',')]
  workspace_crates: Vec<String>,

  #[arg(long = "sidebarconfig-collapsed", num_args = 0..=1, default_missing_value = "true", default_value = "true")]
  sidebarconfig_collapsed: bool,

  #[arg(long)]
  sidebar_output: Option<PathBuf>,

  #[arg(long)]
  sidebar_root_link: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
  #[command(about = "Manage Docusaurus components (React, CSS, theme overrides)")]
  Components {
    #[command(subcommand)]
    command: ComponentsCommand,
  },
}

#[derive(Subcommand)]
enum ComponentsCommand {
  #[command(about = "Install all components in a Docusaurus project")]
  #[command(
    long_about = "Install all required React components, CSS files, and theme overrides.\n\
                            Creates necessary directories and copies all files.\n\
                            Skips files that already exist (no overwrites).\n\n\
                            Example:\n  \
                            cargo doc-docusaurus components init /path/to/docusaurus"
  )]
  Init {
    #[arg(help = "Path to Docusaurus project root directory")]
    docusaurus_path: PathBuf,
  },

  #[command(about = "Update existing components to latest version")]
  #[command(
    long_about = "Sync (update) only the components that are already installed.\n\
                            Does not create new files, only updates existing ones.\n\
                            Useful for upgrading to latest component versions.\n\n\
                            Example:\n  \
                            cargo doc-docusaurus components sync /path/to/docusaurus"
  )]
  Sync {
    #[arg(help = "Path to Docusaurus project root directory")]
    docusaurus_path: PathBuf,
  },

  #[command(about = "List all available components and their installation status")]
  #[command(
    long_about = "Show all available components with their files and types.\n\
                            If a path is provided, shows installation status (✅/⚠️).\n\
                            Components include: RustCode, RustCrateLink, RustModuleTitle,\n\
                            DocSidebarItem/Link, and rust-documentation.css.\n\n\
                            Examples:\n  \
                            cargo doc-docusaurus components list\n  \
                            cargo doc-docusaurus components list /path/to/docusaurus"
  )]
  List {
    #[arg(help = "Optional path to check installation status")]
    docusaurus_path: Option<PathBuf>,
  },

  #[command(name = "css-vars")]
  #[command(about = "Show customizable CSS variables for theming")]
  #[command(
    long_about = "Display all CSS custom properties (variables) that can be overridden\n\
                            to customize colors for modules, structs, traits, functions, etc.\n\
                            Includes both light and dark mode variables.\n\n\
                            Examples:\n  \
                            cargo doc-docusaurus components css-vars\n  \
                            cargo doc-docusaurus components css-vars --css-only >> src/css/custom.css"
  )]
  CssVars {
    #[arg(
      long = "css-only",
      help = "Output only CSS without explanatory text (for piping to file)"
    )]
    css_only: bool,
  },
}

fn main() -> Result<()> {
  let args = std::env::args()
    .enumerate()
    .filter(|(i, arg)| !(*i == 1 && arg == "doc-docusaurus"))
    .map(|(_, arg)| arg);

  let cli = Cli::parse_from(args);

  if let Some(command) = cli.command {
    match command {
      Commands::Components { command } => match command {
        ComponentsCommand::Init { docusaurus_path } => {
          components::init_components(&docusaurus_path)?;
        }
        ComponentsCommand::Sync { docusaurus_path } => {
          components::sync_components(&docusaurus_path)?;
        }
        ComponentsCommand::List { docusaurus_path } => {
          components::list_components(docusaurus_path.as_deref())?;
        }
        ComponentsCommand::CssVars { css_only } => {
          components::print_css_variables(css_only);
        }
      },
    }
    return Ok(());
  }

  if let Some(input) = cli.input.as_ref() {
    let options = ConversionOptions {
      input_path: input,
      output_dir: &cli.output,
      include_private: cli.include_private,
      base_path: &cli.base_path,
      workspace_crates: &cli.workspace_crates,
      sidebarconfig_collapsed: cli.sidebarconfig_collapsed,
      sidebar_output: cli.sidebar_output.as_deref(),
      sidebar_root_link: cli.sidebar_root_link.as_deref(),
    };

    cargo_doc_docusaurus::convert_json_file(&options)?;
    println!("✓ Conversion complete! Output: {}", cli.output.display());
    return Ok(());
  }

  eprintln!("Error: No input file or command specified");
  eprintln!("  cargo doc-docusaurus <INPUT.json> -o <OUTPUT>");
  eprintln!("  cargo doc-docusaurus components init <PATH>");
  std::process::exit(1);
}
