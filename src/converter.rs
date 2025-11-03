//! Markdown converter for rustdoc JSON data.

use anyhow::Result;
use rustdoc_types::{Crate, Id, Item, ItemEnum, Visibility};
use std::cell::RefCell;
use std::collections::HashMap;

thread_local! {
    /// Thread-local storage for the base path to use in generated links
    static BASE_PATH: RefCell<String> = const { RefCell::new(String::new()) };
    /// Thread-local storage for workspace crate names
    static WORKSPACE_CRATES: RefCell<Vec<String>> = const { RefCell::new(Vec::new()) };
    /// Thread-local storage for the sidebar root link URL
    static SIDEBAR_ROOT_LINK: RefCell<Option<String>> = const { RefCell::new(None) };
}

/// Represents the multi-file markdown output
pub struct MarkdownOutput {
  /// Crate name
  pub crate_name: String,
  /// Map of relative file path -> content
  pub files: HashMap<String, String>,
  /// Sidebar configuration (optional, for Docusaurus)
  pub sidebar: Option<String>,
}

/// Represents a sidebar item for Docusaurus
#[derive(Debug, Clone)]
enum SidebarItem {
  /// A document reference with optional label
  Doc {
    id: String,
    label: Option<String>,
    custom_props: Option<String>, // Can be either className or customProps JSON
  },
  /// A link item (for dynamic sidebars)
  Link {
    href: String,
    label: String,
    custom_props: Option<String>,
  },
  /// A category with sub-items
  Category {
    label: String,
    items: Vec<SidebarItem>,
    collapsed: bool,
    link: Option<String>, // Optional link to make category clickable
  },
}

/// Convert a rustdoc Crate to multi-file markdown format.
pub fn convert_to_markdown_multifile(
  crate_data: &Crate,
  include_private: bool,
  base_path: &str,
  workspace_crates: &[String],
  sidebarconfig_collapsed: bool,
  sidebar_root_link: Option<&str>,
) -> Result<MarkdownOutput> {
  // Set the base path, workspace crates, and sidebar root link for this conversion in thread-local storage
  BASE_PATH.with(|bp| *bp.borrow_mut() = base_path.to_string());
  WORKSPACE_CRATES.with(|wc| *wc.borrow_mut() = workspace_crates.to_vec());
  SIDEBAR_ROOT_LINK.with(|srl| *srl.borrow_mut() = sidebar_root_link.map(|s| s.to_string()));

  let root_item = crate_data
    .index
    .get(&crate_data.root)
    .ok_or_else(|| anyhow::anyhow!("Root item not found in index"))?;

  let crate_name = root_item.name.as_deref().unwrap_or("unknown");

  // Build a map of item_id -> full_path using the paths data
  let item_paths = build_path_map(crate_data);

  // Group items by module (no longer duplicating re-exports)
  let mut modules = group_by_module(crate_data, &item_paths, include_private);

  // Build a map of re-exported modules (module_path -> list of re-exported submodule paths)
  let reexported_modules = build_reexported_modules(crate_data, &item_paths, include_private);

  let mut files = HashMap::new();

  // Check if we have items in the root crate
  let root_module_key = crate_name.to_string();
  let has_root_items = modules.contains_key(&root_module_key);

  // First, ensure ALL modules (including empty ones) are in the modules map
  // This is crucial for generating index.md files and sidebar entries for parent modules
  // that only contain submodules (prevents Docusaurus referencing missing doc ids).
  for (_id, item) in &crate_data.index {
    if let ItemEnum::Module(_) = &item.inner {
      if let Some(path) = item_paths.get(_id) {
        let module_path = path.join("::");
        // Ensure this module exists in the map (even if empty)
        modules.entry(module_path).or_default();
      }
    }
  }

  // Build module hierarchy to determine which modules have submodules
  let module_hierarchy = build_module_hierarchy(&modules, crate_name);

  // Ensure parent modules from hierarchy are present in `modules` so we
  // generate index pages and matching sidebar keys for parent modules that
  // only contain submodules (prevents Docusaurus referencing missing doc ids).
  // This is a minimal change: it inserts empty item lists for parents that
  // don't already exist so `generate_module_overview` will still create
  // a corresponding `index.md` file.
  for parent in module_hierarchy.keys() {
    modules.entry(parent.clone()).or_default();
  }

  // Generate index.md - either with crate overview or with root module content
  if has_root_items {
    // If there are items in the root module, combine crate overview with root content
    let root_items = &modules[&root_module_key];
    let index_content = generate_combined_crate_and_root_content(
      crate_name,
      root_item,
      crate_data,
      &modules,
      root_items,
      &module_hierarchy,
      &reexported_modules,
    );
    files.insert("index.md".to_string(), index_content);
  } else {
    // Just crate overview if no root items
    let index_content = generate_crate_index(crate_name, root_item, &modules);
    files.insert("index.md".to_string(), index_content);
  }

  // Generate overview files and individual pages for each module
  for (module_name, items) in &modules {
    // Skip the root module as it's already handled in index.md
    if module_name == &root_module_key {
      // Generate individual pages for root-level items
      generate_individual_pages(
        items,
        "",
        &mut files,
        crate_data,
        &item_paths,
        crate_name,
        crate_name,
        include_private,
      );
      continue;
    }

    let module_filename = module_name
      .strip_prefix(&format!("{}::", crate_name))
      .unwrap_or(module_name)
      .replace("::", "/");

    // Always generate module overview (even if items are re-exported)
    // This ensures all modules are navigable
    let overview_path = format!("{}/index.md", module_filename);

    // Generate module overview page (index-style)
    let module_overview = generate_module_overview(
      module_name,
      items, // Use direct items only, not all recursive items
      crate_data,
      &item_paths,
      crate_name,
      &module_hierarchy,
    );
    files.insert(overview_path.clone(), module_overview);

    // Always generate individual pages for items
    // All modules use subdirectories, so items go in the module directory
    let item_prefix = if module_filename.is_empty() {
      String::new()
    } else {
      format!("{}/", module_filename)
    };
    generate_individual_pages(
      items,
      &item_prefix,
      &mut files,
      crate_data,
      &item_paths,
      crate_name,
      module_name,
      include_private,
    );
  }

  // Generate sidebar structure with sidebars for each module
  let sidebar = generate_all_sidebars(
    crate_name,
    &modules,
    &item_paths,
    crate_data,
    sidebarconfig_collapsed,
  );

  Ok(MarkdownOutput {
    crate_name: crate_name.to_string(),
    files,
    sidebar: Some(sidebar),
  })
}

/// Convert a rustdoc Crate to markdown format (legacy single-file).
pub fn convert_to_markdown(crate_data: &Crate, include_private: bool) -> Result<String> {
  let mut output = String::new();

  let root_item = crate_data
    .index
    .get(&crate_data.root)
    .ok_or_else(|| anyhow::anyhow!("Root item not found in index"))?;

  let crate_name = root_item.name.as_deref().unwrap_or("unknown");
  output.push_str(&format!("# {}\n\n", crate_name));

  if let Some(docs) = &root_item.docs {
    output.push_str(&format!("{}\n\n", docs));
  }

  // Build a map of item_id -> full_path using the paths data
  let item_paths = build_path_map(crate_data);

  // Group items by module
  let modules = group_by_module(crate_data, &item_paths, include_private);

  // Generate hierarchical ToC
  output.push_str("## Table of Contents\n\n");
  output.push_str(&generate_toc(&modules, crate_name));
  output.push_str("\n\n---\n\n");

  // Generate content organized by module
  output.push_str(&generate_content(
    &modules,
    crate_data,
    &item_paths,
    include_private,
  ));

  Ok(output)
}

fn build_path_map(crate_data: &Crate) -> HashMap<Id, Vec<String>> {
  crate_data
    .paths
    .iter()
    .map(|(id, summary)| (*id, summary.path.clone()))
    .collect()
}

fn build_module_hierarchy(
  modules: &HashMap<String, Vec<(Id, Item)>>,
  crate_name: &str,
) -> HashMap<String, Vec<String>> {
  let mut hierarchy: HashMap<String, Vec<String>> = HashMap::new();

  for module_name in modules.keys() {
    // Skip the root crate module itself
    if module_name == crate_name {
      continue;
    }

    // Extract the relative module path
    let relative_path = module_name
      .strip_prefix(&format!("{}::", crate_name))
      .unwrap_or(module_name);

    // Split into components
    let components: Vec<&str> = relative_path.split("::").collect();

    // Handle top-level modules (direct children of crate root)
    if components.len() == 1 {
      hierarchy
        .entry(crate_name.to_string())
        .or_default()
        .push(module_name.clone());
    }

    // For each component, check if it's a parent of this module
    for i in 0..components.len() {
      let parent_path = if i == 0 {
        format!("{}::{}", crate_name, components[0])
      } else {
        let parent_components = &components[0..=i];
        format!("{}::{}", crate_name, parent_components.join("::"))
      };

      // If this is not the full path, it's a parent
      if parent_path != *module_name && components.len() > i + 1 {
        let child_path = if i + 1 < components.len() - 1 {
          let child_components = &components[0..=i + 1];
          format!("{}::{}", crate_name, child_components.join("::"))
        } else {
          module_name.clone()
        };

        hierarchy.entry(parent_path).or_default().push(child_path);
      }
    }
  }

  // Deduplicate children
  for children in hierarchy.values_mut() {
    children.sort();
    children.dedup();
  }

  hierarchy
}

/// Build a map of re-exported modules
/// Returns: parent_module_path -> list of (child_module_name, child_module_full_path)
fn build_reexported_modules(
  crate_data: &Crate,
  item_paths: &HashMap<Id, Vec<String>>,
  include_private: bool,
) -> HashMap<String, Vec<(String, String)>> {
  let mut reexports: HashMap<String, Vec<(String, String)>> = HashMap::new();

  // Iterate through all modules to find their Use items
  for (module_id, module_item) in &crate_data.index {
    if let ItemEnum::Module(module_data) = &module_item.inner {
      // Get the module path
      let module_path = if let Some(path) = item_paths.get(module_id) {
        path.join("::")
      } else {
        continue;
      };

      // Process all items in this module to find re-exports
      for item_id in &module_data.items {
        if let Some(item) = crate_data.index.get(item_id) {
          if let ItemEnum::Use(import) = &item.inner {
            // Only process public re-exports
            if !include_private && !is_public(item) {
              continue;
            }

            // Try to find the imported item
            if let Some(imported_id) = &import.id {
              if let Some(imported_item) = crate_data.index.get(imported_id) {
                // Check if this is a glob import
                if import.is_glob {
                  // Glob re-export - find all public submodules
                  if let ItemEnum::Module(source_module_data) = &imported_item.inner {
                    for source_item_id in &source_module_data.items {
                      if let Some(source_item) = crate_data.index.get(source_item_id) {
                        // Only process modules
                        if let ItemEnum::Module(_) = &source_item.inner {
                          // Only add public modules
                          if !include_private && !is_public(source_item) {
                            continue;
                          }

                          if let Some(source_item_name) = &source_item.name {
                            // Get the full path of the source module
                            if let Some(source_path) = item_paths.get(source_item_id) {
                              let source_full_path = source_path.join("::");
                              reexports
                                .entry(module_path.clone())
                                .or_default()
                                .push((source_item_name.clone(), source_full_path));
                            }
                          }
                        }
                      }
                    }
                  }
                } else {
                  // Single module re-export
                  if let ItemEnum::Module(_) = &imported_item.inner {
                    if let Some(imported_name) = &imported_item.name {
                      if let Some(imported_path) = item_paths.get(imported_id) {
                        let imported_full_path = imported_path.join("::");
                        reexports
                          .entry(module_path.clone())
                          .or_default()
                          .push((imported_name.clone(), imported_full_path));
                      }
                    }
                  }
                }
              }
            }
          }
        }
      }
    }
  }

  // Deduplicate
  for list in reexports.values_mut() {
    list.sort();
    list.dedup();
  }

  reexports
}

/// Check if all items in a module are re-exported in its parent module
fn group_by_module(
  crate_data: &Crate,
  item_paths: &HashMap<Id, Vec<String>>,
  include_private: bool,
) -> HashMap<String, Vec<(Id, Item)>> {
  let mut modules: HashMap<String, Vec<(Id, Item)>> = HashMap::new();

  for (id, item) in &crate_data.index {
    if id == &crate_data.root {
      continue;
    }

    if !include_private && !is_public(item) {
      continue;
    }

    // Skip if we can't format this item type
    if !can_format_item(item) {
      continue;
    }

    // Get the module path (all elements except the last one)
    let module_path = if let Some(path) = item_paths.get(id) {
      if path.len() > 1 {
        // Item is in a submodule
        path[..path.len() - 1].join("::")
      } else if path.len() == 1 {
        // Item is at the root of the crate - use crate name as the module path
        path[0].clone()
      } else {
        continue; // Skip items with empty path
      }
    } else {
      continue; // Skip items without path info
    };

    modules
      .entry(module_path)
      .or_default()
      .push((*id, item.clone()));
  }

  // Process re-exports (ItemEnum::Use)
  // For glob re-exports (pub use module::*), we generate duplicate files like rustdoc does
  // For simple re-exports, we only show the link in the Re-exports section
  for (module_id, module_item) in &crate_data.index {
    if let ItemEnum::Module(module_data) = &module_item.inner {
      // Get the module path
      let module_path = if let Some(path) = item_paths.get(module_id) {
        path.join("::")
      } else {
        continue;
      };

      // Process all items in this module
      for item_id in &module_data.items {
        if let Some(item) = crate_data.index.get(item_id) {
          // Check if this item is a re-export
          if let ItemEnum::Use(import) = &item.inner {
            // Only process public re-exports
            if !include_private && !is_public(item) {
              continue;
            }

            // Always add the Use item itself for the Re-exports section
            modules
              .entry(module_path.clone())
              .or_default()
              .push((*item_id, item.clone()));

            // For glob re-exports (pub use module::*), also add all re-exported items
            // This matches rustdoc's behavior of generating duplicate documentation
            if import.is_glob {
              if let Some(imported_id) = &import.id {
                // Prevent self-referential re-exports (e.g., pub use self::*)
                if imported_id == module_id {
                  continue;
                }

                // Resolve the re-export chain to find the final item
                let mut visited = std::collections::HashSet::new();
                if let Some((resolved_id, imported_item)) =
                  resolve_reexport_chain(imported_id, crate_data, 0, &mut visited)
                {
                  if let ItemEnum::Module(imported_module_data) = &imported_item.inner {
                    // Get the imported module path to check for circular references
                    let imported_module_path = item_paths.get(&resolved_id).map(|p| p.join("::"));

                    // Skip if the imported module is a parent of the current module
                    // (prevents infinite loops with circular re-exports)
                    if let Some(imported_path) = &imported_module_path {
                      if module_path.starts_with(&format!("{}::", imported_path)) {
                        continue;
                      }
                    }

                    // Add all items from the imported module
                    for imported_item_id in &imported_module_data.items {
                      if let Some(imported_item) = crate_data.index.get(imported_item_id) {
                        // Skip if not public (unless include_private is true)
                        if !include_private && !is_public(imported_item) {
                          continue;
                        }

                        // Skip if we can't format this item type
                        if !can_format_item(imported_item) {
                          continue;
                        }

                        // Skip Use items within the glob to avoid nested re-exports
                        if matches!(imported_item.inner, ItemEnum::Use(_)) {
                          continue;
                        }

                        // Skip Module items to avoid duplicating module definitions
                        if matches!(imported_item.inner, ItemEnum::Module(_)) {
                          continue;
                        }

                        // Add the imported item to this module
                        modules
                          .entry(module_path.clone())
                          .or_default()
                          .push((*imported_item_id, imported_item.clone()));
                      }
                    }
                  }
                }
              }
            }
          }
        }
      }
    }
  }

  // Sort items within each module by name and remove duplicates
  for items in modules.values_mut() {
    items.sort_by(|a, b| {
      let name_a = a.1.name.as_deref().unwrap_or("");
      let name_b = b.1.name.as_deref().unwrap_or("");
      name_a.cmp(name_b)
    });
    // Remove duplicates (same ID)
    items.dedup_by(|a, b| a.0 == b.0);
  }

  modules
}

fn can_format_item(item: &Item) -> bool {
  matches!(
    item.inner,
    ItemEnum::Struct(_)
      | ItemEnum::Enum(_)
      | ItemEnum::Function(_)
      | ItemEnum::Trait(_)
      | ItemEnum::Module(_)
      | ItemEnum::Constant { .. }
      | ItemEnum::TypeAlias(_)
  )
}

/// Get the rustdoc-style prefix for an item type (e.g., "fn.", "struct.", etc.)
fn get_item_prefix(item: &Item) -> &'static str {
  match &item.inner {
    ItemEnum::Function(_) => "fn.",
    ItemEnum::Struct(_) => "struct.",
    ItemEnum::Enum(_) => "enum.",
    ItemEnum::Trait(_) => "trait.",
    ItemEnum::Constant { .. } => "constant.",
    ItemEnum::TypeAlias(_) => "type.",
    ItemEnum::Module(_) => "", // Modules don't get a prefix
    _ => "",
  }
}

fn get_item_type_label(item: &Item) -> &'static str {
  match &item.inner {
    ItemEnum::Function(_) => "Function",
    ItemEnum::Struct(_) => "Struct",
    ItemEnum::Enum(_) => "Enum",
    ItemEnum::Trait(_) => "Trait",
    ItemEnum::Constant { .. } => "Constant",
    ItemEnum::TypeAlias(_) => "Type",
    ItemEnum::Module(_) => "Module",
    _ => "",
  }
}

fn generate_toc(modules: &HashMap<String, Vec<(Id, Item)>>, crate_name: &str) -> String {
  let mut toc = String::new();

  // Sort modules alphabetically
  let mut module_names: Vec<_> = modules.keys().collect();
  module_names.sort();

  for module_name in module_names {
    let items = &modules[module_name];

    // Get the last component of the module path for display
    let display_name = module_name
      .strip_prefix(&format!("{}::", crate_name))
      .unwrap_or(module_name);

    toc.push_str(&format!("- **{}**\n", display_name));

    for (_id, item) in items {
      if let Some(name) = &item.name {
        let full_path = format!("{}::{}", module_name, name);
        let anchor = full_path.to_lowercase().replace("::", "-");
        toc.push_str(&format!("  - [{}](#{})\n", name, anchor));
      }
    }
  }

  toc
}

fn generate_content(
  modules: &HashMap<String, Vec<(Id, Item)>>,
  crate_data: &Crate,
  item_paths: &HashMap<Id, Vec<String>>,
  include_private: bool,
) -> String {
  let mut output = String::new();

  // Sort modules alphabetically
  let mut module_names: Vec<_> = modules.keys().collect();
  module_names.sort();

  for module_name in module_names {
    let items = &modules[module_name];

    // Module header
    output.push_str(&format!("# Module: `{}`\n\n", module_name));

    // Generate content for each item in the module
    for (id, item) in items {
      if let Some(section) =
        format_item_with_path(id, item, crate_data, item_paths, include_private)
      {
        output.push_str(&section);
        output.push_str("\n\n");
      }
    }

    output.push_str("---\n\n");
  }

  output
}

fn format_item_with_path(
  item_id: &Id,
  item: &Item,
  crate_data: &Crate,
  item_paths: &HashMap<Id, Vec<String>>,
  include_private: bool,
) -> Option<String> {
  let full_path = item_paths.get(item_id)?;
  let full_name = full_path.join("::");

  let mut output = format_item(item_id, item, crate_data, include_private)?;

  // Replace the simple name header with the full path
  if let Some(name) = &item.name {
    let old_header = format!("## {}\n\n", name);
    let new_header = format!("## {}\n\n", full_name);
    output = output.replace(&old_header, &new_header);
  }

  Some(output)
}

fn is_public(item: &Item) -> bool {
  matches!(item.visibility, Visibility::Public)
}

/// Resolve a chain of re-exports to find the final item
/// Returns (final_id, final_item) if successful, None if the chain is circular or too deep
fn resolve_reexport_chain<'a>(
  item_id: &Id,
  crate_data: &'a Crate,
  depth: usize,
  visited: &mut std::collections::HashSet<Id>,
) -> Option<(Id, &'a Item)> {
  const MAX_DEPTH: usize = 10;

  if depth > MAX_DEPTH {
    return None;
  }

  if !visited.insert(*item_id) {
    // Circular reference detected
    return None;
  }

  if let Some(item) = crate_data.index.get(item_id) {
    if let ItemEnum::Use(import) = &item.inner {
      // This is a re-export, follow the chain
      if let Some(imported_id) = &import.id {
        return resolve_reexport_chain(imported_id, crate_data, depth + 1, visited);
      }
    }
    // Not a re-export, return the item
    Some((*item_id, item))
  } else {
    None
  }
}

/// Get the visibility indicator for an item (e.g., "🔒" for restricted visibility)
fn get_visibility_indicator(item: &Item) -> &'static str {
  match &item.visibility {
    Visibility::Public => "",
    _ => " 🔒", // Lock emoji for crate/restricted visibility
  }
}

/// Format a struct definition with links extracted
#[allow(clippy::single_char_add_str, clippy::manual_flatten)]
fn format_struct_definition_with_links(
  name: &str,
  s: &rustdoc_types::Struct,
  item: &Item,
  crate_data: &Crate,
  include_private: bool,
) -> (String, Vec<(String, String)>) {
  let mut code = String::new();
  let mut all_links = Vec::new();

  // Add visibility and struct keyword
  let visibility = match &item.visibility {
    rustdoc_types::Visibility::Public => "pub ",
    _ => "",
  };

  code.push_str(&format!("{}struct {}", visibility, name));

  // Add generic parameters
  let non_synthetic_params: Vec<_> = s
    .generics
    .params
    .iter()
    .filter(|p| {
      !matches!(&p.kind, rustdoc_types::GenericParamDefKind::Lifetime { .. })
        || !is_synthetic_lifetime(&p.name)
    })
    .collect();

  if !non_synthetic_params.is_empty() {
    code.push('<');
    let params: Vec<String> = non_synthetic_params
      .iter()
      .map(|p| p.name.clone())
      .collect();
    code.push_str(&params.join(", "));
    code.push('>');
  }

  // Add struct body based on kind
  match &s.kind {
    rustdoc_types::StructKind::Plain { fields, .. } => {
      if fields.is_empty() {
        #[allow(clippy::single_char_add_str)]
        code.push_str(";");
      } else {
        #[allow(clippy::single_char_add_str)]
        code.push_str(" {");
        for field_id in fields {
          if let Some(field) = crate_data.index.get(field_id) {
            if let Some(field_name) = &field.name {
              if let ItemEnum::StructField(ty) = &field.inner {
                // Show field visibility based on include_private flag
                let field_visibility = if include_private {
                  match &field.visibility {
                    rustdoc_types::Visibility::Public => "pub ",
                    rustdoc_types::Visibility::Crate => "pub(crate) ",
                    rustdoc_types::Visibility::Restricted { .. } => "",
                    rustdoc_types::Visibility::Default => "",
                  }
                } else {
                  match &field.visibility {
                    rustdoc_types::Visibility::Public => "pub ",
                    _ => continue,
                  }
                };

                let (field_type, links) = format_type_with_links(ty, crate_data, Some(item));
                all_links.extend(links);
                code.push_str(&format!(
                  "\n    {}{}: {},",
                  field_visibility, field_name, field_type
                ));
              }
            }
          }
        }
        code.push_str("\n}");
      }
    }
    rustdoc_types::StructKind::Tuple(fields) => {
      code.push('(');
      let mut visible_fields = Vec::new();
      for field_id in fields {
        if let Some(id) = field_id {
          if let Some(field) = crate_data.index.get(id) {
            if let ItemEnum::StructField(ty) = &field.inner {
              if include_private {
                let field_visibility = match &field.visibility {
                  rustdoc_types::Visibility::Public => "pub ",
                  rustdoc_types::Visibility::Crate => "pub(crate) ",
                  rustdoc_types::Visibility::Restricted { .. } => "",
                  rustdoc_types::Visibility::Default => "",
                };
                let (field_type, links) = format_type_with_links(ty, crate_data, Some(item));
                all_links.extend(links);
                if field_visibility.is_empty() {
                  visible_fields.push(field_type);
                } else {
                  visible_fields.push(format!("{}{}", field_visibility, field_type));
                }
              } else {
                match &field.visibility {
                  rustdoc_types::Visibility::Public => {
                    let (field_type, links) = format_type_with_links(ty, crate_data, Some(item));
                    all_links.extend(links);
                    visible_fields.push(format!("pub {}", field_type));
                  }
                  _ => continue,
                }
              }
            }
          }
        }
      }
      code.push_str(&visible_fields.join(", "));
      code.push_str(");");
    }
    rustdoc_types::StructKind::Unit => {
      code.push_str(";");
    }
  }

  (code, all_links)
}

/// Format an enum definition with links extracted
#[allow(clippy::manual_flatten)]
fn format_enum_definition_with_links(
  name: &str,
  e: &rustdoc_types::Enum,
  item: &Item,
  crate_data: &Crate,
) -> (String, Vec<(String, String)>) {
  let mut code = String::new();
  let mut all_links = Vec::new();

  // Add visibility and enum keyword
  let visibility = match &item.visibility {
    rustdoc_types::Visibility::Public => "pub ",
    _ => "",
  };

  code.push_str(&format!("{}enum {}", visibility, name));

  // Add generic parameters
  let non_synthetic_params: Vec<_> = e
    .generics
    .params
    .iter()
    .filter(|p| {
      !matches!(&p.kind, rustdoc_types::GenericParamDefKind::Lifetime { .. })
        || !is_synthetic_lifetime(&p.name)
    })
    .collect();

  if !non_synthetic_params.is_empty() {
    code.push('<');
    let params: Vec<String> = non_synthetic_params
      .iter()
      .map(|p| p.name.clone())
      .collect();
    code.push_str(&params.join(", "));
    code.push('>');
  }

  code.push_str(" {");

  // Add variants with their fields
  for variant_id in &e.variants {
    if let Some(variant) = crate_data.index.get(variant_id) {
      if let Some(variant_name) = &variant.name {
        code.push_str(&format!("\n    {}", variant_name));

        // Check if the variant has fields
        if let ItemEnum::Variant(variant_inner) = &variant.inner {
          match &variant_inner.kind {
            rustdoc_types::VariantKind::Plain => {
              // Unit variant, no fields
            }
            rustdoc_types::VariantKind::Tuple(field_ids) => {
              // Tuple variant with fields: Message(Type1, Type2)
              code.push('(');
              let mut field_types = Vec::new();
              for field_id in field_ids {
                if let Some(id) = field_id {
                  if let Some(field_item) = crate_data.index.get(id) {
                    if let ItemEnum::StructField(ty) = &field_item.inner {
                      let (type_str, links) = format_type_with_links(ty, crate_data, Some(item));
                      field_types.push(type_str);
                      all_links.extend(links);
                    }
                  }
                }
              }
              code.push_str(&field_types.join(", "));
              code.push(')');
            }
            rustdoc_types::VariantKind::Struct {
              fields,
              has_stripped_fields: _,
            } => {
              // Struct variant with named fields: Message { field1: Type1, field2: Type2 }
              code.push_str(" { ");
              let mut field_strs = Vec::new();
              for field_id in fields {
                if let Some(field_item) = crate_data.index.get(field_id) {
                  if let Some(field_name) = &field_item.name {
                    if let ItemEnum::StructField(ty) = &field_item.inner {
                      let (type_str, links) = format_type_with_links(ty, crate_data, Some(item));
                      field_strs.push(format!("{}: {}", field_name, type_str));
                      all_links.extend(links);
                    }
                  }
                }
              }
              code.push_str(&field_strs.join(", "));
              code.push_str(" }");
            }
          }
        }
        code.push(',');
      }
    }
  }

  code.push_str("\n}");

  (code, all_links)
}

/// Format a function definition with links extracted
#[allow(clippy::format_in_format_args)]
fn format_function_definition_with_links(
  name: &str,
  f: &rustdoc_types::Function,
  item: &Item,
  crate_data: &Crate,
) -> (String, Vec<(String, String)>) {
  let mut code = String::new();
  let mut all_links = Vec::new();

  // Collect generic parameters
  let generic_params: Vec<String> = if !f.generics.params.is_empty() {
    f.generics.params.iter().map(format_generic_param).collect()
  } else {
    Vec::new()
  };

  // Collect function inputs
  let mut inputs = Vec::new();
  for (param_name, ty) in &f.sig.inputs {
    let (type_str, links) = format_type_with_links(ty, crate_data, Some(item));
    all_links.extend(links);
    inputs.push(format!("{}: {}", param_name, type_str));
  }

  // Format on multiple lines if signature is too long (> 80 chars) or has many parameters (> 3)
  let single_line = format!(
    "fn {}{}",
    if !generic_params.is_empty() {
      format!("{}<{}>", name, generic_params.join(", "))
    } else {
      name.to_string()
    },
    format!("({})", inputs.join(", "))
  );

  if inputs.len() > 3 || single_line.len() > 80 {
    // Multi-line format
    code.push_str(&format!("fn {}", name));
    if !generic_params.is_empty() {
      code.push('<');
      code.push_str(&generic_params.join(", "));
      code.push('>');
    }
    code.push_str("(\n");
    for (i, input) in inputs.iter().enumerate() {
      code.push_str(&format!("    {}", input));
      if i < inputs.len() - 1 {
        code.push(',');
      }
      code.push('\n');
    }
    code.push(')');
  } else {
    // Single line format
    code.push_str(&format!("fn {}", name));
    if !generic_params.is_empty() {
      code.push('<');
      code.push_str(&generic_params.join(", "));
      code.push('>');
    }
    code.push('(');
    code.push_str(&inputs.join(", "));
    code.push(')');
  }

  if let Some(output_type) = &f.sig.output {
    let (type_str, links) = format_type_with_links(output_type, crate_data, Some(item));
    all_links.extend(links);
    code.push_str(&format!(" -> {}", type_str));
  }

  (code, all_links)
}

#[allow(clippy::single_char_add_str)]
fn format_item(
  item_id: &rustdoc_types::Id,
  item: &Item,
  crate_data: &Crate,
  include_private: bool,
) -> Option<String> {
  let name = item.name.as_ref()?;
  let mut output = String::new();

  match &item.inner {
    ItemEnum::Struct(s) => {
      // Format struct definition with links
      let (code, links) =
        format_struct_definition_with_links(name, s, item, crate_data, include_private);
      let links_json = format_links_as_json(&links);
      output.push_str(&format!(
        "<RustCode code={{`{}`}} links={{{}}} />\n\n",
        code, links_json
      ));

      if let Some(docs) = &item.docs {
        output.push_str(&format!("{}\n\n", sanitize_docs_for_mdx(docs)));
      }

      let non_synthetic_params: Vec<_> = s
        .generics
        .params
        .iter()
        .filter(|p| {
          !matches!(&p.kind, rustdoc_types::GenericParamDefKind::Lifetime { .. })
            || !is_synthetic_lifetime(&p.name)
        })
        .collect();

      if !non_synthetic_params.is_empty() {
        output.push_str("### Generic Parameters\n\n");
        for param in non_synthetic_params {
          output.push_str(&format!("- {}\n", format_generic_param(param)));
        }
        output.push('\n');
      }

      match &s.kind {
        rustdoc_types::StructKind::Plain { fields, .. } => {
          if !fields.is_empty() {
            // Filter fields based on include_private flag
            let visible_fields: Vec<_> = if include_private {
              fields.iter().collect()
            } else {
              fields
                .iter()
                .filter(|&field_id| {
                  if let Some(field) = crate_data.index.get(field_id) {
                    is_public(field)
                  } else {
                    false
                  }
                })
                .collect()
            };

            if !visible_fields.is_empty() {
              output.push_str("### Fields\n\n");
              for field_id in visible_fields {
                if let Some(field) = crate_data.index.get(field_id) {
                  if let Some(field_name) = &field.name {
                    let (type_str, type_links) = if let ItemEnum::StructField(ty) = &field.inner {
                      format_type_with_links(ty, crate_data, Some(item))
                    } else {
                      ("?".to_string(), Vec::new())
                    };

                    let field_sig = format!("{}: {}", field_name, type_str);
                    let links_json = format_links_as_json(&type_links);
                    output.push_str(&format!(
                      "<RustCode inline code={{`{}`}} links={{{}}} />\n\n",
                      field_sig, links_json
                    ));

                    if let Some(docs) = &field.docs {
                      let first_line = docs.lines().next().unwrap_or("").trim();
                      if !first_line.is_empty() {
                        output.push_str(&format!(
                          "<div className=\"rust-field-doc\">{}</div>\n\n",
                          first_line
                        ));
                      }
                    }
                  }
                }
              }
              output.push_str("\n");
            }
          }
        }
        rustdoc_types::StructKind::Tuple(fields) => {
          let types: Vec<String> = fields
            .iter()
            .filter_map(|field_id| {
              field_id.and_then(|id| {
                crate_data.index.get(&id).map(|field| {
                  if let ItemEnum::StructField(ty) = &field.inner {
                    format_type(ty, crate_data)
                  } else {
                    "?".to_string()
                  }
                })
              })
            })
            .collect();
          output.push_str(&format!("**Tuple Struct**: `({})`\n\n", types.join(", ")));
        }
        rustdoc_types::StructKind::Unit => {
          output.push_str("**Unit Struct**\n\n");
        }
      }

      let (inherent_impls, trait_impls) = collect_impls_for_type(item_id, crate_data);

      if !inherent_impls.is_empty() {
        output.push_str("### Methods\n\n");
        for impl_block in inherent_impls {
          let methods = format_impl_methods(impl_block, crate_data, Some(item));
          for (sig, links, doc) in methods {
            let links_json = format_links_as_json(&links);
            output.push_str(&format!(
              "<RustCode inline code={{`{}`}} links={{{}}} />\n\n",
              sig, links_json
            ));
            if let Some(doc) = doc {
              output.push_str(&format!("{}\n\n", doc));
            }
            output.push_str("---\n\n");
          }
        }
      }

      if !trait_impls.is_empty() {
        let user_impls: Vec<_> = trait_impls
          .iter()
          .filter(|impl_block| !impl_block.is_synthetic && impl_block.blanket_impl.is_none())
          .collect();

        if !user_impls.is_empty() {
          let mut derives = Vec::new();
          let mut trait_with_methods = Vec::new();

          for impl_block in user_impls {
            if let Some(trait_ref) = &impl_block.trait_ {
              let methods = format_impl_methods(impl_block, crate_data, Some(item));
              if methods.is_empty() {
                derives.push(trait_ref.path.as_str());
              } else {
                trait_with_methods.push((trait_ref, methods));
              }
            }
          }

          let public_derives: Vec<_> = derives
            .into_iter()
            .filter(|t| !is_compiler_internal_trait(t))
            .collect();

          if !public_derives.is_empty() {
            output.push_str("**Traits:** ");
            output.push_str(&public_derives.join(", "));
            output.push_str("\n\n");
          }

          if !trait_with_methods.is_empty() {
            output.push_str("### Trait Implementations\n\n");

            // Sort trait implementations alphabetically by trait path
            let mut sorted_trait_with_methods = trait_with_methods;
            sorted_trait_with_methods.sort_by(|a, b| a.0.path.cmp(&b.0.path));

            for (trait_ref, methods) in sorted_trait_with_methods {
              output.push_str(&format!("#### {}\n\n", trait_ref.path));
              for (sig, links, doc) in methods {
                let links_json = format_links_as_json(&links);
                output.push_str(&format!(
                  "<RustCode inline code={{`{}`}} links={{{}}} />\n\n",
                  sig, links_json
                ));
                if let Some(doc) = doc {
                  output.push_str(&format!("{}\n\n", doc));
                }
                output.push_str("---\n\n");
              }
            }
          }
        }
      }
    }
    ItemEnum::Enum(e) => {
      // Format enum definition with links
      let (code, links) = format_enum_definition_with_links(name, e, item, crate_data);
      let links_json = format_links_as_json(&links);
      output.push_str(&format!(
        "<RustCode code={{`{}`}} links={{{}}} />\n\n",
        code, links_json
      ));

      if let Some(docs) = &item.docs {
        output.push_str(&format!("{}\n\n", sanitize_docs_for_mdx(docs)));
      }

      let non_synthetic_params: Vec<_> = e
        .generics
        .params
        .iter()
        .filter(|p| {
          !matches!(&p.kind, rustdoc_types::GenericParamDefKind::Lifetime { .. })
            || !is_synthetic_lifetime(&p.name)
        })
        .collect();

      if !non_synthetic_params.is_empty() {
        output.push_str("### Generic Parameters\n\n");
        for param in non_synthetic_params {
          output.push_str(&format!("- {}\n", format_generic_param(param)));
        }
        output.push('\n');
      }

      if !e.variants.is_empty() {
        output.push_str("### Variants\n\n");
        for variant_id in &e.variants {
          if let Some(variant) = crate_data.index.get(variant_id) {
            if let Some(variant_name) = &variant.name {
              let variant_kind = if let ItemEnum::Variant(v) = &variant.inner {
                match &v.kind {
                  rustdoc_types::VariantKind::Plain => None,
                  rustdoc_types::VariantKind::Tuple(fields) => {
                    let types: Vec<_> = fields
                      .iter()
                      .map(|field_id| {
                        if let Some(id) = field_id {
                          if let Some(field_item) = crate_data.index.get(id) {
                            if let ItemEnum::StructField(ty) = &field_item.inner {
                              return format_type_plain(ty, crate_data);
                            }
                          }
                        }
                        "?".to_string()
                      })
                      .collect();
                    Some(format!("({})", types.join(", ")))
                  }
                  rustdoc_types::VariantKind::Struct { fields, .. } => {
                    let field_list: Vec<String> = fields
                      .iter()
                      .filter_map(|field_id| {
                        crate_data.index.get(field_id).and_then(|f| {
                          f.name.as_ref().map(|name| {
                            let field_type = if let ItemEnum::StructField(ty) = &f.inner {
                              format_type_plain(ty, crate_data)
                            } else {
                              "?".to_string()
                            };
                            format!("{}: {}", name, field_type)
                          })
                        })
                      })
                      .collect();
                    Some(format!("{{ {} }}", field_list.join(", ")))
                  }
                }
              } else {
                None
              };

              output.push_str("- `");
              output.push_str(variant_name);
              if let Some(kind) = variant_kind {
                output.push_str(&kind);
              }
              output.push('`');

              if let Some(docs) = &variant.docs {
                let first_line = docs.lines().next().unwrap_or("").trim();
                if !first_line.is_empty() {
                  output.push_str(&format!(" - {}", first_line));
                }
              }
              output.push('\n');
            }
          }
        }
        output.push('\n');
      }

      let (inherent_impls, trait_impls) = collect_impls_for_type(item_id, crate_data);

      if !inherent_impls.is_empty() {
        output.push_str("### Methods\n\n");
        for impl_block in inherent_impls {
          let methods = format_impl_methods(impl_block, crate_data, Some(item));
          for (sig, links, doc) in methods {
            let links_json = format_links_as_json(&links);
            output.push_str(&format!(
              "<RustCode inline code={{`{}`}} links={{{}}} />\n\n",
              sig, links_json
            ));
            if let Some(doc) = doc {
              output.push_str(&format!("{}\n\n", doc));
            }
            output.push_str("---\n\n");
          }
        }
      }

      if !trait_impls.is_empty() {
        let user_impls: Vec<_> = trait_impls
          .iter()
          .filter(|impl_block| !impl_block.is_synthetic && impl_block.blanket_impl.is_none())
          .collect();

        if !user_impls.is_empty() {
          let mut derives = Vec::new();
          let mut trait_with_methods = Vec::new();

          for impl_block in user_impls {
            if let Some(trait_ref) = &impl_block.trait_ {
              let methods = format_impl_methods(impl_block, crate_data, Some(item));
              if methods.is_empty() {
                derives.push(trait_ref.path.as_str());
              } else {
                trait_with_methods.push((trait_ref, methods));
              }
            }
          }

          let public_derives: Vec<_> = derives
            .into_iter()
            .filter(|t| !is_compiler_internal_trait(t))
            .collect();

          if !public_derives.is_empty() {
            output.push_str("**Traits:** ");
            output.push_str(&public_derives.join(", "));
            output.push_str("\n\n");
          }

          if !trait_with_methods.is_empty() {
            output.push_str("### Trait Implementations\n\n");

            // Sort trait implementations alphabetically by trait path
            let mut sorted_trait_with_methods = trait_with_methods;
            sorted_trait_with_methods.sort_by(|a, b| a.0.path.cmp(&b.0.path));

            for (trait_ref, methods) in sorted_trait_with_methods {
              output.push_str(&format!("#### {}\n\n", trait_ref.path));
              for (sig, links, doc) in methods {
                let links_json = format_links_as_json(&links);
                output.push_str(&format!(
                  "<RustCode inline code={{`{}`}} links={{{}}} />\n\n",
                  sig, links_json
                ));
                if let Some(doc) = doc {
                  output.push_str(&format!("{}\n\n", doc));
                }
                output.push_str("---\n\n");
              }
            }
          }
        }
      }
    }
    ItemEnum::Function(f) => {
      output.push_str("*Function*\n\n");

      if let Some(docs) = &item.docs {
        output.push_str(&format!("{}\n\n", sanitize_docs_for_mdx(docs)));
      }

      // Format function definition with links
      let (code, links) = format_function_definition_with_links(name, f, item, crate_data);
      let links_json = format_links_as_json(&links);
      output.push_str(&format!(
        "<RustCode code={{`{}`}} links={{{}}} />\n\n",
        code, links_json
      ));
    }
    ItemEnum::Trait(t) => {
      // Add code signature like rustdoc
      output.push_str("```rust\n");

      // Add visibility and trait keyword
      let visibility = match &item.visibility {
        rustdoc_types::Visibility::Public => "pub ",
        _ => "",
      };

      output.push_str(&format!("{}trait {}", visibility, name));

      // Show simplified trait signature
      output.push_str(" { /* ... */ }\n");
      output.push_str("```\n\n");

      if let Some(docs) = &item.docs {
        output.push_str(&format!("{}\n\n", sanitize_docs_for_mdx(docs)));
      }

      if !t.items.is_empty() {
        output.push_str("### Methods\n\n");
        for method_id in &t.items {
          if let Some(method) = crate_data.index.get(method_id) {
            if let Some(method_name) = &method.name {
              output.push_str(&format!("- `{}`", method_name));
              if let Some(method_docs) = &method.docs {
                output.push_str(&format!(": {}", method_docs.lines().next().unwrap_or("")));
              }
              output.push('\n');
            }
          }
        }
        output.push('\n');
      }
    }
    ItemEnum::Module(_) => {
      output.push_str(&format!("## Module: {}\n\n", name));

      if let Some(docs) = &item.docs {
        output.push_str(&format!("{}\n\n", sanitize_docs_for_mdx(docs)));
      }
    }
    ItemEnum::Constant { .. } => {
      output.push_str(&format!("## {}\n\n", name));
      output.push_str("*Constant*\n\n");

      if let Some(docs) = &item.docs {
        output.push_str(&format!("{}\n\n", sanitize_docs_for_mdx(docs)));
      }
    }
    ItemEnum::TypeAlias(ta) => {
      output.push_str(&format!("## {}\n\n", name));
      output.push_str(&format!(
        "*Type Alias*: `{}`\n\n",
        format_type(&ta.type_, crate_data)
      ));

      if let Some(docs) = &item.docs {
        output.push_str(&format!("{}\n\n", sanitize_docs_for_mdx(docs)));
      }
    }
    _ => {
      return None;
    }
  }

  Some(output)
}

fn format_generic_param(param: &rustdoc_types::GenericParamDef) -> String {
  match &param.kind {
    rustdoc_types::GenericParamDefKind::Lifetime { .. } => {
      // Lifetime names already include the ' prefix in rustdoc JSON
      param.name.clone()
    }
    rustdoc_types::GenericParamDefKind::Type { .. } => param.name.clone(),
    rustdoc_types::GenericParamDefKind::Const { .. } => {
      format!("const {}", param.name)
    }
  }
}

fn is_synthetic_lifetime(name: &str) -> bool {
  // Filter compiler-generated synthetic lifetimes
  name == "'_"
    || name.starts_with("'_") && name[2..].chars().all(|c| c.is_ascii_digit())
    || name.starts_with("'life") && name[5..].chars().all(|c| c.is_ascii_digit())
    || name == "'async_trait"
}

fn is_compiler_internal_trait(trait_name: &str) -> bool {
  matches!(
    trait_name,
    "StructuralPartialEq" | "StructuralEq" | "Freeze" | "Unpin" | "RefUnwindSafe" | "UnwindSafe"
  )
}

fn collect_impls_for_type<'a>(
  type_id: &rustdoc_types::Id,
  crate_data: &'a Crate,
) -> (Vec<&'a rustdoc_types::Impl>, Vec<&'a rustdoc_types::Impl>) {
  use rustdoc_types::Type;

  let mut inherent_impls = Vec::new();
  let mut trait_impls = Vec::new();

  for item in crate_data.index.values() {
    if let ItemEnum::Impl(impl_block) = &item.inner {
      let matches = match &impl_block.for_ {
        Type::ResolvedPath(path) => path.id == *type_id,
        _ => false,
      };

      if matches {
        if impl_block.trait_.is_some() {
          trait_impls.push(impl_block);
        } else {
          inherent_impls.push(impl_block);
        }
      }
    }
  }

  (inherent_impls, trait_impls)
}

#[allow(clippy::type_complexity)]
fn format_impl_methods(
  impl_block: &rustdoc_types::Impl,
  crate_data: &Crate,
  parent_item: Option<&Item>,
) -> Vec<(String, Vec<(String, String)>, Option<String>)> {
  let mut methods = Vec::new();

  for method_id in &impl_block.items {
    if let Some(method) = crate_data.index.get(method_id) {
      if let ItemEnum::Function(f) = &method.inner {
        if let Some(method_name) = &method.name {
          let (sig, links) =
            format_function_signature_with_links(method_name, f, crate_data, parent_item);
          let doc = method.docs.as_ref().and_then(|d| {
            let first_line = d.lines().next().unwrap_or("").trim();
            if !first_line.is_empty() {
              Some(first_line.to_string())
            } else {
              None
            }
          });
          methods.push((sig, links, doc));
        }
      }
    }
  }

  methods
}

#[allow(clippy::format_in_format_args)]
fn format_function_signature_with_links(
  name: &str,
  f: &rustdoc_types::Function,
  crate_data: &Crate,
  current_item: Option<&Item>,
) -> (String, Vec<(String, String)>) {
  let mut sig = format!("fn {}", name);
  let mut links = Vec::new();

  let non_synthetic_params: Vec<String> = f
    .generics
    .params
    .iter()
    .filter(|p| {
      !matches!(&p.kind, rustdoc_types::GenericParamDefKind::Lifetime { .. })
        || !is_synthetic_lifetime(&p.name)
    })
    .map(format_generic_param)
    .collect();

  if !non_synthetic_params.is_empty() {
    sig.push('<');
    sig.push_str(&non_synthetic_params.join(", "));
    sig.push('>');
  }

  sig.push('(');
  let mut inputs = Vec::new();
  for (param_name, ty) in &f.sig.inputs {
    let (type_str, type_links) = format_type_with_links(ty, crate_data, current_item);
    links.extend(type_links);
    inputs.push(format!("{}: {}", param_name, type_str));
  }

  // Format on multiple lines if signature is too long (> 80 chars) or has many parameters (> 3)
  let single_line = format!(
    "fn {}{}",
    if !non_synthetic_params.is_empty() {
      format!("{}<{}>", name, non_synthetic_params.join(", "))
    } else {
      name.to_string()
    },
    format!("({})", inputs.join(", "))
  );

  if inputs.len() > 3 || single_line.len() > 80 {
    // Multi-line format
    sig = format!("fn {}", name);
    if !non_synthetic_params.is_empty() {
      sig.push('<');
      sig.push_str(&non_synthetic_params.join(", "));
      sig.push('>');
    }
    sig.push_str("(\n");
    for (i, input) in inputs.iter().enumerate() {
      sig.push_str(&format!("    {}", input));
      if i < inputs.len() - 1 {
        sig.push(',');
      }
      sig.push('\n');
    }
    sig.push(')');
  } else {
    // Single line format
    sig.push_str(&inputs.join(", "));
    sig.push(')');
  }

  if let Some(output_type) = &f.sig.output {
    let (type_str, type_links) = format_type_with_links(output_type, crate_data, current_item);
    links.extend(type_links);
    sig.push_str(&format!(" -> {}", type_str));
  }

  (sig, links)
}

fn format_type(ty: &rustdoc_types::Type, crate_data: &Crate) -> String {
  format_type_depth(ty, crate_data, 0)
}

fn format_type_depth(ty: &rustdoc_types::Type, crate_data: &Crate, depth: usize) -> String {
  const MAX_DEPTH: usize = 50;

  if depth > MAX_DEPTH {
    return "...".to_string();
  }

  use rustdoc_types::Type;
  match ty {
    Type::ResolvedPath(path) => {
      let short_name = get_short_type_name(&path.path);
      let link = Some(path.id)
        .as_ref()
        .and_then(|id| generate_type_link(&path.path, id, crate_data, None));
      let mut result = if let Some(link) = link {
        format!("[{}]({})", short_name, link)
      } else {
        short_name
      };
      if let Some(args) = &path.args {
        result.push_str(&format_generic_args(args, crate_data));
      }
      result
    }
    Type::DynTrait(dt) => {
      if let Some(first) = dt.traits.first() {
        let short_name = get_short_type_name(&first.trait_.path);
        let link = generate_type_link(&first.trait_.path, &first.trait_.id, crate_data, None);
        if let Some(link) = link {
          format!("dyn [{}]({})", short_name, link)
        } else {
          format!("dyn {}", short_name)
        }
      } else {
        "dyn Trait".to_string()
      }
    }
    Type::Generic(name) => name.clone(),
    Type::Primitive(name) => name.clone(),
    Type::FunctionPointer(_) => "fn(...)".to_string(),
    Type::Tuple(types) => {
      let formatted: Vec<_> = types
        .iter()
        .map(|t| format_type_depth(t, crate_data, depth + 1))
        .collect();
      format!("({})", formatted.join(", "))
    }
    Type::Slice(inner) => format!("[{}]", format_type_depth(inner, crate_data, depth + 1)),
    Type::Array { type_, len } => format!(
      "[{}; {}]",
      format_type_depth(type_, crate_data, depth + 1),
      len
    ),
    Type::Pat { type_, .. } => format_type_depth(type_, crate_data, depth + 1),
    Type::ImplTrait(_bounds) => "impl Trait".to_string(),
    Type::Infer => "_".to_string(),
    Type::RawPointer { is_mutable, type_ } => {
      if *is_mutable {
        format!("*mut {}", format_type_depth(type_, crate_data, depth + 1))
      } else {
        format!("*const {}", format_type_depth(type_, crate_data, depth + 1))
      }
    }
    Type::BorrowedRef {
      lifetime,
      is_mutable,
      type_,
    } => {
      let lifetime_str = lifetime.as_deref().unwrap_or("");
      let space = if lifetime_str.is_empty() { "" } else { " " };
      if *is_mutable {
        format!(
          "&{}{} mut {}",
          lifetime_str,
          space,
          format_type_depth(type_, crate_data, depth + 1)
        )
      } else {
        format!(
          "&{}{}{}",
          lifetime_str,
          space,
          format_type_depth(type_, crate_data, depth + 1)
        )
      }
    }
    Type::QualifiedPath {
      name,
      self_type,
      trait_,
      ..
    } => {
      if let Some(trait_) = trait_ {
        let trait_short = get_short_type_name(&trait_.path);
        let trait_link = generate_type_link(&trait_.path, &trait_.id, crate_data, None);
        let trait_part = if let Some(link) = trait_link {
          format!("[{}]({})", trait_short, link)
        } else {
          trait_short
        };
        format!(
          "<{} as {}>::{}",
          format_type_depth(self_type, crate_data, depth + 1),
          trait_part,
          name
        )
      } else {
        format!(
          "{}::{}",
          format_type_depth(self_type, crate_data, depth + 1),
          name
        )
      }
    }
  }
}

/// Format a type without links (for use in code blocks)
fn format_type_plain(ty: &rustdoc_types::Type, crate_data: &Crate) -> String {
  use rustdoc_types::Type;
  match ty {
    Type::ResolvedPath(path) => {
      let short_name = get_short_type_name(&path.path);
      let mut result = short_name;
      if let Some(args) = &path.args {
        result.push_str(&format_generic_args_plain(args, crate_data));
      }
      result
    }
    Type::DynTrait(dt) => {
      if let Some(first) = dt.traits.first() {
        let short_name = get_short_type_name(&first.trait_.path);
        format!("dyn {}", short_name)
      } else {
        "dyn Trait".to_string()
      }
    }
    Type::Generic(name) => name.clone(),
    Type::Primitive(name) => name.clone(),
    Type::FunctionPointer(_) => "fn(...)".to_string(),
    Type::Tuple(types) => {
      let formatted: Vec<_> = types
        .iter()
        .map(|t| format_type_plain(t, crate_data))
        .collect();
      format!("({})", formatted.join(", "))
    }
    Type::Slice(inner) => format!("[{}]", format_type_plain(inner, crate_data)),
    Type::Array { type_, len } => format!("[{}; {}]", format_type_plain(type_, crate_data), len),
    Type::Pat { type_, .. } => format_type_plain(type_, crate_data),
    Type::ImplTrait(_bounds) => "impl Trait".to_string(),
    Type::Infer => "_".to_string(),
    Type::RawPointer { is_mutable, type_ } => {
      if *is_mutable {
        format!("*mut {}", format_type_plain(type_, crate_data))
      } else {
        format!("*const {}", format_type_plain(type_, crate_data))
      }
    }
    Type::BorrowedRef {
      lifetime,
      is_mutable,
      type_,
    } => {
      let lifetime_str = lifetime.as_deref().unwrap_or("");
      let space = if lifetime_str.is_empty() { "" } else { " " };
      if *is_mutable {
        format!(
          "&{}{} mut {}",
          lifetime_str,
          space,
          format_type_plain(type_, crate_data)
        )
      } else {
        format!(
          "&{}{}{}",
          lifetime_str,
          space,
          format_type_plain(type_, crate_data)
        )
      }
    }
    Type::QualifiedPath {
      name,
      self_type,
      trait_,
      ..
    } => {
      if let Some(trait_) = trait_ {
        let trait_short = get_short_type_name(&trait_.path);
        format!(
          "<{} as {}>::{}",
          format_type_plain(self_type, crate_data),
          trait_short,
          name
        )
      } else {
        format!("{}::{}", format_type_plain(self_type, crate_data), name)
      }
    }
  }
}

fn format_generic_args_plain(args: &rustdoc_types::GenericArgs, crate_data: &Crate) -> String {
  use rustdoc_types::{GenericArg, GenericArgs};
  match args {
    GenericArgs::AngleBracketed { args, .. } => {
      if args.is_empty() {
        String::new()
      } else {
        let formatted: Vec<String> = args
          .iter()
          .filter_map(|arg| match arg {
            GenericArg::Lifetime(lt) if lt != "'_" => Some(lt.clone()),
            GenericArg::Lifetime(_) => None,
            GenericArg::Type(ty) => Some(format_type_plain(ty, crate_data)),
            GenericArg::Const(c) => Some(c.expr.clone()),
            GenericArg::Infer => Some("_".to_string()),
          })
          .collect();
        if formatted.is_empty() {
          String::new()
        } else {
          format!("<{}>", formatted.join(", "))
        }
      }
    }
    GenericArgs::Parenthesized { inputs, output } => {
      let inputs_str: Vec<_> = inputs
        .iter()
        .map(|t| format_type_plain(t, crate_data))
        .collect();
      let mut result = format!("({})", inputs_str.join(", "));
      if let Some(output) = output {
        result.push_str(&format!(" -> {}", format_type_plain(output, crate_data)));
      }
      result
    }
    GenericArgs::ReturnTypeNotation => "(..)".to_string(),
  }
}

fn get_short_type_name(full_path: &str) -> String {
  full_path
    .split("::")
    .last()
    .unwrap_or(full_path)
    .to_string()
}

fn format_links_as_json(links: &[(String, String)]) -> String {
  if links.is_empty() {
    return "[]".to_string();
  }

  let items: Vec<String> = links
    .iter()
    .map(|(text, href)| {
      // Escape quotes in text and href
      let text_escaped = text.replace('\\', "\\\\").replace('"', "\\\"");
      let href_escaped = href.replace('\\', "\\\\").replace('"', "\\\"");
      // Use quoted keys for MDX/JSX compatibility
      format!(
        r#"{{"text": "{}", "href": "{}"}}"#,
        text_escaped, href_escaped
      )
    })
    .collect();

  format!("[{}]", items.join(", "))
}

/// Sanitize documentation comments for MDX compatibility
///
/// MDX is stricter than regular markdown about HTML tags. This function ensures
/// that HTML blocks (like <details>) are properly separated from text paragraphs
/// with blank lines.
fn sanitize_docs_for_mdx(docs: &str) -> String {
  let lines: Vec<&str> = docs.lines().collect();
  let mut result: Vec<String> = Vec::new();
  let mut i = 0;

  while i < lines.len() {
    let current_line = lines[i];
    let trimmed = current_line.trim();

    // Check if this line starts with an HTML opening tag
    if trimmed.starts_with('<') && !trimmed.starts_with("</") {
      // Extract tag name (e.g., "details" from "<details>")
      if let Some(tag_end) = trimmed.find(|c: char| ['>', ' '].contains(&c)) {
        let tag_name = &trimmed[1..tag_end];

        // Only process block-level HTML tags
        if matches!(
          tag_name,
          "details" | "summary" | "div" | "table" | "pre" | "blockquote"
        ) {
          // Ensure blank line before the HTML block
          if !result.is_empty() && !result.last().unwrap().is_empty() {
            result.push(String::new());
          }

          // Split multiple HTML tags on the same line (e.g., "<details><summary>")
          // MDX requires each tag to be on its own line
          let mut current_line_content = trimmed.to_string();
          while !current_line_content.is_empty() {
            if let Some(tag_start) = current_line_content.find('<') {
              // Add any content before the tag
              if tag_start > 0 {
                result.push(current_line_content[..tag_start].to_string());
              }

              // Find the end of this tag
              if let Some(tag_end) = current_line_content[tag_start..].find('>') {
                let tag_end_abs = tag_start + tag_end + 1;
                result.push(current_line_content[tag_start..tag_end_abs].to_string());
                current_line_content = current_line_content[tag_end_abs..].to_string();
              } else {
                // Malformed tag, just add the rest
                result.push(current_line_content.clone());
                break;
              }
            } else {
              // No more tags, add remaining content if any
              if !current_line_content.trim().is_empty() {
                result.push(current_line_content.clone());
              }
              break;
            }
          }

          // Continue adding lines until we find the closing tag
          i += 1;
          while i < lines.len() {
            let next_line = lines[i];
            let next_trimmed = next_line.trim();

            // Check if we found the closing tag
            if next_trimmed.contains(&format!("</{}>", tag_name)) {
              // Split this line too in case it has multiple tags
              let mut current_line_content = next_trimmed.to_string();
              while !current_line_content.is_empty() {
                if let Some(tag_start) = current_line_content.find('<') {
                  if tag_start > 0 {
                    result.push(current_line_content[..tag_start].to_string());
                  }
                  if let Some(tag_end) = current_line_content[tag_start..].find('>') {
                    let tag_end_abs = tag_start + tag_end + 1;
                    result.push(current_line_content[tag_start..tag_end_abs].to_string());
                    current_line_content = current_line_content[tag_end_abs..].to_string();
                  } else {
                    result.push(current_line_content.clone());
                    break;
                  }
                } else {
                  if !current_line_content.trim().is_empty() {
                    result.push(current_line_content.clone());
                  }
                  break;
                }
              }
              i += 1;
              break;
            } else {
              // Trim HTML lines to avoid indentation issues with MDX
              result.push(next_trimmed.to_string());
            }
            i += 1;
          }

          // Ensure blank line after the HTML block
          if i < lines.len() && !lines[i].trim().is_empty() {
            result.push(String::new());
          }
          continue;
        }
      }
    }

    // Preserve original line (don't trim it)
    result.push(current_line.to_string());
    i += 1;
  }

  result.join("\n")
}

fn generate_type_link(
  full_path: &str,
  item_id: &Id,
  crate_data: &Crate,
  current_item: Option<&Item>,
) -> Option<String> {
  generate_type_link_depth(full_path, item_id, crate_data, current_item, 0)
}

#[allow(clippy::bind_instead_of_map)]
fn generate_type_link_depth(
  full_path: &str,
  item_id: &Id,
  crate_data: &Crate,
  current_item: Option<&Item>,
  depth: usize,
) -> Option<String> {
  const MAX_DEPTH: usize = 10;

  if depth >= MAX_DEPTH {
    return None;
  }

  // Always prefer the path from crate_data.paths when available,
  // as it contains the most accurate full path information
  // BUT: if depth > 0, we're in a recursive call and should trust the provided full_path
  let full_path = if depth == 0 {
    if let Some(path_info) = crate_data.paths.get(item_id) {
      path_info.path.join("::")
    } else if full_path.starts_with("$crate") {
      // Fallback for $crate placeholder
      full_path.replace("$crate", "unknown")
    } else {
      full_path.to_string()
    }
  } else {
    full_path.to_string()
  };
  let full_path = full_path.as_str();

  // Check if this is a local or external type by looking at crate_id in paths
  let is_local = if let Some(path_info) = crate_data.paths.get(item_id) {
    path_info.crate_id == 0
  } else {
    // If not in paths, check if it's in the current crate's index
    crate_data.index.contains_key(item_id)
  };

  // Handle local types (from the current crate)
  if is_local {
    if let Some(item) = crate_data.index.get(item_id) {
      // Local type - we need to build the full path from the item's location
      // Get the crate name
      let _crate_name = crate_data.index.get(&crate_data.root)?.name.as_ref()?;

      // Get the item prefix (struct., enum., trait., etc.)
      let prefix = get_item_prefix(item);

      // Extract module path for the target item
      // First try from crate_data.paths as it's more reliable
      let target_module_path = if let Some(path_info) = crate_data.paths.get(item_id) {
        // Get the full path from paths (e.g., ["test_crate", "patterns", "Builder"])
        let path_components: Vec<&str> = path_info.path.iter().map(|s| s.as_str()).collect();
        if path_components.len() > 2 {
          // Skip crate name and item name, join the middle parts
          // e.g., ["test_crate", "patterns", "Builder"] -> "patterns"
          Some(path_components[1..path_components.len() - 1].join("/"))
        } else {
          // Root module (only crate name and item name)
          Some("".to_string())
        }
      } else if let Some(span) = &item.span {
        // Fallback to span if paths is not available
        let span_filename = &span.filename;
        if let Some(filename_str) = span_filename.to_str() {
          if let Some(src_idx) = filename_str.rfind("/src/") {
            let after_src = &filename_str[src_idx + 5..];
            if let Some(rs_idx) = after_src.rfind(".rs") {
              let module_path = &after_src[..rs_idx];
              if module_path == "lib" || module_path == "main" {
                Some("".to_string())
              } else {
                Some(module_path.to_string())
              }
            } else {
              None
            }
          } else {
            None
          }
        } else {
          None
        }
      } else {
        None
      };

      // Extract module path for the current item (if provided)
      let _current_module_path = if let Some(current) = current_item {
        // First try to get it from span
        if let Some(span) = &current.span {
          let span_filename = &span.filename;
          if let Some(filename_str) = span_filename.to_str() {
            if let Some(src_idx) = filename_str.rfind("/src/") {
              let after_src = &filename_str[src_idx + 5..];
              if let Some(rs_idx) = after_src.rfind(".rs") {
                let module_path = &after_src[..rs_idx];
                if module_path == "lib" || module_path == "main" {
                  Some("".to_string())
                } else {
                  Some(module_path.to_string())
                }
              } else {
                None
              }
            } else {
              None
            }
          } else {
            None
          }
        } else {
          // If no span (e.g., re-export), try to infer from the item's path in paths
          // Get the item's id and look it up in paths
          crate_data.paths.get(&current.id).map(|path_info| {
            let full_path: Vec<&str> = path_info.path.iter().map(|s| s.as_str()).collect();
            if full_path.len() > 2 {
              // Skip crate name and item name, join the middle parts
              full_path[1..full_path.len() - 1].join("/")
            } else {
              // Root module
              String::new()
            }
          })
        }
      } else {
        None
      };

      // The full_path might be just the type name or include module path
      let path_segments: Vec<&str> = full_path.split("::").collect();
      let type_name = path_segments.last().unwrap_or(&full_path);

      // Generate absolute link from crate root
      // This works for both original files and re-exports without any path calculations
      if let Some(target_path) = target_module_path {
        let crate_name = path_segments.first().unwrap_or(&"");

        let base = BASE_PATH.with(|bp| bp.borrow().clone());
        let base_prefix = if base.is_empty() { String::new() } else { base };

        if target_path.is_empty() {
          // Target is in root module: /base_path/crate_name/struct.TypeName
          return Some(format!(
            "{}/{}/{}{}",
            base_prefix, crate_name, prefix, type_name
          ));
        } else {
          // Target is in a nested module: /base_path/crate_name/module/path/struct.TypeName
          return Some(format!(
            "{}/{}/{}/{}{}",
            base_prefix, crate_name, target_path, prefix, type_name
          ));
        }
      } else {
        // Fallback: use crate root path
        let crate_name = path_segments.first().unwrap_or(&"");
        let base = BASE_PATH.with(|bp| bp.borrow().clone());
        let base_prefix = if base.is_empty() { String::new() } else { base };
        return Some(format!(
          "{}/{}/{}{}",
          base_prefix, crate_name, prefix, type_name
        ));
      }
    } // end if let Some(item)
  } // end if is_local

  // External type - check if it's std/core/alloc first, then try docs.rs
  let path_parts: Vec<&str> = full_path.split("::").collect();

  if path_parts.len() >= 2 {
    let crate_name = path_parts[0];

    // Check if it's a standard library crate (std, core, alloc)
    if crate_name == "std" || crate_name == "core" || crate_name == "alloc" {
      // Build doc.rust-lang.org URL
      // e.g., std::sync::Arc -> https://doc.rust-lang.org/std/sync/struct.Arc.html
      // e.g., core::fmt::Formatter -> https://doc.rust-lang.org/core/fmt/struct.Formatter.html

      let type_name = path_parts.last()?;

      // Special handling for type aliases that should redirect to their canonical type
      // Only redirect specific known aliases, not all instances of these types
      if full_path == "core::fmt::Result" || full_path == "std::fmt::Result" {
        // core::fmt::Result is an alias for Result<(), fmt::Error>
        // Better to link to the generic Result documentation
        return Some("https://doc.rust-lang.org/std/result/enum.Result.html".to_string());
      }

      // For core types, prefer linking to std documentation when available
      // (std is re-exported and more familiar to users)
      if crate_name == "core" {
        match full_path {
          "core::result::Result" => {
            return Some("https://doc.rust-lang.org/std/result/enum.Result.html".to_string());
          }
          "core::option::Option" => {
            return Some("https://doc.rust-lang.org/std/option/enum.Option.html".to_string());
          }
          _ => {}
        }
      }

      // Filter out internal implementation modules
      let mut module_parts: Vec<&str> = path_parts[1..path_parts.len() - 1].to_vec();
      let internal_modules = ["bounded", "unbounded", "inner", "private", "imp"];
      module_parts.retain(|part| !internal_modules.contains(part));
      let module_path = module_parts.join("/");

      // Try to guess the item type from common patterns
      let item_type =
        if type_name.ends_with("Error") || *type_name == "Option" || *type_name == "Result" {
          "enum"
        } else {
          "struct" // Default to struct for most std types
        };

      return Some(format!(
        "https://doc.rust-lang.org/{}/{}/{}.{}.html",
        crate_name, module_path, item_type, type_name
      ));
    }

    // External crate - use crate_id to get the REAL crate name
    // (not the first path segment which might be a module name)
    let real_crate_name = if let Some(path_info) = crate_data.paths.get(item_id) {
      if path_info.crate_id != 0 {
        // It's from an external crate - look up the real name
        crate_data
          .external_crates
          .get(&path_info.crate_id)
          .map(|c| c.name.as_str())
          .unwrap_or(crate_name)
      } else {
        // It's from the current crate
        crate_name
      }
    } else {
      crate_name
    };

    // Check if this external crate is part of the workspace
    // If so, generate an internal link instead of docs.rs
    // Note: Normalize both names by replacing hyphens with underscores
    // because crate names in Cargo.toml use hyphens but rustdoc uses underscores
    let normalized_crate_name = real_crate_name.replace('-', "_");
    let is_workspace_crate = WORKSPACE_CRATES.with(|wc| {
      wc.borrow().iter().any(|c| {
        let normalized_c = c.replace('-', "_");
        normalized_c == normalized_crate_name
      })
    });

    if is_workspace_crate {
      // Generate internal link for workspace crate
      // Get the item prefix
      let prefix = crate_data
        .paths
        .get(item_id)
        .map(|p| match p.kind {
          rustdoc_types::ItemKind::Struct => "struct.",
          rustdoc_types::ItemKind::Enum => "enum.",
          rustdoc_types::ItemKind::Trait => "trait.",
          rustdoc_types::ItemKind::Function => "fn.",
          rustdoc_types::ItemKind::TypeAlias => "type.",
          rustdoc_types::ItemKind::Constant => "constant.",
          _ => "struct.",
        })
        .unwrap_or("struct.");

      let type_name = path_parts.last()?;

      // Get module path
      let mut module_parts: Vec<&str> = path_parts[1..path_parts.len() - 1].to_vec();
      let internal_modules = ["bounded", "unbounded", "inner", "private", "imp"];
      module_parts.retain(|part| !internal_modules.contains(part));
      let module_path = module_parts.join("/");

      let base = BASE_PATH.with(|bp| bp.borrow().clone());
      let base_prefix = if base.is_empty() { String::new() } else { base };

      if module_path.is_empty() {
        // Top-level type: /base_path/crate_name/struct.TypeName
        return Some(format!(
          "{}/{}/{}{}",
          base_prefix, real_crate_name, prefix, type_name
        ));
      } else {
        // Nested module: /base_path/crate_name/module/path/struct.TypeName
        return Some(format!(
          "{}/{}/{}/{}{}",
          base_prefix, real_crate_name, module_path, prefix, type_name
        ));
      }
    }

    // Not a workspace crate - generate docs.rs link
    // Try to get the item kind from paths to generate correct URL
    let item_kind = crate_data
      .paths
      .get(item_id)
      .map(|p| &p.kind)
      .and_then(|k| match k {
        rustdoc_types::ItemKind::Struct => Some("struct"),
        rustdoc_types::ItemKind::Enum => Some("enum"),
        rustdoc_types::ItemKind::Trait => Some("trait"),
        rustdoc_types::ItemKind::Function => Some("fn"),
        rustdoc_types::ItemKind::TypeAlias => Some("type"),
        rustdoc_types::ItemKind::Constant => Some("constant"),
        _ => Some("struct"), // Default
      })
      .unwrap_or("struct");

    let type_name = path_parts.last()?;

    // Get module path, filtering out common internal implementation modules
    let mut module_parts: Vec<&str> = path_parts[1..path_parts.len() - 1].to_vec();

    // Remove common internal module names that are typically not in public re-exports
    // These are often implementation details that docs.rs doesn't expose in URLs
    let internal_modules = ["bounded", "unbounded", "inner", "private", "imp"];
    module_parts.retain(|part| !internal_modules.contains(part));

    let module_path = module_parts.join("/");

    // Format: https://docs.rs/crate_name/latest/crate_name/module/path/struct.TypeName.html
    if module_path.is_empty() {
      // Top-level type in crate
      return Some(format!(
        "https://docs.rs/{}/latest/{}/{}.{}.html",
        real_crate_name, real_crate_name, item_kind, type_name
      ));
    } else {
      return Some(format!(
        "https://docs.rs/{}/latest/{}/{}/{}.{}.html",
        real_crate_name, real_crate_name, module_path, item_kind, type_name
      ));
    }
  }

  // Single-segment path - try to find in paths first (could be from any crate)
  if path_parts.len() == 1 {
    let type_name = path_parts[0];

    // Check if we have this type in paths (could be external or std)
    if let Some(path_info) = crate_data.paths.get(item_id) {
      let full_path_from_paths = path_info.path.join("::");
      // Only recurse if the resolved path is different from the input path
      if full_path_from_paths != full_path {
        // Recursively call with the full path
        return generate_type_link_depth(
          &full_path_from_paths,
          item_id,
          crate_data,
          current_item,
          depth + 1,
        );
      }
    }

    // Fallback: common std library types (for backward compatibility)
    match type_name {
      "String" => {
        return Some("https://doc.rust-lang.org/std/string/struct.String.html".to_string());
      }
      "Vec" => return Some("https://doc.rust-lang.org/std/vec/struct.Vec.html".to_string()),
      "Option" => return Some("https://doc.rust-lang.org/std/option/enum.Option.html".to_string()),
      "Result" => return Some("https://doc.rust-lang.org/std/result/enum.Result.html".to_string()),
      "Box" => return Some("https://doc.rust-lang.org/std/boxed/struct.Box.html".to_string()),
      "Rc" => return Some("https://doc.rust-lang.org/std/rc/struct.Rc.html".to_string()),
      "Arc" => return Some("https://doc.rust-lang.org/std/sync/struct.Arc.html".to_string()),
      "HashMap" => {
        return Some("https://doc.rust-lang.org/std/collections/struct.HashMap.html".to_string());
      }
      "HashSet" => {
        return Some("https://doc.rust-lang.org/std/collections/struct.HashSet.html".to_string());
      }
      "BTreeMap" => {
        return Some("https://doc.rust-lang.org/std/collections/struct.BTreeMap.html".to_string());
      }
      "BTreeSet" => {
        return Some("https://doc.rust-lang.org/std/collections/struct.BTreeSet.html".to_string());
      }
      "Mutex" => return Some("https://doc.rust-lang.org/std/sync/struct.Mutex.html".to_string()),
      "RwLock" => return Some("https://doc.rust-lang.org/std/sync/struct.RwLock.html".to_string()),
      "Cell" => return Some("https://doc.rust-lang.org/std/cell/struct.Cell.html".to_string()),
      "RefCell" => {
        return Some("https://doc.rust-lang.org/std/cell/struct.RefCell.html".to_string());
      }
      "Path" => return Some("https://doc.rust-lang.org/std/path/struct.Path.html".to_string()),
      "PathBuf" => {
        return Some("https://doc.rust-lang.org/std/path/struct.PathBuf.html".to_string());
      }
      _ => {}
    }
  }

  None
}

fn format_type_with_links(
  ty: &rustdoc_types::Type,
  crate_data: &Crate,
  current_item: Option<&Item>,
) -> (String, Vec<(String, String)>) {
  format_type_with_links_depth(ty, crate_data, current_item, 0)
}

fn format_type_with_links_depth(
  ty: &rustdoc_types::Type,
  crate_data: &Crate,
  current_item: Option<&Item>,
  depth: usize,
) -> (String, Vec<(String, String)>) {
  const MAX_DEPTH: usize = 50;

  if depth > MAX_DEPTH {
    return ("...".to_string(), Vec::new());
  }

  use rustdoc_types::Type;
  let mut links = Vec::new();

  let type_str = match ty {
    Type::ResolvedPath(path) => {
      let short_name = get_short_type_name(&path.path);
      if let Some(link) = Some(path.id)
        .as_ref()
        .and_then(|id| generate_type_link(&path.path, id, crate_data, current_item))
      {
        links.push((short_name.clone(), link));
      }
      let mut result = short_name;
      if let Some(args) = &path.args {
        let (args_str, args_links) = format_generic_args_with_links(args, crate_data, current_item);
        links.extend(args_links);
        result.push_str(&args_str);
      }
      result
    }
    Type::DynTrait(dt) => {
      if let Some(first) = dt.traits.first() {
        let short_name = get_short_type_name(&first.trait_.path);
        if let Some(link) = generate_type_link(
          &first.trait_.path,
          &first.trait_.id,
          crate_data,
          current_item,
        ) {
          links.push((short_name.clone(), link));
        }
        format!("dyn {}", short_name)
      } else {
        "dyn Trait".to_string()
      }
    }
    Type::Generic(name) => name.clone(),
    Type::Primitive(name) => name.clone(),
    Type::FunctionPointer(_) => "fn(...)".to_string(),
    Type::Tuple(types) => {
      let mut parts = Vec::new();
      for t in types {
        let (type_str, type_links) =
          format_type_with_links_depth(t, crate_data, current_item, depth + 1);
        links.extend(type_links);
        parts.push(type_str);
      }
      format!("({})", parts.join(", "))
    }
    Type::Slice(inner) => {
      let (inner_str, inner_links) =
        format_type_with_links_depth(inner, crate_data, current_item, depth + 1);
      links.extend(inner_links);
      format!("[{}]", inner_str)
    }
    Type::Array { type_, len } => {
      let (type_str, type_links) =
        format_type_with_links_depth(type_, crate_data, current_item, depth + 1);
      links.extend(type_links);
      format!("[{}; {}]", type_str, len)
    }
    Type::Pat { type_, .. } => {
      let (type_str, type_links) =
        format_type_with_links_depth(type_, crate_data, current_item, depth + 1);
      links.extend(type_links);
      type_str
    }
    Type::ImplTrait(bounds) => {
      // Extract links from trait bounds in impl Trait
      for bound in bounds {
        if let rustdoc_types::GenericBound::TraitBound { trait_, .. } = bound {
          let short_name = get_short_type_name(&trait_.path);
          if let Some(link) = generate_type_link(&trait_.path, &trait_.id, crate_data, current_item)
          {
            links.push((short_name, link));
          }
          // Also extract links from generic arguments (e.g., Into<T>)
          if let Some(args) = &trait_.args {
            let (_, args_links) = format_generic_args_with_links(args, crate_data, current_item);
            links.extend(args_links);
          }
        }
      }
      "impl Trait".to_string()
    }
    Type::Infer => "_".to_string(),
    Type::RawPointer { is_mutable, type_ } => {
      let (type_str, type_links) =
        format_type_with_links_depth(type_, crate_data, current_item, depth + 1);
      links.extend(type_links);
      if *is_mutable {
        format!("*mut {}", type_str)
      } else {
        format!("*const {}", type_str)
      }
    }
    Type::BorrowedRef {
      lifetime,
      is_mutable,
      type_,
    } => {
      let (type_str, type_links) =
        format_type_with_links_depth(type_, crate_data, current_item, depth + 1);
      links.extend(type_links);
      let lifetime_str = lifetime.as_deref().unwrap_or("");
      let space = if lifetime_str.is_empty() { "" } else { " " };
      if *is_mutable {
        format!("&{}{} mut {}", lifetime_str, space, type_str)
      } else {
        format!("&{}{}{}", lifetime_str, space, type_str)
      }
    }
    Type::QualifiedPath {
      name,
      self_type,
      trait_,
      ..
    } => {
      let (self_str, self_links) =
        format_type_with_links_depth(self_type, crate_data, current_item, depth + 1);
      links.extend(self_links);
      if let Some(trait_) = trait_ {
        let trait_short = get_short_type_name(&trait_.path);
        if let Some(link) = generate_type_link(&trait_.path, &trait_.id, crate_data, current_item) {
          links.push((trait_short.clone(), link));
        }
        format!("<{} as {}>::{}", self_str, trait_short, name)
      } else {
        format!("{}::{}", self_str, name)
      }
    }
  };

  (type_str, links)
}

fn format_generic_args_with_links(
  args: &rustdoc_types::GenericArgs,
  crate_data: &Crate,
  current_item: Option<&Item>,
) -> (String, Vec<(String, String)>) {
  use rustdoc_types::{GenericArg, GenericArgs};
  let mut links = Vec::new();

  let args_str = match args {
    GenericArgs::AngleBracketed { args, .. } => {
      if args.is_empty() {
        String::new()
      } else {
        let mut formatted = Vec::new();
        for arg in args {
          match arg {
            GenericArg::Type(ty) => {
              let (type_str, type_links) = format_type_with_links(ty, crate_data, current_item);
              links.extend(type_links);
              formatted.push(type_str);
            }
            GenericArg::Lifetime(lt) => {
              if !is_synthetic_lifetime(lt) {
                formatted.push(lt.clone());
              }
            }
            _ => {}
          }
        }
        if formatted.is_empty() {
          String::new()
        } else {
          format!("<{}>", formatted.join(", "))
        }
      }
    }
    GenericArgs::Parenthesized { inputs, output } => {
      let mut inputs_parts = Vec::new();
      for input in inputs {
        let (type_str, type_links) = format_type_with_links(input, crate_data, current_item);
        links.extend(type_links);
        inputs_parts.push(type_str);
      }
      if let Some(out) = output {
        let (out_str, out_links) = format_type_with_links(out, crate_data, current_item);
        links.extend(out_links);
        format!("({}) -> {}", inputs_parts.join(", "), out_str)
      } else {
        format!("({})", inputs_parts.join(", "))
      }
    }
    GenericArgs::ReturnTypeNotation => String::new(),
  };

  (args_str, links)
}

fn format_generic_args(args: &rustdoc_types::GenericArgs, crate_data: &Crate) -> String {
  use rustdoc_types::{GenericArg, GenericArgs};
  match args {
    GenericArgs::AngleBracketed { args, .. } => {
      if args.is_empty() {
        String::new()
      } else {
        let formatted: Vec<String> = args
          .iter()
          .filter_map(|arg| match arg {
            GenericArg::Lifetime(lt) if lt != "'_" => Some(lt.clone()),
            GenericArg::Lifetime(_) => None,
            GenericArg::Type(ty) => Some(format_type(ty, crate_data)),
            GenericArg::Const(c) => Some(c.expr.clone()),
            GenericArg::Infer => Some("_".to_string()),
          })
          .collect();
        if formatted.is_empty() {
          String::new()
        } else {
          format!("<{}>", formatted.join(", "))
        }
      }
    }
    GenericArgs::Parenthesized { inputs, output } => {
      let inputs_str: Vec<_> = inputs.iter().map(|t| format_type(t, crate_data)).collect();
      let mut result = format!("({})", inputs_str.join(", "));
      if let Some(output) = output {
        result.push_str(&format!(" -> {}", format_type(output, crate_data)));
      }
      result
    }
    GenericArgs::ReturnTypeNotation => "(..)".to_string(),
  }
}

fn generate_crate_index(
  crate_name: &str,
  root_item: &Item,
  modules: &HashMap<String, Vec<(Id, Item)>>,
) -> String {
  let mut output = String::new();

  // Import RustCode component for inline code rendering
  output.push_str("import RustCode from '@site/src/components/RustCode';\n");
  output.push_str("import Link from '@docusaurus/Link';\n\n");

  output.push_str(&format!("# {}\n\n", crate_name));

  if let Some(docs) = &root_item.docs {
    output.push_str(&format!("{}\n\n", docs));
  }

  // Module listing with summary
  output.push_str("## Modules\n\n");

  let mut module_names: Vec<_> = modules.keys().collect();
  module_names.sort();

  for module_name in module_names {
    let items = &modules[module_name];

    let display_name = module_name
      .strip_prefix(&format!("{}::", crate_name))
      .unwrap_or(module_name);

    // For Docusaurus structure: submodules get index.md in their directory
    let module_file = if display_name.contains("::") {
      format!("{}/", display_name.replace("::", "/"))
    } else {
      format!("{}.md", display_name)
    };

    // Count item types
    let mut counts = HashMap::new();
    for (_id, item) in items {
      let type_name = match &item.inner {
        ItemEnum::Struct(_) => "structs",
        ItemEnum::Enum(_) => "enums",
        ItemEnum::Function(_) => "functions",
        ItemEnum::Trait(_) => "traits",
        ItemEnum::Constant { .. } => "constants",
        ItemEnum::TypeAlias(_) => "type aliases",
        ItemEnum::Module(_) => "modules",
        _ => continue,
      };
      *counts.entry(type_name).or_insert(0) += 1;
    }

    output.push_str(&format!("### [`{}`]({})\n\n", display_name, module_file));

    if !counts.is_empty() {
      let mut summary: Vec<String> = counts
        .iter()
        .map(|(name, count)| format!("{} {}", count, name))
        .collect();
      summary.sort();
      output.push_str(&format!("*{}*\n\n", summary.join(", ")));
    }
  }

  output
}

fn generate_combined_crate_and_root_content(
  crate_name: &str,
  root_item: &Item,
  _crate_data: &Crate,
  _modules: &HashMap<String, Vec<(Id, Item)>>,
  root_items: &[(Id, Item)],
  module_hierarchy: &HashMap<String, Vec<String>>,
  reexported_modules: &HashMap<String, Vec<(String, String)>>,
) -> String {
  let mut output = String::new();

  // Calculate sidebar key for the crate
  let base_path = BASE_PATH.with(|bp| bp.borrow().clone());
  let base_path_for_sidebar = base_path
    .strip_prefix("/docs/")
    .or_else(|| base_path.strip_prefix("/docs"))
    .or_else(|| base_path.strip_prefix("/"))
    .unwrap_or(&base_path);
  let sidebar_key = format!("{}/{}", base_path_for_sidebar, crate_name).replace("/", "_");

  // Add frontmatter with displayed_sidebar
  output.push_str("---\n");
  output.push_str(&format!("title: {}\n", crate_name));
  output.push_str(&format!("displayed_sidebar: '{}'\n", sidebar_key));
  output.push_str("---\n\n");

  // Import RustCode component for inline code rendering
  output.push_str("import RustCode from '@site/src/components/RustCode';\n");
  output.push_str("import Link from '@docusaurus/Link';\n\n");

  output.push_str(&format!("# Crate {}\n\n", crate_name));

  if let Some(docs) = &root_item.docs {
    output.push_str(&format!("{}\n\n", docs));
  }

  // If we have root-level items, show them first
  if !root_items.is_empty() {
    // Separate re-exports (Use items) from regular items
    let mut re_exports = Vec::new();
    let mut regular_items = Vec::new();

    for (id, item) in root_items {
      if matches!(&item.inner, ItemEnum::Use(_)) {
        re_exports.push((id, item));
      } else {
        regular_items.push((id, item));
      }
    }

    // Show Re-exports section first (if any)
    // Only show re-exports where the source module/item is public (rustdoc behavior)
    if !re_exports.is_empty() {
      let mut public_re_exports = Vec::new();

      for (id, item) in &re_exports {
        if let ItemEnum::Use(use_item) = &item.inner {
          // Check if the source is public
          let is_source_public = if let Some(import_id) = &use_item.id {
            if let Some(imported_item) = _crate_data.index.get(import_id) {
              is_public(imported_item)
            } else {
              // Not found in crate index - external dependency, assume public
              true
            }
          } else {
            // No import ID - assume public
            true
          };

          if is_source_public {
            public_re_exports.push((id, item, use_item));
          }
        }
      }

      if !public_re_exports.is_empty() {
        output.push_str("## Re-exports\n\n");

        for (_id, _item, use_item) in &public_re_exports {
          // Use the full source path (e.g., "patterns::Builder")
          let source_path = &use_item.source;

          // Build code string for RustCode component
          let code_str = if use_item.is_glob {
            format!("pub use {}::*;", source_path)
          } else {
            format!("pub use {};", source_path)
          };

          // Extract the final component of the path for linking
          // e.g., "generated::MessageRole" -> "MessageRole"
          let type_name = source_path.split("::").last().unwrap_or(source_path);

          // Try to find link to the re-exported item using absolute links
          let links: Vec<(String, String)> = if let Some(import_id) = &use_item.id {
            if let Some(link) = generate_type_link(source_path, import_id, _crate_data, None) {
              vec![(type_name.to_string(), link)]
            } else {
              // External dependency - no link
              vec![]
            }
          } else {
            vec![]
          };

          let links_json = format_links_as_json(&links);

          // Use RustCode inline component for consistent formatting
          output.push_str(&format!(
            "<RustCode inline code={{`{}`}} links={{{}}} />\n\n",
            code_str, links_json
          ));
        }
      }
    }

    let mut by_type: HashMap<&str, Vec<&Item>> = HashMap::new();
    for (_id, item) in &regular_items {
      let type_name = match &item.inner {
        ItemEnum::Struct(_) => "Structs",
        ItemEnum::Enum(_) => "Enums",
        ItemEnum::Function(_) => "Functions",
        ItemEnum::Trait(_) => "Traits",
        ItemEnum::Constant { .. } => "Constants",
        ItemEnum::TypeAlias(_) => "Type Aliases",
        ItemEnum::Module(_) => continue, // Skip module items, use hierarchy instead
        ItemEnum::Use(_) => continue,    // Use items are handled separately in Re-exports section
        _ => continue,
      };
      by_type.entry(type_name).or_default().push(item);
    }

    let type_order = [
      "Modules",
      "Structs",
      "Enums",
      "Functions",
      "Traits",
      "Constants",
      "Type Aliases",
    ];
    for type_name in &type_order {
      // Special handling for Modules - use hierarchy to show top-level modules
      if *type_name == "Modules" {
        let mut all_modules: Vec<(String, String)> = Vec::new();

        // Add modules from hierarchy (direct submodules)
        if let Some(top_level_modules) = module_hierarchy.get(crate_name) {
          for module_path in top_level_modules {
            let module_name = module_path.split("::").last().unwrap_or(module_path);
            all_modules.push((module_name.to_string(), module_path.clone()));
          }
        }

        // Add re-exported modules
        if let Some(reexported) = reexported_modules.get(crate_name) {
          for (module_name, module_path) in reexported {
            // Extract just the module name (last component)
            let short_name = module_path.split("::").last().unwrap_or(module_name);
            all_modules.push((short_name.to_string(), module_path.clone()));
          }
        }

        // Sort and deduplicate by module_name only (not the full path)
        // This prevents showing "app" twice when there's both "app" and "app::app"
        all_modules.sort();
        let mut seen_names = std::collections::HashSet::new();
        all_modules.retain(|(module_name, _)| seen_names.insert(module_name.clone()));

        if !all_modules.is_empty() {
          output.push_str(&format!("## {}\n\n", type_name));
          for (module_name, module_path) in all_modules {
            // For re-exported modules, link to their original location
            let link_path = module_path
              .strip_prefix(&format!("{}::", crate_name))
              .unwrap_or(&module_path)
              .replace("::", "/");

            // Try to get documentation from root_items
            let doc_line = root_items
              .iter()
              .find(|(_, item)| {
                if let Some(item_name) = &item.name {
                  item_name == &module_name && matches!(&item.inner, ItemEnum::Module(_))
                } else {
                  false
                }
              })
              .and_then(|(_, item)| item.docs.as_ref())
              .and_then(|docs| docs.lines().next())
              .filter(|line| !line.is_empty());

            // Only add " — " if there's documentation
            if let Some(doc_text) = doc_line {
              output.push_str(&format!(
                "<div><Link to=\"{}/\" className=\"rust-mod\">{}</Link> — {}</div>\n\n",
                link_path, module_name, doc_text
              ));
            } else {
              output.push_str(&format!(
                "<div><Link to=\"{}/\" className=\"rust-mod\">{}</Link></div>\n\n",
                link_path, module_name
              ));
            }
          }
        }
        continue;
      }

      if let Some(items_of_type) = by_type.get(type_name) {
        output.push_str(&format!("## {}\n\n", type_name));

        // Determine CSS class based on type
        let css_class = match *type_name {
          "Structs" | "Enums" => "rust-struct",
          "Traits" => "rust-trait",
          "Functions" => "rust-fn",
          "Constants" => "rust-constant",
          "Type Aliases" => "rust-type",
          _ => "rust-item",
        };

        for item in items_of_type {
          if let Some(name) = &item.name {
            // Other items link to their individual pages with rustdoc-style prefix
            let prefix = get_item_prefix(item);
            let link = format!("{}{}", prefix, name);
            let visibility_indicator = get_visibility_indicator(item);

            output.push_str("<div>");
            output.push_str(&format!(
              "<Link to=\"{}\" className=\"{}\">{}</Link> {}",
              link, css_class, name, visibility_indicator
            ));
            if let Some(docs) = &item.docs {
              let sanitized = sanitize_docs_for_mdx(docs);
              if let Some(first_line) = sanitized.lines().next() {
                if !first_line.is_empty() {
                  output.push_str(&format!(" — {}", first_line));
                }
              }
            }
            output.push_str("</div>\n\n");
          }
        }
      }
    }
  }

  output
}

#[allow(clippy::too_many_arguments)]
fn generate_individual_pages(
  items: &[(Id, Item)],
  path_prefix: &str,
  files: &mut HashMap<String, String>,
  _crate_data: &Crate,
  item_paths: &HashMap<Id, Vec<String>>,
  _crate_name: &str,
  _module_name: &str,
  include_private: bool,
) {
  for (id, item) in items {
    // Skip Use items (re-exports) - they're only shown in the module overview
    // The actual items are documented in their original modules
    if matches!(&item.inner, ItemEnum::Use(_)) {
      continue;
    }

    if let Some(name) = &item.name {
      // Skip module items as they get their own overview pages
      if matches!(&item.inner, ItemEnum::Module(_)) {
        continue;
      }

      // Use rustdoc-style prefix for item filename (e.g., "fn.send_message.md")
      let item_prefix = get_item_prefix(item);
      let file_path = format!("{}{}{}.md", path_prefix, item_prefix, name);

      if let Some(mut content) =
        format_item_with_path(id, item, _crate_data, item_paths, include_private)
      {
        // Add frontmatter for Docusaurus navigation with type label and sidebar
        let type_label = get_item_type_label(item);
        let title = if type_label.is_empty() {
          name.to_string()
        } else {
          format!("{} {}", type_label, name)
        };

        // Calculate sidebar key from module path (same as module overview)
        let base_path = BASE_PATH.with(|bp| bp.borrow().clone());
        let base_path_for_sidebar = base_path
          .strip_prefix("/docs/")
          .or_else(|| base_path.strip_prefix("/docs"))
          .or_else(|| base_path.strip_prefix("/"))
          .unwrap_or(&base_path);
        let sidebar_key = if _module_name == _crate_name {
          // For items in the crate root, use "_items" suffix
          // to match the sidebar generated for leaf items of the crate
          format!("{}/{}_items", base_path_for_sidebar, _crate_name).replace("/", "_")
        } else {
          let module_path = _module_name.replace("::", "/");
          format!("{}/{}", base_path_for_sidebar, module_path).replace("/", "_")
        };

        let frontmatter = format!(
          "---\ntitle: \"{}\"\ndisplayed_sidebar: '{}'\n---\n\nimport RustCode from '@site/src/components/RustCode';\nimport Link from '@docusaurus/Link';\n\n",
          title, sidebar_key
        );

        // Add breadcrumb path (like rustdoc does for all items)
        // For re-exported items (duplicates), use the current module path + item name
        // For original items, use their full path from item_paths
        let breadcrumb = if _module_name == _crate_name {
          // Root module - just crate::ItemName
          format!("**{}::{}**\n\n", _module_name, name)
        } else {
          // Check if this is the original location or a re-export
          let original_path = item_paths.get(id).map(|p| p.join("::"));
          let expected_path = format!("{}::{}", _module_name, name);

          // If the original path matches the expected path, it's the original item
          // Otherwise, it's a re-exported duplicate - use the current module path
          if original_path.as_deref() == Some(expected_path.as_str()) {
            format!("**{}**\n\n", expected_path)
          } else {
            // Re-exported item - use current module path
            format!("**{}**\n\n", expected_path)
          }
        };

        content = format!("{}{}{}", frontmatter, breadcrumb, content);
        files.insert(file_path, content);
      }
    }
  }
}

#[allow(clippy::same_item_push)]
fn generate_module_overview(
  module_name: &str,
  items: &[(Id, Item)],
  _crate_data: &Crate,
  _item_paths: &HashMap<Id, Vec<String>>,
  crate_name: &str,
  module_hierarchy: &HashMap<String, Vec<String>>,
) -> String {
  let mut output = String::new();

  let display_name = module_name
    .strip_prefix(&format!("{}::", crate_name))
    .unwrap_or(module_name);

  // Get just the last component of the module name (rustdoc style)
  let short_name = display_name.split("::").last().unwrap_or(display_name);

  // Calculate sidebar key from module path
  let base_path = BASE_PATH.with(|bp| bp.borrow().clone());
  let base_path_for_sidebar = base_path
    .strip_prefix("/docs/")
    .or_else(|| base_path.strip_prefix("/docs"))
    .or_else(|| base_path.strip_prefix("/"))
    .unwrap_or(&base_path);

  // For module overview pages, use the PARENT module's sidebar
  // This way the module page shows "In <parent>" with siblings
  let sidebar_module = if module_name == crate_name {
    // Root module uses its own sidebar
    crate_name.to_string()
  } else if module_name.contains("::") {
    // Sub-module uses parent's sidebar
    module_name.rsplit_once("::").unwrap().0.to_string()
  } else {
    // Top-level module (crate_name::module) uses crate's sidebar
    crate_name.to_string()
  };

  let sidebar_key = if sidebar_module == crate_name {
    // If this module's parent is the crate root, use the "_modules" variant
    // which shows "In <crate>" with crate's modules, not "Crates"
    if module_name == crate_name {
      // This IS the crate root page itself - use the regular sidebar
      format!("{}/{}", base_path_for_sidebar, crate_name).replace("/", "_")
    } else {
      // This is a child of the crate root - use the "_modules" variant
      format!("{}/{}_modules", base_path_for_sidebar, crate_name).replace("/", "_")
    }
  } else {
    // This module's parent is another module (not the crate)
    // Use the parent's "_children" sidebar which shows the parent's contents
    let module_path = sidebar_module.replace("::", "/");
    format!("{}/{}_children", base_path_for_sidebar, module_path).replace("/", "_")
  };

  // Add FrontMatter for Docusaurus with the module name as title and sidebar
  output.push_str("---\n");
  output.push_str(&format!("title: {}\n", short_name));
  output.push_str(&format!("sidebar_label: {}\n", short_name));
  output.push_str(&format!("displayed_sidebar: '{}'\n", sidebar_key));
  output.push_str("---\n\n");

  // Import RustCode component
  output.push_str("import RustCode from '@site/src/components/RustCode';\n");
  output.push_str("import Link from '@docusaurus/Link';\n\n");

  // Breadcrumb with :: separator (rustdoc style)
  let breadcrumb = module_name;
  output.push_str(&format!("**{}**\n\n", breadcrumb));

  output.push_str(&format!("# Module {}\n\n", short_name));

  // Module documentation (if any module item exists)
  for (_id, item) in items {
    if matches!(&item.inner, ItemEnum::Module(_)) {
      if let Some(docs) = &item.docs {
        output.push_str(&format!("{}\n\n", sanitize_docs_for_mdx(docs)));
      }
      break;
    }
  }

  // Separate re-exports (Use items) from regular items
  let mut re_exports = Vec::new();
  let mut regular_items = Vec::new();

  for (id, item) in items {
    if matches!(&item.inner, ItemEnum::Use(_)) {
      re_exports.push((id, item));
    } else {
      regular_items.push((id, item));
    }
  }

  // Show Re-exports section first (if any)
  // Only show re-exports where the source module/item is public (rustdoc behavior)
  if !re_exports.is_empty() {
    let mut public_re_exports = Vec::new();

    for (id, item) in &re_exports {
      if let ItemEnum::Use(use_item) = &item.inner {
        // Check if the source is public
        let is_source_public = if let Some(import_id) = &use_item.id {
          if let Some(imported_item) = _crate_data.index.get(import_id) {
            is_public(imported_item)
          } else {
            // External dependency - always show
            true
          }
        } else {
          // No import ID - assume public
          true
        };

        if is_source_public {
          public_re_exports.push((id, item, use_item));
        }
      }
    }

    if !public_re_exports.is_empty() {
      output.push_str("## Re-exports\n\n");

      for (_id, _item, use_item) in &public_re_exports {
        // Use the full source path for proper linking
        let source_path = &use_item.source;

        // Build code string for RustCode component
        let code_str = if use_item.is_glob {
          format!("pub use {}::*;", source_path)
        } else {
          format!("pub use {};", source_path)
        };

        // Extract the final component of the path for text matching
        // e.g., "patterns::Builder" -> "Builder"
        let type_name = source_path.split("::").last().unwrap_or(source_path);

        // Try to find link to the re-exported item using absolute links
        let links: Vec<(String, String)> = if let Some(import_id) = &use_item.id {
          if let Some(link) = generate_type_link(source_path, import_id, _crate_data, None) {
            vec![(type_name.to_string(), link)]
          } else {
            // External dependency - no link
            vec![]
          }
        } else {
          vec![]
        };

        let links_json = format_links_as_json(&links);

        // Use RustCode inline component for consistent formatting
        output.push_str(&format!(
          "<RustCode inline code={{`{}`}} links={{{}}} />\n\n",
          code_str, links_json
        ));
      }
    }
  }

  // Table of contents for this module (rustdoc style overview)
  let mut by_type: HashMap<&str, Vec<(&Id, &Item)>> = HashMap::new();
  for (id, item) in &regular_items {
    let type_name = match &item.inner {
      ItemEnum::Struct(_) => "Structs",
      ItemEnum::Enum(_) => "Enums",
      ItemEnum::Function(_) => "Functions",
      ItemEnum::Trait(_) => "Traits",
      ItemEnum::Constant { .. } => "Constants",
      ItemEnum::TypeAlias(_) => "Type Aliases",
      ItemEnum::Module(_) => continue, // Skip modules from items, we'll use hierarchy instead
      ItemEnum::Use(_) => continue,    // Use items are handled separately in Re-exports section
      _ => continue,
    };
    by_type.entry(type_name).or_default().push((id, item));
  }

  let type_order = [
    "Modules",
    "Structs",
    "Enums",
    "Functions",
    "Traits",
    "Constants",
    "Type Aliases",
  ];
  for type_name in &type_order {
    // Special handling for Modules - use hierarchy instead of items
    if *type_name == "Modules" {
      if let Some(submodules) = module_hierarchy.get(module_name) {
        if !submodules.is_empty() {
          // Collect all submodules
          let mut valid_submodules = Vec::new();
          for submodule_path in submodules {
            let submodule_name = submodule_path.split("::").last().unwrap_or(submodule_path);
            valid_submodules.push((submodule_path, submodule_name));
          }

          // Only show Modules section if there are valid submodules
          if !valid_submodules.is_empty() {
            output.push_str(&format!("## {}\n\n", type_name));
            for (submodule_path, submodule_name) in valid_submodules {
              // Try to get the module item from the crate index
              let module_item = _crate_data.index.iter().find(|(_, item)| {
                if let Some(item_name) = &item.name {
                  // Match by path to handle re-exported modules
                  item_name == submodule_name
                    && matches!(&item.inner, ItemEnum::Module(_))
                    && submodule_path.ends_with(&format!("::{}", submodule_name))
                } else {
                  false
                }
              });

              let visibility_indicator = module_item
                .map(|(_, item)| get_visibility_indicator(item))
                .unwrap_or("");

              let doc_line = module_item
                .and_then(|(_, item)| item.docs.as_ref())
                .and_then(|docs| docs.lines().next())
                .filter(|line| !line.is_empty());

              // Only add " — " if there's documentation
              if let Some(doc_text) = doc_line {
                output.push_str(&format!(
                  "<div><Link to=\"{}/\" className=\"rust-mod\">{}</Link> {} — {}</div>\n\n",
                  submodule_name, submodule_name, visibility_indicator, doc_text
                ));
              } else {
                output.push_str(&format!(
                  "<div><Link to=\"{}/\" className=\"rust-mod\">{}</Link> {}</div>\n\n",
                  submodule_name, submodule_name, visibility_indicator
                ));
              }
            }
          }
        }
      }
      continue;
    }

    if let Some(items_of_type) = by_type.get(type_name) {
      output.push_str(&format!("## {}\n\n", type_name));

      // Determine CSS class based on type
      let css_class = match *type_name {
        "Modules" => "rust-mod",
        "Structs" | "Enums" => "rust-struct",
        "Traits" => "rust-trait",
        "Functions" => "rust-fn",
        "Constants" => "rust-constant",
        "Type Aliases" => "rust-type",
        _ => "rust-item",
      };

      for (id, item) in items_of_type {
        // For Use items, get the name from the use.name field
        let item_name: Option<&String> = if let ItemEnum::Use(use_item) = &item.inner {
          Some(&use_item.name)
        } else {
          item.name.as_ref()
        };

        if let Some(name) = item_name {
          // Special handling for Use items (external re-exports)
          if let ItemEnum::Use(_) = &item.inner {
            // For external re-exports, just show the name without a link
            // (we don't have the full item definition to create a proper page)
            // or we could link to the external documentation if available
            let prefix = "struct."; // Default to struct prefix
            output.push_str(&format!("[{}]({}{})\n", name, prefix, name));
            continue;
          }

          // Determine the correct link path
          let link = if let Some(item_path) = _item_paths.get(id) {
            // Get the module part of the item path (all except last element)
            let item_module_path = if item_path.len() > 1 {
              &item_path[..item_path.len() - 1]
            } else {
              item_path.as_slice()
            };
            let item_module = item_module_path.join("::");

            // Check if this item is defined directly in the current module
            if item_module == module_name {
              // Item is defined directly in this module - use simple link with prefix
              let prefix = get_item_prefix(item);
              format!("{}{}", prefix, name)
            } else {
              // Item is in a submodule or re-exported from elsewhere - calculate relative path
              let current_module_parts: Vec<&str> = module_name.split("::").collect();
              let item_module_parts = item_module_path;

              // Calculate relative path
              let mut relative_parts = Vec::new();

              // Go up to common ancestor
              let common_prefix_len = current_module_parts
                .iter()
                .zip(item_module_parts.iter())
                .take_while(|(a, b)| a == b)
                .count();

              // Add ".." for each level up
              for _ in 0..(current_module_parts.len() - common_prefix_len) {
                relative_parts.push("..");
              }

              // Add path down to item
              for part in &item_module_parts[common_prefix_len..] {
                relative_parts.push(part);
              }

              let prefix = get_item_prefix(item);
              let mut path = relative_parts.join("/");
              if !path.is_empty() {
                path.push('/');
              }
              path.push_str(&format!("{}{}", prefix, name));
              path
            }
          } else {
            // Fallback if no path info
            let prefix = get_item_prefix(item);
            format!("{}{}", prefix, name)
          };

          let visibility_indicator = get_visibility_indicator(item);

          output.push_str("<div>");
          output.push_str(&format!(
            "<Link to=\"{}\" className=\"{}\">{}</Link> {}",
            link, css_class, name, visibility_indicator
          ));
          if let Some(docs) = &item.docs {
            let sanitized = sanitize_docs_for_mdx(docs);
            if let Some(first_line) = sanitized.lines().next() {
              if !first_line.is_empty() {
                output.push_str(&format!(" — {}", first_line));
              }
            }
          }
          output.push_str("</div>\n\n");
        }
      }
    }
  }

  output
}

/// Generate sidebar structure for Docusaurus
/// This generates multiple sidebars - one for each module that has content
fn generate_all_sidebars(
  crate_name: &str,
  modules: &HashMap<String, Vec<(Id, Item)>>,
  _item_paths: &HashMap<Id, Vec<String>>,
  crate_data: &Crate,
  sidebarconfig_collapsed: bool,
) -> String {
  let mut all_sidebars = HashMap::new();

  // Get the base_path from thread-local storage
  let base_path = BASE_PATH.with(|bp| bp.borrow().clone());

  // For Docusaurus sidebar, paths must be relative to the docs/ folder
  let sidebar_prefix = if base_path == "/docs" || base_path == "docs" {
    ""
  } else if base_path.starts_with("/docs/") {
    base_path.strip_prefix("/docs/").unwrap()
  } else if base_path.starts_with("docs/") {
    base_path.strip_prefix("docs/").unwrap()
  } else {
    &base_path
  };

  // Generate TWO sidebars for the root crate:
  // 1. With is_root=true (shows "Crates" section) - used by the crate's own page
  let root_sidebar_for_crate = generate_sidebar_for_module(
    crate_name,
    crate_name,
    modules,
    crate_data,
    sidebar_prefix,
    sidebarconfig_collapsed,
    true, // is_root - shows "Crates" section
    &crate_data.crate_version,
    false, // show_all_parent_items - false for modules
  );

  let root_path = if sidebar_prefix.is_empty() {
    crate_name.to_string()
  } else {
    format!("{}/{}", sidebar_prefix, crate_name)
  };
  all_sidebars.insert(root_path.clone(), root_sidebar_for_crate);

  // 2. With is_root=false (shows crate's modules) - used by the crate's child modules
  let root_sidebar_for_modules = generate_sidebar_for_module(
    crate_name,
    crate_name,
    modules,
    crate_data,
    sidebar_prefix,
    sidebarconfig_collapsed,
    false, // is_root=false - shows "In <parent>" with crate's modules
    &crate_data.crate_version,
    false, // show_all_parent_items - false for modules
  );

  // Use a different key for this sidebar (add "_modules" suffix)
  let root_path_for_modules = format!("{}_modules", root_path);
  all_sidebars.insert(root_path_for_modules, root_sidebar_for_modules);

  // Generate sidebar for each submodule (for dynamic sidebar when entering modules)
  eprintln!("[DEBUG] Total modules to process: {}", modules.keys().len());
  for module_key in modules.keys() {
    eprintln!("[DEBUG] Processing module: {}", module_key);
    if module_key == crate_name {
      eprintln!("[DEBUG] Skipping root crate: {}", crate_name);
      continue; // Skip root, already handled
    }

    let sidebar = generate_sidebar_for_module(
      crate_name,
      module_key,
      modules,
      crate_data,
      sidebar_prefix,
      sidebarconfig_collapsed,
      false, // not root
      &crate_data.crate_version,
      false, // show_all_parent_items - false for modules
    );

    // Convert module_key from Rust path (::) to file path (/)
    let module_path_normalized = module_key.replace("::", "/");
    let module_path = if sidebar_prefix.is_empty() {
      module_path_normalized.clone()
    } else {
      format!("{}/{}", sidebar_prefix, module_path_normalized)
    };
    all_sidebars.insert(module_path.clone(), sidebar);

    // Check if this module has sub-modules (direct children) or items
    // Generate a _children sidebar if the module has submodules or items (structs, enums, etc.)
    let has_submodules_or_items = modules.keys().any(|key| {
      if let Some(stripped) = key.strip_prefix(&format!("{}::", module_key)) {
        !stripped.contains("::")
      } else {
        false
      }
    }) || modules
      .get(module_key)
      .map(|items| {
        items
          .iter()
          .any(|(_, item)| !matches!(&item.inner, ItemEnum::Module(_) | ItemEnum::Use(_)))
      })
      .unwrap_or(false);

    eprintln!(
      "[DEBUG] Module '{}' has_submodules_or_items: {}",
      module_key, has_submodules_or_items
    );

    // If this module has sub-modules or items, generate an additional sidebar for them
    if has_submodules_or_items {
      let submodule_sidebar = generate_sidebar_for_module(
        crate_name,
        module_key, // Use this module as the "parent"
        modules,
        crate_data,
        sidebar_prefix,
        sidebarconfig_collapsed,
        false,
        &crate_data.crate_version,
        true, // show_all_parent_items = true to show THIS module's contents
      );

      // Use "_children" suffix to distinguish from the module's own sidebar
      let submodule_sidebar_key = format!("{}_children", module_path.replace("/", "_"));
      all_sidebars.insert(submodule_sidebar_key, submodule_sidebar);
    }
  }

  // Generate sidebar for each leaf item (struct, enum, trait, fn, etc.)
  // Each item gets its own sidebar showing "In <parent_module>" with all parent items
  // BUT: all items in the same module share the same sidebar!
  // So we generate one sidebar per module (not per item) and use the module path as key
  let mut processed_modules = std::collections::HashSet::new();

  for (module_key, items) in modules {
    // Skip if we already processed this module
    if processed_modules.contains(module_key) {
      continue;
    }

    // Check if this module has any non-module items
    let has_leaf_items = items
      .iter()
      .any(|(_, item)| !matches!(&item.inner, ItemEnum::Module(_) | ItemEnum::Use(_)));

    if !has_leaf_items {
      continue; // No leaf items, skip
    }

    processed_modules.insert(module_key.clone());

    // Generate sidebar for this module (to be used by all leaf items in it)
    let parent_module = module_key;

    eprintln!(
      "[DEBUG] Generating leaf items sidebar for module_key: {}",
      module_key
    );

    let item_sidebar = generate_sidebar_for_module(
      crate_name,
      parent_module,
      modules,
      crate_data,
      sidebar_prefix,
      sidebarconfig_collapsed,
      false, // is_root = false - leaf items always show "In <module>", never "Crates"
      &crate_data.crate_version,
      true, // show_all_parent_items - true for leaf items (struct, enum, etc.)
    );

    // The sidebar key is the module path (not the item path!)
    // This matches what's written in the frontmatter of item files
    let parent_module_path = parent_module.replace("::", "/");

    let sidebar_key = if sidebar_prefix.is_empty() {
      parent_module_path.clone()
    } else {
      format!("{}/{}", sidebar_prefix, parent_module_path)
    };

    // If this is for leaf items of the crate root, add "_items" suffix
    // to avoid collision with the crate's own sidebar (which shows "Crates")
    let sidebar_key = if parent_module == crate_name {
      format!("{}_items", sidebar_key.replace("/", "_"))
    } else {
      sidebar_key
    };

    all_sidebars.insert(sidebar_key, item_sidebar);
  }

  // Convert to TypeScript with multiple sidebars
  sidebars_to_js(&all_sidebars, sidebarconfig_collapsed)
}

/// Generate sidebar for a specific module
#[allow(clippy::too_many_arguments)]
fn generate_sidebar_for_module(
  _crate_name: &str, // Prefixed with _ to avoid unused warning
  module_key: &str,
  modules: &HashMap<String, Vec<(Id, Item)>>,
  _crate_data: &Crate, // Prefixed with _ to avoid unused warning
  sidebar_prefix: &str,
  _sidebarconfig_collapsed: bool, // Prefixed with _ to avoid unused warning
  is_root: bool,
  crate_version: &Option<String>,
  show_all_parent_items: bool, // New parameter: if true, show all items in parent module (for leaf items)
) -> Vec<SidebarItem> {
  let module_items = modules.get(module_key).cloned().unwrap_or_default();

  // Convert module_key from :: to / for doc IDs
  let _module_path = module_key.replace("::", "/"); // Prefixed with _ to avoid unused warning

  let mut sidebar_items = Vec::new();

  // Add "Go back" link and crate title for root crates, or just crate title for modules
  if is_root {
    // For root crate: use the configured sidebar_root_link if available
    let sidebar_root_link = SIDEBAR_ROOT_LINK.with(|srl| srl.borrow().clone());

    if let Some(link) = sidebar_root_link {
      sidebar_items.push(SidebarItem::Link {
        href: link,
        label: "← Go back".to_string(),
        custom_props: Some("rust-sidebar-back-link".to_string()),
      });
    }

    // Add crate title with version for root crates
    // The title itself is clickable and links to the crate index
    let crate_root_path = if sidebar_prefix.is_empty() {
      format!("{}/index", _crate_name)
    } else {
      format!("{}/{}/index", sidebar_prefix, _crate_name)
    };

    // Use customProps to pass crate name and version to a custom sidebar component
    sidebar_items.push(SidebarItem::Doc {
      id: crate_root_path,
      label: Some(_crate_name.to_string()), // Fallback label
      custom_props: Some(format!(
        "{{ rustCrateTitle: true, crateName: '{}', version: '{}' }}",
        _crate_name,
        crate_version.as_deref().unwrap_or("")
      )),
    });

    // For root crate, the title is already clickable, so we don't add a separate Overview
  } else {
    // For submodules: show crate name with version (rustdoc style)
    // This links to the crate root
    let crate_root_path = if sidebar_prefix.is_empty() {
      format!("{}/index", _crate_name)
    } else {
      format!("{}/{}/index", sidebar_prefix, _crate_name)
    };

    // Use customProps to pass crate name and version to a custom sidebar component
    sidebar_items.push(SidebarItem::Doc {
      id: crate_root_path,
      label: Some(_crate_name.to_string()), // Fallback label
      custom_props: Some(format!(
        "{{ rustCrateTitle: true, crateName: '{}', version: '{}' }}",
        _crate_name,
        crate_version.as_deref().unwrap_or("")
      )),
    });

    // Module title commented out - the overview is already on the right side
    // We don't need a separate "Overview" link in the sidebar
    /*
    // Add Overview link to the submodule's index
    // Use customProps to render it as a module title (similar to crate title but without version)
    let module_index_path = if sidebar_prefix.is_empty() {
        format!("{}/index", module_path)
    } else {
        format!("{}/{}/index", sidebar_prefix, module_path)
    };

    let module_display_name = module_key.split("::").last().unwrap_or(module_key);

    sidebar_items.push(SidebarItem::Doc {
        id: module_index_path,
        label: Some(module_display_name.to_string()),
        custom_props: Some(format!(
            "{{ rustModuleTitle: true, moduleName: '{}' }}",
            module_display_name
        )),
    });
    */
  }

  // Categorize items by type
  let mut by_type: HashMap<&str, Vec<&Item>> = HashMap::new();

  for (_, item) in &module_items {
    if matches!(&item.inner, ItemEnum::Use(_)) {
      continue;
    }

    let type_name = match &item.inner {
      ItemEnum::Module(_) => "Modules",
      ItemEnum::Struct(_) | ItemEnum::StructField(_) => "Structs",
      ItemEnum::Enum(_) | ItemEnum::Variant(_) => "Enums",
      ItemEnum::Function(_) => "Functions",
      ItemEnum::Trait(_) => "Traits",
      ItemEnum::Constant { .. } => "Constants",
      ItemEnum::TypeAlias(_) => "Type Aliases",
      ItemEnum::Macro(_) => "Macros",
      ItemEnum::ProcMacro(_) => "Proc Macros",
      ItemEnum::Static { .. } => "Statics",
      _ => continue,
    };

    by_type.entry(type_name).or_default().push(item);
  }

  // Add "In <parent>" section for ALL modules and crates (rustdoc style)
  // - For crate root (is_root = true): show workspace sibling crates
  // - For modules: show "In <parent>" with parent's content
  // - For leaf items: show "In <module>" with module's content

  // Determine which module's items to show based on show_all_parent_items and is_root:
  let (parent_module, siblings_label) = if show_all_parent_items {
    // For leaf items: show all items from the current module (not parent)
    eprintln!("[DEBUG] Leaf item sidebar for module_key: {}", module_key);
    (Some(module_key), format!("In {}", module_key))
  } else if is_root {
    // For root crate with is_root=true: show ONLY workspace crates, not the crate's modules
    // The workspace crates section is added separately below
    eprintln!(
      "[DEBUG] Root crate sidebar (is_root=true) for module_key: {}",
      module_key
    );
    (None, String::new()) // Don't collect any modules, only show "Crates" section
  } else if module_key == _crate_name {
    // For root crate with is_root=false: show crate's own modules
    // This is used by the crate's child modules to navigate
    eprintln!(
      "[DEBUG] Root crate sidebar (is_root=false) for module_key: {}",
      module_key
    );
    (Some(module_key), format!("In {}", _crate_name))
  } else if module_key.contains("::") {
    // For modules: has parent module - show siblings
    let parent = module_key.rsplit_once("::").unwrap().0;
    eprintln!(
      "[DEBUG] Module sidebar for module_key: {}, parent: {}",
      module_key, parent
    );
    (Some(parent), format!("In {}", parent))
  } else {
    // For top-level modules: show siblings in crate
    eprintln!(
      "[DEBUG] Top-level module sidebar for module_key: {}",
      module_key
    );
    (None, format!("In crate {}", _crate_name))
  };

  // Rustdoc-style: Group parent items by type (Modules, Structs, Enums, etc.)
  // Use the same type_order as before
  let type_order = vec![
    "Modules",
    "Macros",
    "Structs",
    "Enums",
    "Traits",
    "Functions",
    "Type Aliases",
    "Constants",
    "Statics",
    "Primitives",
  ];

  // Group items by type using HashMap
  use std::collections::HashMap;
  let mut items_by_type: HashMap<&str, Vec<SidebarItem>> = HashMap::new();

  // For both modules and leaf items, we need to add child modules
  // - For modules: children of the parent module (siblings of current module)
  // - For leaf items: children of the current module (submodules)
  let child_modules: Vec<&String> = modules
    .keys()
    .filter(|key| {
      if let Some(target_module) = parent_module {
        // Check if this is a direct child of target_module
        // A direct child has the form: target_module::child_name (one more :: than target)
        let target_prefix = format!("{}::", target_module);
        if key.starts_with(&target_prefix) {
          // Count :: in both strings to ensure it's a direct child, not a grandchild
          let target_colons = target_module.matches("::").count();
          let key_colons = key.matches("::").count();
          key_colons == target_colons + 1
        } else {
          false
        }
      } else {
        // Top-level modules of the crate (children of crate root)
        !key.contains("::") && *key != _crate_name
      }
    })
    .collect();

  for child_key in child_modules {
    let child_name = child_key.split("::").last().unwrap_or(child_key);
    let child_path = child_key.replace("::", "/");
    let child_doc_id = if sidebar_prefix.is_empty() {
      format!("{}/index", child_path)
    } else {
      format!("{}/{}/index", sidebar_prefix, child_path)
    };

    let label = child_name.to_string();

    items_by_type
      .entry("Modules")
      .or_default()
      .push(SidebarItem::Doc {
        id: child_doc_id,
        label: Some(label),
        custom_props: Some("rust-mod".to_string()),
      });
  }

  // Add all other items (structs, enums, functions, etc.) from parent_module
  let parent_items_source = if let Some(parent_key) = parent_module {
    modules.get(parent_key)
  } else {
    modules.get(_crate_name)
  };

  if let Some(parent_module_items) = parent_items_source {
    for (_item_id, item) in parent_module_items {
      if let Some(item_name) = &item.name {
        // Skip modules (already added above)
        if matches!(&item.inner, ItemEnum::Module(_)) {
          continue;
        }

        let prefix = get_item_prefix(item);
        let parent_path = if let Some(pk) = parent_module {
          pk.replace("::", "/")
        } else {
          _crate_name.to_string()
        };

        let item_doc_id = if sidebar_prefix.is_empty() {
          format!("{}/{}{}", parent_path, prefix, item_name)
        } else {
          format!("{}/{}/{}{}", sidebar_prefix, parent_path, prefix, item_name)
        };

        // Determine CSS class and type category based on item type
        let (class_name, type_category) = if prefix.starts_with("struct.") {
          ("rust-struct", "Structs")
        } else if prefix.starts_with("enum.") {
          ("rust-struct", "Enums")
        } else if prefix.starts_with("trait.") {
          ("rust-trait", "Traits")
        } else if prefix.starts_with("fn.") {
          ("rust-fn", "Functions")
        } else if prefix.starts_with("constant.") {
          ("rust-constant", "Constants")
        } else if prefix.starts_with("type.") {
          ("rust-type", "Type Aliases")
        } else if prefix.starts_with("macro.") {
          ("rust-macro", "Macros")
        } else if prefix.starts_with("static.") {
          ("rust-static", "Statics")
        } else {
          ("rust-item", "Primitives")
        };

        items_by_type
          .entry(type_category)
          .or_default()
          .push(SidebarItem::Doc {
            id: item_doc_id,
            label: Some(item_name.clone()),
            custom_props: Some(class_name.to_string()),
          });
      }
    }
  } // Close if let Some(parent_module_items)

  // Create categories for each type that has items
  let mut parent_section_items = Vec::new();
  for type_name in type_order {
    if let Some(items) = items_by_type.get(type_name) {
      if !items.is_empty() {
        parent_section_items.push(SidebarItem::Category {
          label: type_name.to_string(),
          items: items.clone(),
          collapsed: false, // Will be rendered as collapsible: false
          link: None,
        });
      }
    }
  }

  // Generate link to parent module
  let parent_link = if let Some(parent_key) = parent_module {
    let parent_path = parent_key.replace("::", "/");
    if sidebar_prefix.is_empty() {
      Some(format!("{}/index", parent_path))
    } else {
      Some(format!("{}/{}/index", sidebar_prefix, parent_path))
    }
  } else {
    // Parent is crate root
    if sidebar_prefix.is_empty() {
      Some(format!("{}/index", _crate_name))
    } else {
      Some(format!("{}/{}/index", sidebar_prefix, _crate_name))
    }
  };

  // Add "In <parent>" section in these cases:
  // - For leaf items (show_all_parent_items=true): always wrap in "In <module>"
  // - For sub-modules where parent is NOT the crate: wrap in "In <parent>"
  // - For modules where parent IS the crate: DON'T wrap (rustdoc behavior without TOC)
  // Root crates (is_root=true) will show "Crates" section instead (added below)
  let should_wrap_in_category = !is_root
    && !parent_section_items.is_empty()
    && (show_all_parent_items || parent_module != Some(_crate_name));

  if should_wrap_in_category {
    sidebar_items.push(SidebarItem::Category {
      label: siblings_label,
      items: parent_section_items,
      collapsed: false, // Keep open like rustdoc
      link: parent_link,
    });
  } else if !is_root && !parent_section_items.is_empty() {
    // For modules whose parent is the crate: add categories directly without wrapper
    sidebar_items.extend(parent_section_items);
  }

  // For root crates: add "Crates" section with workspace sibling crates
  if is_root {
    let workspace_crates = WORKSPACE_CRATES.with(|wc| wc.borrow().clone());

    if workspace_crates.len() > 1 {
      let mut crate_items = Vec::new();

      for crate_name in &workspace_crates {
        // Normalize crate name: replace hyphens with underscores for file paths
        let normalized_crate_name = crate_name.replace("-", "_");

        let crate_doc_id = if sidebar_prefix.is_empty() {
          format!("{}/index", normalized_crate_name)
        } else {
          format!("{}/{}/index", sidebar_prefix, normalized_crate_name)
        };

        let label = crate_name.to_string();

        crate_items.push(SidebarItem::Doc {
          id: crate_doc_id,
          label: Some(label),
          custom_props: Some("rust-mod".to_string()),
        });
      }

      // Sort crate items by label (alphabetically)
      crate_items.sort_by(|a, b| {
        let label_a = match a {
          SidebarItem::Doc { label, .. } => label.as_deref().unwrap_or(""),
          SidebarItem::Link { label, .. } => label.as_str(),
          SidebarItem::Category { label, .. } => label.as_str(),
        };
        let label_b = match b {
          SidebarItem::Doc { label, .. } => label.as_deref().unwrap_or(""),
          SidebarItem::Link { label, .. } => label.as_str(),
          SidebarItem::Category { label, .. } => label.as_str(),
        };
        label_a.cmp(label_b)
      });

      sidebar_items.push(SidebarItem::Category {
        label: "Crates".to_string(),
        items: crate_items,
        collapsed: false,
        link: None,
      });
    }
  }

  sidebar_items
}

/// Convert multiple sidebars to TypeScript code
fn sidebars_to_js(all_sidebars: &HashMap<String, Vec<SidebarItem>>, _collapsed: bool) -> String {
  let mut output = String::new();

  output.push_str("// This file is auto-generated by cargo-doc-md\n");
  output.push_str("// Do not edit manually - this file will be regenerated\n\n");
  output.push_str("import type {SidebarsConfig} from '@docusaurus/plugin-content-docs';\n\n");
  output.push_str("// Rust API documentation sidebars\n");
  output.push_str("// Each module has its own sidebar for better navigation\n");
  output.push_str("// Import this in your docusaurus.config.ts:\n");
  output.push_str("// import { rustSidebars } from './sidebars-rust';\n");
  output.push_str("//\n");
  output.push_str("// Then configure in docs plugin:\n");
  output.push_str("// docs: {\n");
  output.push_str("//   sidebarPath: './sidebars.ts',\n");
  output
    .push_str("//   async sidebarItemsGenerator({ defaultSidebarItemsGenerator, ...args }) {\n");
  output.push_str("//     const items = await defaultSidebarItemsGenerator(args);\n");
  output.push_str("//     const docPath = args.item.id;\n");
  output.push_str("//     // Use module-specific sidebar if available\n");
  output.push_str("//     for (const [path, sidebar] of Object.entries(rustSidebars)) {\n");
  output.push_str("//       if (docPath.startsWith(path + '/')) {\n");
  output.push_str("//         return sidebar;\n");
  output.push_str("//       }\n");
  output.push_str("//     }\n");
  output.push_str("//     return items;\n");
  output.push_str("//   },\n");
  output.push_str("// }\n\n");

  output.push_str("export const rustSidebars: Record<string, any[]> = {\n");

  // Sort by path for consistent output
  let mut sorted_paths: Vec<_> = all_sidebars.keys().cloned().collect();
  sorted_paths.sort();

  let first_path = sorted_paths.first().cloned();

  for path in &sorted_paths {
    let items = &all_sidebars[path];
    // Convert path with slashes and dots to valid sidebar key (replace / and . with _)
    let sidebar_key = path.replace("/", "_").replace(".", "_");
    output.push_str(&format!("  '{}': [\n", sidebar_key));
    for item in items {
      output.push_str(&format_sidebar_item(item, 2));
    }
    output.push_str("  ],\n");
  }

  output.push_str("};\n\n");

  // NOTE: rootRustSidebar is generated during merge in writer.rs
  // to include all crates from the workspace

  // Also export the main sidebar for backward compatibility
  if let Some(first_path) = first_path {
    let first_sidebar_key = first_path.replace("/", "_").replace(".", "_");
    output.push_str("// Main API documentation sidebar (for backward compatibility)\n");
    output.push_str("export const rustApiDocumentation = rustSidebars['");
    output.push_str(&first_sidebar_key);
    output.push_str("'];\n\n");
    output.push_str("// Or use as a single category:\n");
    output.push_str("export const rustApiCategory = {\n");
    output.push_str("  type: 'category' as const,\n");
    output.push_str("  label: 'API Documentation',\n");
    output.push_str("  collapsed: false,\n");
    output.push_str("  items: rustApiDocumentation,\n");
    output.push_str("};\n");
  }

  output
}

/// Format a single sidebar item with proper indentation
fn format_sidebar_item(item: &SidebarItem, indent: usize) -> String {
  let indent_str = "  ".repeat(indent);

  match item {
    SidebarItem::Doc {
      id,
      label,
      custom_props,
    } => {
      // Remove .md extension if present and convert to doc ID
      let doc_id = id.trim_end_matches(".md").replace(".md", "");

      // If we have a label or customProps, create an object with type, id, label, and optional className/customProps
      if label.is_some() || custom_props.is_some() {
        let mut output = format!("{}{{ type: 'doc', id: '{}'", indent_str, doc_id);

        if let Some(label_text) = label {
          output.push_str(&format!(", label: '{}'", label_text));
        }

        // Determine if custom_props is className or customProps based on format
        if let Some(props) = custom_props {
          if props.starts_with('{') {
            // It's customProps JSON object
            output.push_str(&format!(", customProps: {}", props));
          } else {
            // It's a className string
            output.push_str(&format!(", className: '{}'", props));
          }
        }

        output.push_str(" },\n");
        output
      } else {
        // Just a string reference (Docusaurus will infer the label)
        format!("{}'{doc_id}',\n", indent_str)
      }
    }
    SidebarItem::Link {
      href,
      label,
      custom_props,
    } => {
      // Generate a link item with href
      let mut output = format!(
        "{}{{ type: 'link', href: '{}', label: '{}'",
        indent_str, href, label
      );
      if let Some(props) = custom_props {
        if props.starts_with('{') {
          output.push_str(&format!(", customProps: {}", props));
        } else {
          output.push_str(&format!(", className: '{}'", props));
        }
      }
      output.push_str(" },\n");
      output
    }
    SidebarItem::Category {
      label,
      items,
      collapsed,
      link,
    } => {
      let mut output = String::new();
      output.push_str(&format!("{}{{\n", indent_str));
      output.push_str(&format!("{}  type: 'category',\n", indent_str));
      output.push_str(&format!("{}  label: '{}',\n", indent_str, label));

      // Add link if present (makes the category clickable)
      if let Some(link_path) = link {
        let doc_id = link_path.trim_end_matches(".md").replace(".md", "");
        output.push_str(&format!("{}  link: {{\n", indent_str));
        output.push_str(&format!("{}    type: 'doc',\n", indent_str));
        output.push_str(&format!("{}    id: '{}',\n", indent_str, doc_id));
        output.push_str(&format!("{}  }},\n", indent_str));
      }

      // Nested categories (indent > 0) are not collapsible (rustdoc style)
      // Top-level categories use the collapsed parameter
      if indent > 0 {
        output.push_str(&format!("{}  collapsible: false,\n", indent_str));
      } else {
        output.push_str(&format!("{}  collapsed: {},\n", indent_str, collapsed));
      }

      output.push_str(&format!("{}  items: [\n", indent_str));

      for sub_item in items {
        output.push_str(&format_sidebar_item(sub_item, indent + 2));
      }

      output.push_str(&format!("{}  ],\n", indent_str));
      output.push_str(&format!("{}}},\n", indent_str));
      output
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_sanitize_docs_for_mdx_inline_html() {
    // Test case: HTML tag inline with text (the problematic case)
    let input = "Identifies the sender of the message.\n<details><summary>JSON schema</summary>\n\n```json\n{\n  \"type\": \"string\"\n}\n```\n\n</details>";
    let result = sanitize_docs_for_mdx(input);

    // Should have a blank line before <details>
    assert!(
      result.contains("message.\n\n<details>"),
      "Expected blank line before <details>, got:\n{}",
      result
    );
  }

  #[test]
  fn test_sanitize_docs_for_mdx_already_separated() {
    // Test case: HTML already properly separated
    let input = "Some text.\n\n<details><summary>Info</summary>\nContent\n</details>\n\nMore text.";
    let result = sanitize_docs_for_mdx(input);

    // Should preserve the existing separation
    assert!(
      result.contains("text.\n\n<details>"),
      "Should preserve existing blank lines"
    );
  }

  #[test]
  fn test_sanitize_docs_for_mdx_no_html() {
    // Test case: No HTML tags
    let input = "Just some regular markdown text.\nWith multiple lines.";
    let result = sanitize_docs_for_mdx(input);

    // Should return unchanged
    assert_eq!(result, input, "Plain text should be unchanged");
  }

  #[test]
  fn test_sanitize_docs_for_mdx_inline_html_tags() {
    // Test case: Inline HTML like <code> should not be affected
    let input = "Use the `<code>` tag for inline code.";
    let result = sanitize_docs_for_mdx(input);

    // Should return unchanged (code is not a block-level tag)
    assert_eq!(result, input, "Inline HTML should be unchanged");
  }
}
