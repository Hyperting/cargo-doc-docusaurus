use anyhow::{Context, Result, bail};
use cargo_doc_md::ConversionOptions;
use clap::Parser;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Parser)]
#[command(name = "cargo-doc-md")]
#[command(
    about = "Generate markdown documentation for Rust crates and dependencies",
    long_about = "Cargo subcommand to generate markdown documentation for Rust crates and their dependencies.\n\n\
                  Default behavior: Documents current crate + all dependencies with multi-file output.\n\
                  Creates a master index and organizes modules into separate files for easy navigation.\n\n\
                  Usage:\n  \
                  cargo doc-md              # Document current crate + all dependencies\n  \
                  cargo doc-md --all-deps   # Document only dependencies"
)]
struct Cli {
    #[arg(help = "Path to rustdoc JSON file (omit to auto-document current crate + all deps)")]
    input: Option<PathBuf>,

    #[arg(
        short,
        long,
        default_value = "target/doc-md",
        help = "Output directory [default: target/doc-md]\n\
                Creates: target/doc-md/index.md (master index), target/doc-md/crate_name/*.md (modules)"
    )]
    output: PathBuf,

    #[arg(long, help = "Include private items in documentation")]
    include_private: bool,

    #[arg(
        long,
        help = "Document only specific dependencies (comma-separated)",
        value_delimiter = ','
    )]
    deps: Vec<String>,

    #[arg(
        long,
        help = "Document only all direct dependencies (excludes current crate)"
    )]
    all_deps: bool,
}

fn main() -> Result<()> {
    // When invoked as `cargo doc-md`, cargo passes an extra "doc-md" argument
    // Skip it if present to support both `cargo doc-md` and `cargo-doc-md` invocations
    let args = std::env::args()
        .enumerate()
        .filter(|(i, arg)| !(*i == 1 && arg == "doc-md"))
        .map(|(_, arg)| arg);

    let cli = Cli::parse_from(args);

    // Explicit input file - just convert that file
    if let Some(input) = cli.input.as_ref() {
        let options = ConversionOptions {
            input_path: input,
            output_dir: &cli.output,
            include_private: cli.include_private,
        };

        cargo_doc_md::convert_json_file(&options)?;
        println!(
            "✓ Conversion complete! Output written to: {}",
            cli.output.display()
        );
        return Ok(());
    }

    // Dependency-only mode
    if cli.all_deps || !cli.deps.is_empty() {
        document_dependencies(&cli)?;
        return Ok(());
    }

    // Default: document current crate + all dependencies (like cargo doc)
    println!("📚 Documenting current crate and all dependencies...\n");

    let current_crate = document_current_crate(&cli)?;
    println!();
    let documented_deps = document_all_dependencies(&cli)?;

    // Generate master index
    generate_master_index(&cli.output, current_crate.as_deref(), &documented_deps)?;

    Ok(())
}

struct Dependency {
    name: String,
    version: String,
}

fn document_current_crate(cli: &Cli) -> Result<Option<String>> {
    println!("🔨 Generating rustdoc JSON for current crate...");

    // Run cargo rustdoc to generate JSON
    let output = Command::new("cargo")
        .args([
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
        bail!(
            "cargo rustdoc failed:\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    // Get the crate name from cargo metadata
    let metadata_output = Command::new("cargo")
        .args(["metadata", "--format-version=1", "--no-deps"])
        .output()
        .context("Failed to run cargo metadata")?;

    if !metadata_output.status.success() {
        bail!(
            "cargo metadata failed: {}",
            String::from_utf8_lossy(&metadata_output.stderr)
        );
    }

    let metadata: serde_json::Value = serde_json::from_slice(&metadata_output.stdout)
        .context("Failed to parse cargo metadata")?;

    let packages = metadata["packages"]
        .as_array()
        .context("Missing 'packages' in metadata")?;

    // Get the root package name
    let root_package = packages.first().context("No packages found in metadata")?;

    let crate_name = root_package["name"]
        .as_str()
        .context("Missing 'name' in package")?
        .to_string();

    // Find the generated JSON file
    let json_path =
        PathBuf::from("target/doc").join(format!("{}.json", crate_name.replace("-", "_")));

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

    cargo_doc_md::convert_json_file(&options)?;

    println!(
        "✓ Documentation complete! Output written to: {}/{}",
        cli.output.display(),
        crate_name
    );

    Ok(Some(crate_name))
}

fn document_all_dependencies(cli: &Cli) -> Result<Vec<String>> {
    let deps_to_document = get_all_dependencies()?;

    if deps_to_document.is_empty() {
        println!("No dependencies found");
        return Ok(Vec::new());
    }

    println!("📦 Documenting {} dependencies...", deps_to_document.len());

    let mut successful = Vec::new();
    let mut failed = Vec::new();

    for dep in &deps_to_document {
        println!("\n🔨 Generating docs for '{}'...", dep.name);

        match document_single_dependency(dep, &cli.output, cli.include_private) {
            Ok(()) => {
                successful.push(dep.name.clone());
                println!("  ✓ Successfully documented '{}'", dep.name);
            }
            Err(e) => {
                failed.push(dep.name.clone());
                println!("  ✗ Failed to document '{}': {}", dep.name, e);
            }
        }
    }

    println!("\n📊 Summary:");
    println!("  ✓ Successful: {}", successful.len());
    if !failed.is_empty() {
        println!("  ✗ Failed: {} ({})", failed.len(), failed.join(", "));
    }
    println!(
        "\n✓ Documentation written to: {}/deps",
        cli.output.display()
    );

    Ok(successful)
}

fn document_dependencies(cli: &Cli) -> Result<()> {
    // Get list of dependencies to document
    let deps_to_document = if cli.all_deps {
        get_all_dependencies()?
    } else {
        // For manually specified deps, we don't have version info
        // so we'll pass empty version (will attempt without version)
        cli.deps
            .iter()
            .map(|name| Dependency {
                name: name.clone(),
                version: String::new(),
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
        .args(["metadata", "--format-version=1"])
        .output()
        .context("Failed to run 'cargo metadata'")?;

    if !output.status.success() {
        bail!(
            "cargo metadata failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    let metadata: serde_json::Value =
        serde_json::from_slice(&output.stdout).context("Failed to parse cargo metadata")?;

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

    let root_node = nodes
        .iter()
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

fn document_single_dependency(
    dep: &Dependency,
    output_base: &Path,
    include_private: bool,
) -> Result<()> {
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
        .args([
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
        .stderr(std::process::Stdio::null()) // Suppress stderr (hides panics)
        .output()
        .context("Failed to run cargo rustdoc")?;

    if !output.status.success() {
        bail!("cargo rustdoc failed (exit code: {})", output.status);
    }

    // Find the generated JSON file
    let json_path =
        PathBuf::from("target/doc").join(format!("{}.json", dep.name.replace("-", "_")));

    if !json_path.exists() {
        bail!("Generated JSON file not found at {}", json_path.display());
    }

    // Convert to markdown in deps subdirectory
    // The converter will create a subdirectory with the crate name
    let output_dir = output_base.join("deps");

    let options = ConversionOptions {
        input_path: &json_path,
        output_dir: &output_dir,
        include_private,
    };

    cargo_doc_md::convert_json_file(&options)?;

    Ok(())
}

fn generate_master_index(
    output_dir: &Path,
    current_crate: Option<&str>,
    dependencies: &[String],
) -> Result<()> {
    use std::fs;

    let mut content = String::new();

    content.push_str("# Documentation Index\n\n");
    content.push_str("Generated markdown documentation for this project.\n\n");

    // Current crate section
    if let Some(crate_name) = current_crate {
        content.push_str("## Current Crate\n\n");
        content.push_str(&format!(
            "- [`{}`]({}index.md)\n\n",
            crate_name,
            crate_name.to_string() + "/"
        ));
    }

    // Dependencies section
    if !dependencies.is_empty() {
        content.push_str(&format!("## Dependencies ({})\n\n", dependencies.len()));

        for dep in dependencies {
            let dep_path = format!("deps/{}/index.md", dep);
            content.push_str(&format!("- [`{}`]({})\n", dep, dep_path));
        }
        content.push('\n');
    }

    content.push_str("---\n\n");
    content.push_str(
        "Generated with [cargo-doc-md](https://github.com/Crazytieguy/rustdoc-json-to-markdown)\n",
    );

    let index_path = output_dir.join("index.md");
    fs::write(&index_path, content)
        .with_context(|| format!("Failed to write master index: {}", index_path.display()))?;

    println!("\n✓ Master index created: {}", index_path.display());

    Ok(())
}
