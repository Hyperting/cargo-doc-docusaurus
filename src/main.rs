use anyhow::{Context, Result, bail};
use clap::Parser;
use rustdoc_json_to_markdown::ConversionOptions;
use std::path::PathBuf;
use std::process::Command;

#[derive(Parser)]
#[command(name = "rustdoc-json-to-markdown")]
#[command(about = "Convert rustdoc JSON output to markdown format", long_about = None)]
struct Cli {
    #[arg(help = "Path to rustdoc JSON file (omit to auto-document current crate + deps)")]
    input: Option<PathBuf>,

    #[arg(short, long, default_value = "docs", help = "Output directory for markdown files")]
    output: PathBuf,

    #[arg(long, help = "Include private items")]
    include_private: bool,

    #[arg(long, help = "Document only specific dependencies (comma-separated list)", value_delimiter = ',')]
    deps: Vec<String>,

    #[arg(long, help = "Document only all direct dependencies")]
    all_deps: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Explicit input file - just convert that file
    if let Some(input) = cli.input.as_ref() {
        let options = ConversionOptions {
            input_path: input,
            output_dir: &cli.output,
            include_private: cli.include_private,
        };

        rustdoc_json_to_markdown::convert_json_file(&options)?;
        println!("✓ Conversion complete! Output written to: {}", cli.output.display());
        return Ok(());
    }

    // Dependency-only mode
    if cli.all_deps || !cli.deps.is_empty() {
        document_dependencies(&cli)?;
        return Ok(());
    }

    // Default: document current crate + all dependencies (like cargo doc)
    println!("📚 Documenting current crate and all dependencies...\n");

    document_current_crate(&cli)?;
    println!();
    document_all_dependencies(&cli)?;

    Ok(())
}

struct Dependency {
    name: String,
    version: String,
}

fn document_current_crate(cli: &Cli) -> Result<()> {
    println!("🔨 Generating rustdoc JSON for current crate...");

    // Run cargo rustdoc to generate JSON
    let output = Command::new("cargo")
        .args(&[
            "+nightly",
            "rustdoc",
            "--lib",
            "--",
            "--output-format=json",
            "-Z",
            "unstable-options",
        ])
        .output()
        .context("Failed to run cargo rustdoc")?;

    if !output.status.success() {
        bail!("cargo rustdoc failed:\n{}", String::from_utf8_lossy(&output.stderr));
    }

    // Get the crate name from cargo metadata
    let metadata_output = Command::new("cargo")
        .args(&["metadata", "--format-version=1", "--no-deps"])
        .output()
        .context("Failed to run cargo metadata")?;

    if !metadata_output.status.success() {
        bail!("cargo metadata failed: {}", String::from_utf8_lossy(&metadata_output.stderr));
    }

    let metadata: serde_json::Value = serde_json::from_slice(&metadata_output.stdout)
        .context("Failed to parse cargo metadata")?;

    let packages = metadata["packages"]
        .as_array()
        .context("Missing 'packages' in metadata")?;

    // Get the root package name
    let root_package = packages.first()
        .context("No packages found in metadata")?;

    let crate_name = root_package["name"]
        .as_str()
        .context("Missing 'name' in package")?;

    // Find the generated JSON file
    let json_path = PathBuf::from("target/doc").join(format!("{}.json", crate_name.replace("-", "_")));

    if !json_path.exists() {
        bail!("Generated JSON file not found at {}", json_path.display());
    }

    println!("✓ JSON generated successfully");
    println!("🔄 Converting to markdown...");

    // Convert to markdown
    let options = ConversionOptions {
        input_path: &json_path,
        output_dir: &cli.output,
        include_private: cli.include_private,
    };

    rustdoc_json_to_markdown::convert_json_file(&options)?;

    println!("✓ Documentation complete! Output written to: {}", cli.output.display());

    Ok(())
}

fn document_all_dependencies(cli: &Cli) -> Result<()> {
    let deps_to_document = get_all_dependencies()?;

    if deps_to_document.is_empty() {
        println!("No dependencies found");
        return Ok(());
    }

    println!("📦 Documenting {} dependencies...", deps_to_document.len());

    let mut successful = 0;
    let mut failed = Vec::new();

    for dep in &deps_to_document {
        println!("\n🔨 Generating docs for '{}'...", dep.name);

        match document_single_dependency(dep, &cli.output, cli.include_private) {
            Ok(()) => {
                successful += 1;
                println!("  ✓ Successfully documented '{}'", dep.name);
            }
            Err(e) => {
                failed.push(dep.name.clone());
                println!("  ✗ Failed to document '{}': {}", dep.name, e);
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

fn document_dependencies(cli: &Cli) -> Result<()> {
    // Get list of dependencies to document
    let deps_to_document = if cli.all_deps {
        get_all_dependencies()?
    } else {
        // For manually specified deps, we don't have version info
        // so we'll pass empty version (will attempt without version)
        cli.deps.iter()
            .map(|name| Dependency {
                name: name.clone(),
                version: String::new()
            })
            .collect()
    };

    if deps_to_document.is_empty() {
        bail!("No dependencies found to document");
    }

    println!("📦 Documenting {} dependencies...", deps_to_document.len());

    let mut successful = 0;
    let mut failed = Vec::new();

    for dep in &deps_to_document {
        println!("\n🔨 Generating docs for '{}'...", dep.name);

        match document_single_dependency(dep, &cli.output, cli.include_private) {
            Ok(()) => {
                successful += 1;
                println!("  ✓ Successfully documented '{}'", dep.name);
            }
            Err(e) => {
                failed.push(dep.name.clone());
                println!("  ✗ Failed to document '{}': {}", dep.name, e);
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

fn get_all_dependencies() -> Result<Vec<Dependency>> {
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

    // Map package IDs to package names and versions
    let mut deps = Vec::new();
    for dep_id in dep_ids {
        if let Some(pkg) = packages.iter().find(|p| p["id"].as_str() == Some(&dep_id)) {
            if let (Some(name), Some(version)) = (pkg["name"].as_str(), pkg["version"].as_str()) {
                deps.push(Dependency {
                    name: name.to_string(),
                    version: version.to_string(),
                });
            }
        }
    }

    deps.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(deps)
}

fn document_single_dependency(dep: &Dependency, output_base: &PathBuf, include_private: bool) -> Result<()> {
    // Build the package specification
    // If we have a version, use name@version to disambiguate multiple versions
    let package_spec = if dep.version.is_empty() {
        dep.name.clone()
    } else {
        format!("{}@{}", dep.name, dep.version)
    };

    // Generate rustdoc JSON for the dependency
    // Suppress stderr to hide panics and verbose cargo output
    let output = Command::new("cargo")
        .args(&[
            "+nightly",
            "rustdoc",
            "-p",
            &package_spec,
            "--lib",
            "--",
            "--output-format=json",
            "-Z",
            "unstable-options",
        ])
        .stderr(std::process::Stdio::null())  // Suppress stderr (hides panics)
        .output()
        .context("Failed to run cargo rustdoc")?;

    if !output.status.success() {
        bail!("cargo rustdoc failed (exit code: {})", output.status);
    }

    // Find the generated JSON file
    let json_path = PathBuf::from("target/doc").join(format!("{}.json", dep.name.replace("-", "_")));

    if !json_path.exists() {
        bail!("Generated JSON file not found at {}", json_path.display());
    }

    // Convert to markdown in a subdirectory
    let output_dir = output_base.join("deps").join(&dep.name);

    let options = ConversionOptions {
        input_path: &json_path,
        output_dir: &output_dir,
        include_private,
    };

    rustdoc_json_to_markdown::convert_json_file(&options)?;

    Ok(())
}
