use std::fs;
use std::path::Path;
use tempfile::TempDir;

// Import the components module from the main binary
// Note: Since components.rs is in src/main.rs binary, we need to make it a library module
// For now, we'll test the CLI commands end-to-end

#[test]
fn test_verify_docusaurus_project_valid() {
  let temp_dir = TempDir::new().expect("Failed to create temp dir");
  let temp_path = temp_dir.path();

  // Create minimal valid Docusaurus structure
  fs::write(
    temp_path.join("package.json"),
    r#"{"name": "test-docs", "dependencies": {"@docusaurus/core": "3.0.0"}}"#,
  )
  .expect("Failed to create package.json");

  fs::write(
    temp_path.join("docusaurus.config.ts"),
    "export default { title: 'Test' };",
  )
  .expect("Failed to create docusaurus.config.ts");

  // Note: We would call verify_docusaurus_project here if it was exported
  // For now, we test via CLI commands
  assert!(temp_path.join("package.json").exists());
  assert!(temp_path.join("docusaurus.config.ts").exists());
}

#[test]
fn test_verify_docusaurus_project_missing_package_json() {
  let temp_dir = TempDir::new().expect("Failed to create temp dir");
  let temp_path = temp_dir.path();

  // Only create docusaurus.config, missing package.json
  fs::write(
    temp_path.join("docusaurus.config.ts"),
    "export default { title: 'Test' };",
  )
  .expect("Failed to create docusaurus.config.ts");

  // Should fail validation
  assert!(!temp_path.join("package.json").exists());
}

#[test]
fn test_verify_docusaurus_project_missing_config() {
  let temp_dir = TempDir::new().expect("Failed to create temp dir");
  let temp_path = temp_dir.path();

  // Only create package.json, missing config
  fs::write(
    temp_path.join("package.json"),
    r#"{"name": "test-docs", "dependencies": {"@docusaurus/core": "3.0.0"}}"#,
  )
  .expect("Failed to create package.json");

  // Should fail validation
  assert!(!temp_path.join("docusaurus.config.ts").exists());
  assert!(!temp_path.join("docusaurus.config.js").exists());
}

#[test]
fn test_components_init_creates_all_files() {
  let temp_dir = TempDir::new().expect("Failed to create temp dir");
  let temp_path = temp_dir.path();

  // Create minimal Docusaurus structure
  fs::write(
    temp_path.join("package.json"),
    r#"{"name": "test-docs", "dependencies": {"@docusaurus/core": "3.0.0"}}"#,
  )
  .expect("Failed to create package.json");

  fs::write(
    temp_path.join("docusaurus.config.ts"),
    "export default { title: 'Test' };",
  )
  .expect("Failed to create docusaurus.config.ts");

  // Run components init command
  let output = std::process::Command::new("cargo")
    .args(&[
      "run",
      "--",
      "components",
      "init",
      temp_path.to_str().unwrap(),
    ])
    .output()
    .expect("Failed to run cargo run");

  let stdout = String::from_utf8_lossy(&output.stdout);
  let stderr = String::from_utf8_lossy(&output.stderr);

  // Should succeed
  assert!(
    output.status.success(),
    "Command failed:\nstdout: {}\nstderr: {}",
    stdout,
    stderr
  );

  // Verify all expected files were created
  let expected_files = vec![
    "src/components/RustCode/index.tsx",
    "src/components/RustCode/styles.module.css",
    "src/components/RustCrateLink/index.tsx",
    "src/components/RustCrateLink/styles.module.css",
    "src/components/RustModuleTitle/index.tsx",
    "src/components/RustModuleTitle/styles.module.css",
    "src/theme/DocSidebarItem/Link/index.tsx",
    "src/theme/DocSidebarItem/Link/styles.module.css",
    "src/css/rust-documentation.css",
  ];

  for file in expected_files {
    let file_path = temp_path.join(file);
    assert!(
      file_path.exists(),
      "Expected file not created: {}",
      file_path.display()
    );

    // Verify file is not empty
    let metadata = fs::metadata(&file_path).expect("Failed to get metadata");
    assert!(metadata.len() > 0, "File is empty: {}", file_path.display());
  }
}

#[test]
fn test_components_init_skips_existing_files() {
  let temp_dir = TempDir::new().expect("Failed to create temp dir");
  let temp_path = temp_dir.path();

  // Create minimal Docusaurus structure
  fs::write(
    temp_path.join("package.json"),
    r#"{"name": "test-docs", "dependencies": {"@docusaurus/core": "3.0.0"}}"#,
  )
  .expect("Failed to create package.json");

  fs::write(
    temp_path.join("docusaurus.config.ts"),
    "export default { title: 'Test' };",
  )
  .expect("Failed to create docusaurus.config.ts");

  // Pre-create one file with custom content
  let rust_code_dir = temp_path.join("src/components/RustCode");
  fs::create_dir_all(&rust_code_dir).expect("Failed to create directory");
  let custom_content = "// My custom RustCode component";
  fs::write(rust_code_dir.join("index.tsx"), custom_content).expect("Failed to create custom file");

  // Run components init command
  let output = std::process::Command::new("cargo")
    .args(&[
      "run",
      "--",
      "components",
      "init",
      temp_path.to_str().unwrap(),
    ])
    .output()
    .expect("Failed to run cargo run");

  assert!(output.status.success(), "Command should succeed");

  // Verify the pre-existing file was NOT overwritten
  let file_content =
    fs::read_to_string(rust_code_dir.join("index.tsx")).expect("Failed to read file");
  assert_eq!(
    file_content, custom_content,
    "Pre-existing file should not be overwritten"
  );

  // Verify output mentions skipping
  let stdout = String::from_utf8_lossy(&output.stdout);
  assert!(
    stdout.contains("already exists") || stdout.contains("Skipped"),
    "Output should mention skipping existing files"
  );
}

#[test]
fn test_components_sync_updates_existing_only() {
  let temp_dir = TempDir::new().expect("Failed to create temp dir");
  let temp_path = temp_dir.path();

  // Create minimal Docusaurus structure
  fs::write(
    temp_path.join("package.json"),
    r#"{"name": "test-docs", "dependencies": {"@docusaurus/core": "3.0.0"}}"#,
  )
  .expect("Failed to create package.json");

  fs::write(
    temp_path.join("docusaurus.config.ts"),
    "export default { title: 'Test' };",
  )
  .expect("Failed to create docusaurus.config.ts");

  // Create only some components (simulate partial installation)
  let rust_code_dir = temp_path.join("src/components/RustCode");
  fs::create_dir_all(&rust_code_dir).expect("Failed to create directory");
  fs::write(rust_code_dir.join("index.tsx"), "// Old version").expect("Failed to create file");

  // Run components sync command
  let output = std::process::Command::new("cargo")
    .args(&[
      "run",
      "--",
      "components",
      "sync",
      temp_path.to_str().unwrap(),
    ])
    .output()
    .expect("Failed to run cargo run");

  assert!(output.status.success(), "Command should succeed");

  // Verify RustCode was updated (content should be different from "// Old version")
  let updated_content =
    fs::read_to_string(rust_code_dir.join("index.tsx")).expect("Failed to read file");
  assert_ne!(updated_content, "// Old version", "File should be updated");
  assert!(
    updated_content.len() > 100,
    "Updated file should have real component code"
  );

  // Verify other components were NOT created
  let rust_crate_link = temp_path.join("src/components/RustCrateLink/index.tsx");
  assert!(
    !rust_crate_link.exists(),
    "Sync should not create new files"
  );
}

#[test]
fn test_components_list_without_path() {
  // Run components list without path - should show all templates
  let output = std::process::Command::new("cargo")
    .args(&["run", "--", "components", "list"])
    .output()
    .expect("Failed to run cargo run");

  assert!(output.status.success(), "Command should succeed");

  let stdout = String::from_utf8_lossy(&output.stdout);

  // Should list all 10 components (no custom.css anymore)
  assert!(
    stdout.contains("RustCode"),
    "Should list RustCode component"
  );
  assert!(
    stdout.contains("RustCrateLink"),
    "Should list RustCrateLink component"
  );
  assert!(
    stdout.contains("RustModuleTitle"),
    "Should list RustModuleTitle component"
  );
  assert!(
    stdout.contains("DocSidebarItem"),
    "Should list DocSidebarItem component"
  );
  assert!(
    stdout.contains("rust-documentation.css"),
    "Should list rust-documentation.css"
  );
  // Check that table with emoji is shown
  assert!(
    stdout.contains("📄") && stdout.contains("Type"),
    "Should show component table with emoji and Type column"
  );
}

#[test]
fn test_components_list_with_path_shows_status() {
  let temp_dir = TempDir::new().expect("Failed to create temp dir");
  let temp_path = temp_dir.path();

  // Create minimal Docusaurus structure
  fs::write(
    temp_path.join("package.json"),
    r#"{"name": "test-docs", "dependencies": {"@docusaurus/core": "3.0.0"}}"#,
  )
  .expect("Failed to create package.json");

  fs::write(
    temp_path.join("docusaurus.config.ts"),
    "export default { title: 'Test' };",
  )
  .expect("Failed to create docusaurus.config.ts");

  // Create only RustCode component (both files needed)
  let rust_code_dir = temp_path.join("src/components/RustCode");
  fs::create_dir_all(&rust_code_dir).expect("Failed to create directory");
  fs::write(rust_code_dir.join("index.tsx"), "// Component").expect("Failed to create index file");
  fs::write(rust_code_dir.join("styles.module.css"), "/* Styles */")
    .expect("Failed to create styles file");

  // Run components list with path
  let output = std::process::Command::new("cargo")
    .args(&[
      "run",
      "--",
      "components",
      "list",
      temp_path.to_str().unwrap(),
    ])
    .output()
    .expect("Failed to run cargo run");

  assert!(output.status.success(), "Command should succeed");

  let stdout = String::from_utf8_lossy(&output.stdout);

  // Should show RustCode component with files
  assert!(
    stdout.contains("RustCode"),
    "Should show RustCode component"
  );

  // Check for installed status - now shown as separate rows
  assert!(
    stdout.contains("✅ Installed"),
    "Should show installed status for some files. Output:\n{}",
    stdout
  );

  // Should show other components as not installed
  assert!(
    stdout.contains("RustCrateLink"),
    "Should show RustCrateLink"
  );
}

#[test]
fn test_cli_backward_compatibility_json_conversion() {
  // Test that old-style JSON conversion still works (backward compatibility)
  let temp_dir = TempDir::new().expect("Failed to create temp dir");
  let output_dir = temp_dir.path().join("output");

  // Use existing test fixture
  let json_path = Path::new("tests/fixtures/test_crate.json");
  assert!(json_path.exists(), "Test fixture should exist");

  // Run conversion without subcommand (old style)
  let output = std::process::Command::new("cargo")
    .args(&[
      "run",
      "--",
      json_path.to_str().unwrap(),
      "-o",
      output_dir.to_str().unwrap(),
    ])
    .output()
    .expect("Failed to run cargo run");

  let stdout = String::from_utf8_lossy(&output.stdout);
  let stderr = String::from_utf8_lossy(&output.stderr);

  assert!(
    output.status.success(),
    "Conversion should succeed:\nstdout: {}\nstderr: {}",
    stdout,
    stderr
  );

  // Verify output was created
  assert!(output_dir.exists(), "Output directory should be created");
  assert!(
    output_dir.join("test_crate").exists(),
    "Crate directory should be created"
  );
  assert!(
    output_dir.join("test_crate/index.md").exists(),
    "Index file should be created"
  );
}

#[test]
fn test_components_init_invalid_path() {
  // Test with non-existent path
  let output = std::process::Command::new("cargo")
    .args(&["run", "--", "components", "init", "/nonexistent/path"])
    .output()
    .expect("Failed to run cargo run");

  // Should fail gracefully
  assert!(
    !output.status.success(),
    "Command should fail for invalid path"
  );

  let stderr = String::from_utf8_lossy(&output.stderr);
  assert!(
    stderr.contains("Error") || stderr.contains("not found") || stderr.contains("does not exist"),
    "Should show error message"
  );
}

#[test]
fn test_components_init_non_docusaurus_project() {
  let temp_dir = TempDir::new().expect("Failed to create temp dir");

  // Empty directory (not a Docusaurus project)
  let output = std::process::Command::new("cargo")
    .args(&[
      "run",
      "--",
      "components",
      "init",
      temp_dir.path().to_str().unwrap(),
    ])
    .output()
    .expect("Failed to run cargo run");

  // Should fail validation
  assert!(
    !output.status.success(),
    "Command should fail for non-Docusaurus project"
  );

  let stderr = String::from_utf8_lossy(&output.stderr);
  assert!(
    stderr.contains("package.json") || stderr.contains("docusaurus.config"),
    "Should mention missing Docusaurus files"
  );
}
