//! Component management for Docusaurus integration
//!
//! This module handles installation and synchronization of React components
//! and CSS files needed for Rust documentation in Docusaurus.

use anyhow::{Context, Result, bail};
use std::fs;
use std::path::Path;

/// Template files embedded in the binary
struct Template {
  /// Relative path where the file should be installed (e.g., "src/components/RustCode/index.tsx")
  path: &'static str,
  /// File content
  content: &'static str,
  /// Description for user feedback
  description: &'static str,
}

/// All available templates embedded in the binary
const TEMPLATES: &[Template] = &[
  // RustCode component
  Template {
    path: "src/components/RustCode/index.tsx",
    content: include_str!("../templates/RustCode/index.tsx.txt"),
    description: "RustCode component (syntax highlighting with clickable links)",
  },
  Template {
    path: "src/components/RustCode/styles.module.css",
    content: include_str!("../templates/RustCode/styles.module.css.txt"),
    description: "RustCode styles",
  },
  // RustCrateLink component
  Template {
    path: "src/components/RustCrateLink/index.tsx",
    content: include_str!("../templates/RustCrateLink/index.tsx.txt"),
    description: "RustCrateLink component (crate titles with badges)",
  },
  Template {
    path: "src/components/RustCrateLink/styles.module.css",
    content: include_str!("../templates/RustCrateLink/styles.module.css.txt"),
    description: "RustCrateLink styles",
  },
  // RustModuleTitle component
  Template {
    path: "src/components/RustModuleTitle/index.tsx",
    content: include_str!("../templates/RustModuleTitle/index.tsx.txt"),
    description: "RustModuleTitle component (module titles in sidebar)",
  },
  Template {
    path: "src/components/RustModuleTitle/styles.module.css",
    content: include_str!("../templates/RustModuleTitle/styles.module.css.txt"),
    description: "RustModuleTitle styles",
  },
  // DocSidebarItem/Link swizzle
  Template {
    path: "src/theme/DocSidebarItem/Link/index.tsx",
    content: include_str!("../templates/DocSidebarItem/Link/index.tsx.txt"),
    description: "DocSidebarItem/Link (applies Rust item colors to sidebar)",
  },
  Template {
    path: "src/theme/DocSidebarItem/Link/styles.module.css",
    content: include_str!("../templates/DocSidebarItem/Link/styles.module.css.txt"),
    description: "DocSidebarItem/Link styles",
  },
  // CSS files
  Template {
    path: "src/css/rust-documentation.css",
    content: include_str!("../templates/rust-documentation.css.txt"),
    description: "Rust documentation CSS (colors, syntax highlighting, sidebar)",
  },
];

/// Initialize Docusaurus project with all Rust documentation components
pub fn init_components(docusaurus_path: &Path) -> Result<()> {
  println!("🚀 Installing Rust documentation components...\n");

  // Verify it's a Docusaurus project
  verify_docusaurus_project(docusaurus_path)?;

  let mut installed = 0;
  let mut skipped = 0;

  for template in TEMPLATES {
    let target_path = docusaurus_path.join(template.path);

    // Check if file already exists
    if target_path.exists() {
      println!("⚠️  Skipped (already exists): {}", template.description);
      skipped += 1;
      continue;
    }

    // Create parent directories
    if let Some(parent) = target_path.parent() {
      fs::create_dir_all(parent)
        .with_context(|| format!("Failed to create directory: {}", parent.display()))?;
    }

    // Write file
    fs::write(&target_path, template.content)
      .with_context(|| format!("Failed to write file: {}", target_path.display()))?;

    println!("✅ Installed: {}", template.description);
    installed += 1;
  }

  println!("\n📊 Summary:");
  println!("  ✅ Installed: {}", installed);
  if skipped > 0 {
    println!("  ⚠️  Skipped: {} (already exist)", skipped);
  }

  if installed > 0 {
    print_post_install_instructions();
  }

  Ok(())
}

/// Synchronize (update) existing components
pub fn sync_components(docusaurus_path: &Path) -> Result<()> {
  println!("🔄 Synchronizing Rust documentation components...\n");

  verify_docusaurus_project(docusaurus_path)?;

  let mut updated = 0;
  let mut skipped = 0;

  for template in TEMPLATES {
    let target_path = docusaurus_path.join(template.path);

    // Only update existing files
    if !target_path.exists() {
      println!("⚠️  Not installed (skipped): {}", template.description);
      skipped += 1;
      continue;
    }

    // Check if content is different
    let current_content = fs::read_to_string(&target_path)
      .with_context(|| format!("Failed to read file: {}", target_path.display()))?;

    if current_content == template.content {
      println!("✓ Up to date: {}", template.description);
      continue;
    }

    // Update file
    fs::write(&target_path, template.content)
      .with_context(|| format!("Failed to update file: {}", target_path.display()))?;

    println!("✅ Updated: {}", template.description);
    updated += 1;
  }

  println!("\n📊 Summary:");
  println!("  ✅ Updated: {}", updated);
  if skipped > 0 {
    println!("  ⚠️  Not installed: {}", skipped);
    println!("\nTo install missing components, run:");
    println!("  cargo doc-docusaurus components init <docusaurus-path>");
  }

  Ok(())
}

/// List all available components
pub fn list_components(docusaurus_path: Option<&Path>) -> Result<()> {
  println!("📦 Rust Documentation Components\n");

  if let Some(base_path) = docusaurus_path {
    // Group templates by component
    let groups = group_components();
    let mut installed_count = 0;
    let mut total_files = 0;

    for (component_name, files) in groups.iter() {
      println!("┌──────────────────────────────────────────────────┬─────────────────────┐");
      println!("│ 📄 {:<46}│                     │", component_name);
      println!("├──────────────────────────────────────────────────┼─────────────────────┤");
      println!("│ File                                             │ Status              │");
      println!("├──────────────────────────────────────────────────┼─────────────────────┤");

      for template in files {
        let target_path = base_path.join(template.path);
        let is_installed = target_path.exists();

        if is_installed {
          installed_count += 1;
        }
        total_files += 1;

        let status = if is_installed {
          "✅ Installed       "
        } else {
          "⚠️  Missing         "
        };

        let filename = template.path.split('/').last().unwrap_or(template.path);
        let filename_padded = format!("{:<48}", filename);

        println!("│ {} │ {} │", filename_padded, status);
      }

      println!("└──────────────────────────────────────────────────┴─────────────────────┘");

      // Show installation path for the component
      if let Some(first_file) = files.first() {
        let path_parts: Vec<&str> = first_file.path.split('/').collect();
        let component_path = if path_parts.len() > 1 {
          path_parts[..path_parts.len() - 1].join("/")
        } else {
          first_file.path.to_string()
        };
        println!("   📁 Path: {}", component_path);
      }
      println!();
    }

    println!(
      "📊 Summary: {}/{} files installed\n",
      installed_count, total_files
    );

    if installed_count < total_files {
      println!("💡 To install missing components:");
      println!(
        "   cargo doc-docusaurus components init {}\n",
        base_path.display()
      );
    }
  } else {
    // Show list without status
    let groups = group_components();
    let mut total_files = 0;

    for (component_name, files) in groups.iter() {
      println!("┌──────────────────────────────────────────────────┬─────────────────────┐");
      println!("│ 📄 {:<46}│                     │", component_name);
      println!("├──────────────────────────────────────────────────┼─────────────────────┤");
      println!("│ File                                             │ Type                │");
      println!("├──────────────────────────────────────────────────┼─────────────────────┤");

      for template in files {
        total_files += 1;

        let file_type = if template.path.ends_with(".tsx") {
          "Component          "
        } else if template.path.ends_with(".css") {
          "Stylesheet         "
        } else {
          "Other              "
        };

        let filename = template.path.split('/').last().unwrap_or(template.path);
        let filename_padded = format!("{:<48}", filename);

        println!("│ {} │ {} │", filename_padded, file_type);
      }

      println!("└──────────────────────────────────────────────────┴─────────────────────┘");

      // Show installation path
      if let Some(first_file) = files.first() {
        let path_parts: Vec<&str> = first_file.path.split('/').collect();
        let component_path = if path_parts.len() > 1 {
          path_parts[..path_parts.len() - 1].join("/")
        } else {
          first_file.path.to_string()
        };
        println!("   📁 Path: {}", component_path);
      }
      println!();
    }

    println!(
      "📊 Total: {} files in {} components\n",
      total_files,
      groups.len()
    );
    println!("💡 To check installation status:");
    println!("   cargo doc-docusaurus components list <docusaurus-path>\n");
  }

  Ok(())
}

/// Group templates by component name
fn group_components() -> Vec<(String, Vec<&'static Template>)> {
  let mut groups: Vec<(String, Vec<&'static Template>)> = Vec::new();

  // Define component groups
  let component_names = [
    ("RustCode", "RustCode"),
    ("RustCrateLink", "RustCrateLink"),
    ("RustModuleTitle", "RustModuleTitle"),
    ("DocSidebarItem/Link", "DocSidebarItem/Link"),
    ("rust-documentation.css", "rust-documentation.css"),
  ];

  for (display_name, path_component) in component_names {
    let mut files: Vec<&'static Template> = TEMPLATES
      .iter()
      .filter(|t| t.path.contains(path_component))
      .collect();

    if !files.is_empty() {
      // Sort by extension (tsx before css)
      files.sort_by(|a, b| {
        let a_ext = a.path.ends_with(".tsx");
        let b_ext = b.path.ends_with(".tsx");
        b_ext.cmp(&a_ext)
      });
      groups.push((display_name.to_string(), files));
    }
  }

  groups
}

/// Verify that the given path is a Docusaurus project
fn verify_docusaurus_project(path: &Path) -> Result<()> {
  if !path.exists() {
    bail!("Directory does not exist: {}", path.display());
  }

  if !path.is_dir() {
    bail!("Not a directory: {}", path.display());
  }

  // Check for common Docusaurus files
  let package_json = path.join("package.json");
  let docusaurus_config = path
    .join("docusaurus.config.ts")
    .exists()
    .then_some(())
    .or_else(|| path.join("docusaurus.config.js").exists().then_some(()));

  if !package_json.exists() {
    bail!(
      "Not a Docusaurus project: package.json not found in {}",
      path.display()
    );
  }

  if docusaurus_config.is_none() {
    bail!(
      "Not a Docusaurus project: docusaurus.config.ts/js not found in {}",
      path.display()
    );
  }

  Ok(())
}

/// Print post-installation instructions
fn print_post_install_instructions() {
  println!("\n📝 Next Steps:\n");
  println!("1. Add rust-documentation.css to your docusaurus.config.ts:");
  println!("   stylesheets: [");
  println!("     {{ href: '/css/rust-documentation.css', type: 'text/css' }}");
  println!("   ]\n");
  println!("2. (Optional) Customize colors in your src/css/custom.css:");
  println!("   cargo doc-docusaurus components css-vars\n");
  println!("3. Generate Rust documentation:");
  println!("   RUSTDOCFLAGS=\"-Z unstable-options --output-format json\" \\");
  println!("     cargo +nightly doc --no-deps\n");
  println!("4. Convert to Docusaurus markdown:");
  println!("   cargo doc-docusaurus target/doc/my_crate.json -o docs/api\n");
  println!("For detailed setup guide, see:");
  println!("  https://github.com/Hyperting/cargo-doc-docusaurus/blob/main/SETUP.md");
}

/// Print CSS variables for customization
pub fn print_css_variables(css_only: bool) {
  if !css_only {
    println!("🎨 Rust Documentation CSS Variables\n");
    println!("Add these to your src/css/custom.css to customize colors:\n");
  }

  println!(":root {{");
  println!("  /* Item type colors (light mode) */");
  println!("  --rust-color-module: rgb(173, 124, 0);           /* Modules (gold) */");
  println!("  --rust-color-module-hover: rgb(200, 140, 0);");
  println!("  --rust-color-struct: rgb(0, 125, 119);           /* Structs/Enums (cyan) */");
  println!("  --rust-color-struct-hover: rgb(0, 150, 140);");
  println!("  --rust-color-trait: rgb(110, 79, 201);           /* Traits (purple) */");
  println!("  --rust-color-trait-hover: rgb(130, 100, 220);");
  println!("  --rust-color-function: rgb(0, 92, 197);          /* Functions (blue) */");
  println!("  --rust-color-function-hover: rgb(0, 110, 220);");
  println!("  --rust-color-constant: rgb(150, 80, 40);         /* Constants (brown) */");
  println!("  --rust-color-constant-hover: rgb(180, 100, 50);");
  println!("  --rust-color-macro: rgb(196, 75, 0);             /* Macros (orange-red) */");
  println!("  --rust-color-macro-hover: rgb(220, 95, 20);");
  println!("}}");
  println!();
  println!("[data-theme='dark'] {{");
  println!("  /* Item type colors (dark mode) */");
  println!("  --rust-color-module: rgb(255, 180, 100);         /* Lighter gold */");
  println!("  --rust-color-module-hover: rgb(255, 200, 130);");
  println!("  --rust-color-struct: rgb(100, 200, 250);         /* Lighter cyan */");
  println!("  --rust-color-struct-hover: rgb(130, 220, 255);");
  println!("  --rust-color-trait: rgb(200, 150, 255);          /* Lighter purple */");
  println!("  --rust-color-trait-hover: rgb(220, 180, 255);");
  println!("  --rust-color-function: rgb(150, 220, 255);       /* Lighter blue */");
  println!("  --rust-color-function-hover: rgb(180, 235, 255);");
  println!("  --rust-color-constant: rgb(255, 180, 140);       /* Lighter brown */");
  println!("  --rust-color-constant-hover: rgb(255, 200, 160);");
  println!("  --rust-color-macro: rgb(255, 140, 100);          /* Lighter orange-red */");
  println!("  --rust-color-macro-hover: rgb(255, 160, 120);");
  println!("}}");

  if !css_only {
    println!();
    println!("💡 Usage:");
    println!("   cargo doc-docusaurus components css-vars --css-only >> src/css/custom.css");
    println!();
    println!("📖 More customization options:");
    println!("   See templates/CSS_GUIDE.md for complete variable reference");
  }
}
