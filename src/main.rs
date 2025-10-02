use anyhow::{Context, Result, bail};
use clap::Parser;
use rustdoc_json_to_markdown::ConversionOptions;
use std::path::PathBuf;
use std::process::Command;

#[derive(Parser)]
#[command(name = "rustdoc-json-to-markdown")]
#[command(about = "Convert rustdoc JSON output to markdown format", long_about = None)]
struct Cli {
    #[arg(help = "Path to rustdoc JSON file (optional if using --deps or --all-deps)")]
    input: Option<PathBuf>,

    #[arg(short, long, default_value = "docs", help = "Output directory for markdown files")]
    output: PathBuf,

    #[arg(long, help = "Include private items")]
    include_private: bool,

    #[arg(long, help = "Document specific dependencies (comma-separated list)", value_delimiter = ',')]
    deps: Vec<String>,

    #[arg(long, help = "Document all direct dependencies")]
    all_deps: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Dependency documentation mode
    if cli.all_deps || !cli.deps.is_empty() {
        document_dependencies(&cli)?;
        return Ok(());
    }

    // Single file conversion mode
    let input = cli.input.as_ref()
        .ok_or_else(|| anyhow::anyhow!("Input file required when not using --deps or --all-deps"))?;

    let options = ConversionOptions {
        input_path: input,
        output_dir: &cli.output,
        include_private: cli.include_private,
    };

    rustdoc_json_to_markdown::convert_json_file(&options)?;

    println!("✓ Conversion complete! Output written to: {}", cli.output.display());

    Ok(())
}

fn document_dependencies(cli: &Cli) -> Result<()> {
    // Get list of dependencies to document
    let deps_to_document = if cli.all_deps {
        get_all_dependencies()?
    } else {
        cli.deps.clone()
    };

    if deps_to_document.is_empty() {
        bail!("No dependencies found to document");
    }

    println!("📦 Documenting {} dependencies...", deps_to_document.len());

    let mut successful = 0;
    let mut failed = Vec::new();

    for dep in &deps_to_document {
        println!("\n🔨 Generating docs for '{}'...", dep);

        match document_single_dependency(dep, &cli.output, cli.include_private) {
            Ok(()) => {
                successful += 1;
                println!("  ✓ Successfully documented '{}'", dep);
            }
            Err(e) => {
                failed.push(dep.clone());
                println!("  ✗ Failed to document '{}': {}", dep, e);
            }
        }
    }

    println!("\n📊 Summary:");
    println!("  ✓ Successful: {}", successful);
    if !failed.is_empty() {
        println!("  ✗ Failed: {} ({})", failed.len(), failed.join(", "));
    }
    println!("\n✓ Documentation written to: {}", cli.output.display());

    Ok(())
}

fn get_all_dependencies() -> Result<Vec<String>> {
    // Use cargo metadata to get all direct dependencies
    let output = Command::new("cargo")
        .args(&["metadata", "--format-version=1"])
        .output()
        .context("Failed to run 'cargo metadata'")?;

    if !output.status.success() {
        bail!("cargo metadata failed: {}", String::from_utf8_lossy(&output.stderr));
    }

    let metadata: serde_json::Value = serde_json::from_slice(&output.stdout)
        .context("Failed to parse cargo metadata")?;

    let packages = metadata["packages"]
        .as_array()
        .context("Missing 'packages' in metadata")?;

    // Get the root package
    let resolve = &metadata["resolve"];
    let root = resolve["root"]
        .as_str()
        .context("Missing 'root' in metadata")?;

    // Find dependencies of the root package
    let nodes = resolve["nodes"]
        .as_array()
        .context("Missing 'nodes' in resolve")?;

    let root_node = nodes.iter()
        .find(|n| n["id"].as_str() == Some(root))
        .context("Root package not found in nodes")?;

    let dep_ids: Vec<String> = root_node["dependencies"]
        .as_array()
        .context("Missing 'dependencies' in root node")?
        .iter()
        .filter_map(|v| v.as_str().map(String::from))
        .collect();

    // Map package IDs to package names
    let mut dep_names = Vec::new();
    for dep_id in dep_ids {
        if let Some(pkg) = packages.iter().find(|p| p["id"].as_str() == Some(&dep_id)) {
            if let Some(name) = pkg["name"].as_str() {
                dep_names.push(name.to_string());
            }
        }
    }

    dep_names.sort();
    Ok(dep_names)
}

fn document_single_dependency(dep_name: &str, output_base: &PathBuf, include_private: bool) -> Result<()> {
    // Generate rustdoc JSON for the dependency
    let status = Command::new("cargo")
        .args(&[
            "+nightly",
            "rustdoc",
            "-p",
            dep_name,
            "--lib",
            "--",
            "--output-format=json",
            "-Z",
            "unstable-options",
        ])
        .status()
        .context("Failed to run cargo rustdoc")?;

    if !status.success() {
        bail!("cargo rustdoc failed for '{}'", dep_name);
    }

    // Find the generated JSON file
    let json_path = PathBuf::from("target/doc").join(format!("{}.json", dep_name.replace("-", "_")));

    if !json_path.exists() {
        bail!("Generated JSON file not found at {}", json_path.display());
    }

    // Convert to markdown in a subdirectory
    let output_dir = output_base.join("deps").join(dep_name);

    let options = ConversionOptions {
        input_path: &json_path,
        output_dir: &output_dir,
        include_private,
    };

    rustdoc_json_to_markdown::convert_json_file(&options)?;

    Ok(())
}
