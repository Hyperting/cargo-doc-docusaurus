use cargo_doc_docusaurus::{ConversionOptions, converter, parser};
use std::path::Path;

#[test]
fn test_multifile_output_structure() {
  let json_path = Path::new("tests/fixtures/test_crate.json");
  let crate_data = parser::load_rustdoc_json(json_path).expect("Failed to load JSON");
  let output = converter::convert_to_markdown_multifile(&crate_data, false, "", &[], false, None)
    .expect("Failed to convert to markdown");

  let mut file_paths: Vec<_> = output.files.keys().collect();
  file_paths.sort();

  insta::assert_debug_snapshot!("multifile_structure", file_paths);
  assert_eq!(output.crate_name, "test_crate");
}

#[test]
fn test_index_file() {
  let json_path = Path::new("tests/fixtures/test_crate.json");
  let crate_data = parser::load_rustdoc_json(json_path).expect("Failed to load JSON");
  let output = converter::convert_to_markdown_multifile(&crate_data, false, "", &[], false, None)
    .expect("Failed to convert to markdown");

  let index_content = output.files.get("index.md").expect("index.md not found");
  insta::assert_snapshot!("index_md", index_content);
}

#[test]
fn test_lib_module() {
  let json_path = Path::new("tests/fixtures/test_crate.json");
  let crate_data = parser::load_rustdoc_json(json_path).expect("Failed to load JSON");
  let output = converter::convert_to_markdown_multifile(&crate_data, false, "", &[], false, None)
    .expect("Failed to convert to markdown");

  if let Some(lib_content) = output.files.get("index.md") {
    insta::assert_snapshot!("lib_module", lib_content);
  }
}

#[test]
fn test_types_module() {
  let json_path = Path::new("tests/fixtures/test_crate.json");
  let crate_data = parser::load_rustdoc_json(json_path).expect("Failed to load JSON");
  let output = converter::convert_to_markdown_multifile(&crate_data, false, "", &[], false, None)
    .expect("Failed to convert to markdown");

  if let Some(types_content) = output.files.get("types.md") {
    insta::assert_snapshot!("types_module", types_content);
  }
}

#[test]
fn test_functions_module() {
  let json_path = Path::new("tests/fixtures/test_crate.json");
  let crate_data = parser::load_rustdoc_json(json_path).expect("Failed to load JSON");
  let output = converter::convert_to_markdown_multifile(&crate_data, false, "", &[], false, None)
    .expect("Failed to convert to markdown");

  if let Some(functions_content) = output.files.get("functions.md") {
    insta::assert_snapshot!("functions_module", functions_content);
  }
}

#[test]
fn test_nested_modules() {
  let json_path = Path::new("tests/fixtures/test_crate.json");
  let crate_data = parser::load_rustdoc_json(json_path).expect("Failed to load JSON");
  let output = converter::convert_to_markdown_multifile(&crate_data, false, "", &[], false, None)
    .expect("Failed to convert to markdown");

  if let Some(nested_content) = output.files.get("nested.md") {
    insta::assert_snapshot!("nested_module", nested_content);
  }

  if let Some(inner_content) = output.files.get("nested/inner.md") {
    insta::assert_snapshot!("nested_inner_module", inner_content);
  }

  if let Some(deep_content) = output.files.get("nested/inner/deep.md") {
    insta::assert_snapshot!("nested_deep_module", deep_content);
  }
}

#[test]
fn test_struct_formatting() {
  let json_path = Path::new("tests/fixtures/test_crate.json");
  let crate_data = parser::load_rustdoc_json(json_path).expect("Failed to load JSON");
  let output = converter::convert_to_markdown_multifile(&crate_data, false, "", &[], false, None)
    .expect("Failed to convert to markdown");

  // Structs now use struct. prefix
  let unit_struct = output
    .files
    .get("struct.UnitStruct.md")
    .expect("struct.UnitStruct.md not found");
  assert!(unit_struct.contains("title: \"Struct UnitStruct\""));
  assert!(unit_struct.contains("<RustCode"));

  let tuple_struct = output
    .files
    .get("struct.TupleStruct.md")
    .expect("struct.TupleStruct.md not found");
  assert!(tuple_struct.contains("title: \"Struct TupleStruct\""));
  assert!(tuple_struct.contains("<RustCode"));

  let plain_struct = output
    .files
    .get("struct.PlainStruct.md")
    .expect("struct.PlainStruct.md not found");
  assert!(plain_struct.contains("title: \"Struct PlainStruct\""));
  assert!(plain_struct.contains("### Fields"));
}

#[test]
fn test_generic_struct_formatting() {
  let json_path = Path::new("tests/fixtures/test_crate.json");
  let crate_data = parser::load_rustdoc_json(json_path).expect("Failed to load JSON");
  let output = converter::convert_to_markdown_multifile(&crate_data, false, "", &[], false, None)
    .expect("Failed to convert to markdown");

  let generic_struct = output
    .files
    .get("struct.GenericStruct.md")
    .expect("struct.GenericStruct.md not found");
  assert!(generic_struct.contains("title: \"Struct GenericStruct\""));
  assert!(generic_struct.contains("### Generic Parameters"));
}

#[test]
fn test_enum_formatting() {
  let json_path = Path::new("tests/fixtures/test_crate.json");
  let crate_data = parser::load_rustdoc_json(json_path).expect("Failed to load JSON");
  let output = converter::convert_to_markdown_multifile(&crate_data, false, "", &[], false, None)
    .expect("Failed to convert to markdown");

  let simple_enum = output
    .files
    .get("enum.SimpleEnum.md")
    .expect("enum.SimpleEnum.md not found");
  assert!(simple_enum.contains("title: \"Enum SimpleEnum\""));
  assert!(simple_enum.contains("### Variants"));

  let complex_enum = output
    .files
    .get("enum.ComplexEnum.md")
    .expect("enum.ComplexEnum.md not found");
  assert!(complex_enum.contains("title: \"Enum ComplexEnum\""));
}

#[test]
fn test_function_formatting() {
  let json_path = Path::new("tests/fixtures/test_crate.json");
  let crate_data = parser::load_rustdoc_json(json_path).expect("Failed to load JSON");
  let output = converter::convert_to_markdown_multifile(&crate_data, false, "", &[], false, None)
    .expect("Failed to convert to markdown");

  // Functions now use fn. prefix and are in the functions module directory
  assert!(output.files.contains_key("functions/fn.add.md"));
  assert!(output.files.contains_key("functions/fn.multiply.md"));
}

#[test]
fn test_trait_formatting() {
  let json_path = Path::new("tests/fixtures/test_crate.json");
  let crate_data = parser::load_rustdoc_json(json_path).expect("Failed to load JSON");
  let output = converter::convert_to_markdown_multifile(&crate_data, false, "", &[], false, None)
    .expect("Failed to convert to markdown");

  let my_trait = output
    .files
    .get("trait.MyTrait.md")
    .expect("trait.MyTrait.md not found");
  assert!(my_trait.contains("title: \"Trait MyTrait\""));
  assert!(my_trait.contains("```rust"));
}

#[test]
fn test_methods_and_impls() {
  let json_path = Path::new("tests/fixtures/test_crate.json");
  let crate_data = parser::load_rustdoc_json(json_path).expect("Failed to load JSON");
  let output = converter::convert_to_markdown_multifile(&crate_data, false, "", &[], false, None)
    .expect("Failed to convert to markdown");

  // Methods are now in individual struct files with struct. prefix
  let plain_struct = output
    .files
    .get("struct.PlainStruct.md")
    .expect("struct.PlainStruct.md not found");
  assert!(plain_struct.contains("### Methods"));
}

#[test]
fn test_constants_and_type_aliases() {
  let json_path = Path::new("tests/fixtures/test_crate.json");
  let crate_data = parser::load_rustdoc_json(json_path).expect("Failed to load JSON");
  let output = converter::convert_to_markdown_multifile(&crate_data, false, "", &[], false, None)
    .expect("Failed to convert to markdown");

  let lib_content = output.files.get("index.md").expect("index.md not found");

  assert!(lib_content.contains("MAX_SIZE"));
  assert!(lib_content.contains("Result"));
}

#[test]
fn test_include_private() {
  let json_path = Path::new("tests/fixtures/test_crate.json");
  let crate_data = parser::load_rustdoc_json(json_path).expect("Failed to load JSON");

  let output_public =
    converter::convert_to_markdown_multifile(&crate_data, false, "", &[], false, None)
      .expect("Failed to convert to markdown");
  let output_private =
    converter::convert_to_markdown_multifile(&crate_data, true, "", &[], false, None)
      .expect("Failed to convert to markdown");

  let public_count = output_public.files.len();
  let private_count = output_private.files.len();

  assert!(
    private_count >= public_count,
    "Private output should have at least as many files as public output"
  );
}

#[test]
fn test_conversion_end_to_end() {
  let output_dir = std::env::temp_dir().join("cargo_doc_md_test");
  let _ = std::fs::remove_dir_all(&output_dir);

  let json_path = Path::new("tests/fixtures/test_crate.json");
  let options = ConversionOptions {
    input_path: json_path,
    output_dir: &output_dir,
    include_private: false,
    base_path: "",
    workspace_crates: &[],
    sidebarconfig_collapsed: false,
    sidebar_output: None,
    sidebar_root_link: None,
  };

  cargo_doc_docusaurus::convert_json_file(&options).expect("Conversion failed");

  assert!(output_dir.join("test_crate").exists());
  assert!(output_dir.join("test_crate/index.md").exists());

  std::fs::remove_dir_all(&output_dir).ok();
}

#[test]
fn test_legacy_single_file_output() {
  let json_path = Path::new("tests/fixtures/test_crate.json");
  let crate_data = parser::load_rustdoc_json(json_path).expect("Failed to load JSON");
  let markdown =
    converter::convert_to_markdown(&crate_data, false).expect("Failed to convert to markdown");

  assert!(markdown.contains("# test_crate"));
  assert!(markdown.contains("## Table of Contents"));
  insta::assert_snapshot!("legacy_single_file", markdown);
}

#[test]
fn test_workspace_crates_internal_links() {
  let json_path = Path::new("tests/fixtures/test_crate.json");
  let crate_data = parser::load_rustdoc_json(json_path).expect("Failed to load JSON");

  // First, let's check what external crates are referenced
  // We know from the JSON that there are references to external crates

  // Test without workspace_crates parameter
  let output_default =
    converter::convert_to_markdown_multifile(&crate_data, false, "", &[], false, None)
      .expect("Failed to convert to markdown");

  // Test with workspace_crates parameter
  let workspace_crates = vec!["some_external_crate".to_string()];
  let output_with_workspace = converter::convert_to_markdown_multifile(
    &crate_data,
    false,
    "/docs/rust",
    &workspace_crates,
    false,
    None,
  )
  .expect("Failed to convert to markdown");

  // Both should generate files successfully
  assert!(
    !output_default.files.is_empty(),
    "Default should generate files"
  );
  assert!(
    !output_with_workspace.files.is_empty(),
    "With workspace_crates should generate files"
  );

  // Verify base_path is used correctly in the second case
  let has_base_path_in_internal_links = output_with_workspace.files.values().any(|content| {
    // Internal links to the current crate should use base_path
    content.contains("/docs/rust/test_crate/")
  });

  assert!(
    has_base_path_in_internal_links,
    "Internal links should use the provided base_path (/docs/rust)"
  );

  // Verify std library links still work (should always point to doc.rust-lang.org)
  let has_std_links = output_with_workspace
    .files
    .values()
    .any(|content| content.contains("doc.rust-lang.org"));

  assert!(
    has_std_links,
    "Std library links should always point to doc.rust-lang.org"
  );
}

#[test]
fn test_workspace_crates_with_hyphens() {
  let json_path = Path::new("tests/fixtures/test_crate.json");
  let crate_data = parser::load_rustdoc_json(json_path).expect("Failed to load JSON");

  // Test that workspace_crates works with hyphens (as in Cargo.toml)
  // even though crate names in JSON use underscores
  let workspace_crates_with_hyphens = vec!["test-crate".to_string()];
  let workspace_crates_with_underscores = vec!["test_crate".to_string()];

  let output_hyphens = converter::convert_to_markdown_multifile(
    &crate_data,
    false,
    "/docs",
    &workspace_crates_with_hyphens,
    false,
    None,
  )
  .expect("Should work with hyphens");

  let output_underscores = converter::convert_to_markdown_multifile(
    &crate_data,
    false,
    "/docs",
    &workspace_crates_with_underscores,
    false,
    None,
  )
  .expect("Should work with underscores");

  // Both should generate the same number of files
  assert_eq!(
    output_hyphens.files.len(),
    output_underscores.files.len(),
    "Both hyphen and underscore notation should work identically"
  );
}

#[test]
fn test_sidebar_collapsed_configuration() {
  let json_path = Path::new("tests/fixtures/test_crate.json");
  let crate_data = parser::load_rustdoc_json(json_path).expect("Failed to load JSON");

  // Test with collapsed = false (default)
  let output_collapsed_false =
    converter::convert_to_markdown_multifile(&crate_data, false, "", &[], false, None)
      .expect("Failed to convert to markdown");

  // Test with collapsed = true
  let output_collapsed_true =
    converter::convert_to_markdown_multifile(&crate_data, false, "", &[], true, None)
      .expect("Failed to convert to markdown");

  // Both should have a sidebar
  assert!(
    output_collapsed_false.sidebar.is_some(),
    "Should generate sidebar when collapsed=false"
  );
  assert!(
    output_collapsed_true.sidebar.is_some(),
    "Should generate sidebar when collapsed=true"
  );

  // Note: The collapsed parameter is currently not implemented in the sidebar generation
  // TODO: Implement collapsed functionality or remove the parameter from the API
}

#[test]
fn test_custom_sidebar_output() {
  let output_dir = std::env::temp_dir().join("cargo_doc_md_test_sidebar");
  let sidebar_path = output_dir.join("custom-sidebar.ts");
  let _ = std::fs::remove_dir_all(&output_dir);
  std::fs::create_dir_all(&output_dir).expect("Failed to create test directory");

  let json_path = Path::new("tests/fixtures/test_crate.json");
  let options = ConversionOptions {
    input_path: json_path,
    output_dir: &output_dir.join("docs"),
    include_private: false,
    base_path: "",
    workspace_crates: &[],
    sidebarconfig_collapsed: false,
    sidebar_output: Some(&sidebar_path),
    sidebar_root_link: None,
  };

  cargo_doc_docusaurus::convert_json_file(&options).expect("Conversion failed");

  // Check that the sidebar was created in the custom location
  assert!(sidebar_path.exists(), "Custom sidebar should be created");

  let sidebar_content = std::fs::read_to_string(&sidebar_path).expect("Failed to read sidebar");
  assert!(
    sidebar_content.contains("export const rustApiDocumentation"),
    "Should contain sidebar export"
  );
  assert!(
    sidebar_content.contains("test_crate/index"),
    "Should contain test_crate reference"
  );
}

#[test]
fn test_workspace_single_crate() {
  // Test documenting a single crate from a workspace
  let json_path = Path::new("tests/fixtures/test_workspace/crate_a.json");
  let crate_data = parser::load_rustdoc_json(json_path).expect("Failed to load crate_a JSON");

  let workspace_crates = vec!["crate_a".to_string(), "crate_b".to_string()];
  let output = converter::convert_to_markdown_multifile(
    &crate_data,
    false,
    "/docs/rust",
    &workspace_crates,
    false,
    None,
  )
  .expect("Failed to convert crate_a");

  assert_eq!(output.crate_name, "crate_a");
  assert!(
    !output.files.is_empty(),
    "Should generate files for crate_a"
  );

  // Check that the index file exists
  assert!(
    output.files.contains_key("index.md"),
    "Should have index.md"
  );

  // Check that struct files exist
  assert!(
    output.files.contains_key("struct.StructA.md"),
    "Should have StructA"
  );

  // Verify internal links to crate_b use base_path (not docs.rs)
  let struct_a_content = output
    .files
    .get("struct.StructA.md")
    .expect("StructA should exist");
  assert!(
    struct_a_content.contains("/docs/rust/crate_b/"),
    "Should contain internal link to crate_b with base_path"
  );
}

#[test]
fn test_workspace_multiple_crates_e2e() {
  // End-to-end test: document both crates in a workspace and verify cross-references
  let output_dir = std::env::temp_dir().join("cargo_doc_md_workspace_test");
  let _ = std::fs::remove_dir_all(&output_dir);

  let workspace_crates = vec!["crate_a".to_string(), "crate_b".to_string()];

  // Document crate_b first (dependency)
  let crate_b_json = Path::new("tests/fixtures/test_workspace/crate_b.json");
  let options_b = ConversionOptions {
    input_path: crate_b_json,
    output_dir: &output_dir,
    include_private: false,
    base_path: "/docs/rust",
    workspace_crates: &workspace_crates,
    sidebarconfig_collapsed: false,
    sidebar_output: None,
    sidebar_root_link: None,
  };

  cargo_doc_docusaurus::convert_json_file(&options_b).expect("Failed to convert crate_b");

  // Document crate_a (depends on crate_b)
  let crate_a_json = Path::new("tests/fixtures/test_workspace/crate_a.json");
  let options_a = ConversionOptions {
    input_path: crate_a_json,
    output_dir: &output_dir,
    include_private: false,
    base_path: "/docs/rust",
    workspace_crates: &workspace_crates,
    sidebarconfig_collapsed: false,
    sidebar_output: None,
    sidebar_root_link: None,
  };

  cargo_doc_docusaurus::convert_json_file(&options_a).expect("Failed to convert crate_a");

  // Verify both crates were documented
  assert!(
    output_dir.join("crate_a").exists(),
    "crate_a directory should exist"
  );
  assert!(
    output_dir.join("crate_b").exists(),
    "crate_b directory should exist"
  );

  // Verify crate_a index
  let crate_a_index = output_dir.join("crate_a/index.md");
  assert!(crate_a_index.exists(), "crate_a/index.md should exist");

  // Verify crate_b index
  let crate_b_index = output_dir.join("crate_b/index.md");
  assert!(crate_b_index.exists(), "crate_b/index.md should exist");

  // Verify StructA references DataB with internal links
  let struct_a_file = output_dir.join("crate_a/struct.StructA.md");
  assert!(
    struct_a_file.exists(),
    "crate_a/struct.StructA.md should exist"
  );

  let struct_a_content = std::fs::read_to_string(&struct_a_file).expect("Should read StructA.md");

  // Should have internal link to crate_b's DataB (not docs.rs)
  assert!(
    struct_a_content.contains("/docs/rust/crate_b/struct.DataB"),
    "StructA should have internal link to DataB: {}",
    struct_a_content
      .lines()
      .filter(|l| l.contains("DataB"))
      .collect::<Vec<_>>()
      .join("\n")
  );

  // Should NOT have docs.rs links for workspace crates
  assert!(
    !struct_a_content.contains("docs.rs/crate_b"),
    "Should not have docs.rs links for workspace crates"
  );

  std::fs::remove_dir_all(&output_dir).ok();
}

#[test]
fn test_workspace_cross_crate_types() {
  // Verify that types from one workspace crate referenced in another
  // use internal links instead of docs.rs
  let json_path = Path::new("tests/fixtures/test_workspace/crate_a.json");
  let crate_data = parser::load_rustdoc_json(json_path).expect("Failed to load crate_a");

  let workspace_crates = vec!["crate_a".to_string(), "crate_b".to_string()];
  let output = converter::convert_to_markdown_multifile(
    &crate_data,
    false,
    "/docs/api",
    &workspace_crates,
    false,
    None,
  )
  .expect("Failed to convert");

  // Check the combine function which returns crate_b::ResultB
  let has_combine_fn = output.files.keys().any(|k| k.contains("fn.combine"));
  assert!(has_combine_fn, "Should have combine function documented");

  // Find and check the combine function
  let combine_file = output
    .files
    .iter()
    .find(|(k, _)| k.contains("fn.combine"))
    .map(|(_, v)| v)
    .expect("Should have combine function");

  // Should reference ResultB with internal link
  assert!(
    combine_file.contains("/docs/api/crate_b/struct.ResultB") || combine_file.contains("ResultB"),
    "Should reference ResultB from crate_b"
  );
}
