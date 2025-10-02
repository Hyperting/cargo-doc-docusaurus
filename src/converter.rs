use anyhow::Result;
use rustdoc_types::{Crate, Item, ItemEnum, Visibility};

pub fn convert_to_markdown(crate_data: &Crate, include_private: bool) -> Result<String> {
    let mut output = String::new();

    let root_item = crate_data.index.get(&crate_data.root)
        .ok_or_else(|| anyhow::anyhow!("Root item not found in index"))?;

    let crate_name = root_item.name.as_deref().unwrap_or("unknown");
    output.push_str(&format!("# {}\n\n", crate_name));

    if let Some(docs) = &root_item.docs {
        output.push_str(&format!("{}\n\n", docs));
    }

    output.push_str("## Table of Contents\n\n");

    let mut items: Vec<_> = crate_data.index.iter().collect();
    items.sort_by(|a, b| {
        let name_a = a.1.name.as_deref().unwrap_or("");
        let name_b = b.1.name.as_deref().unwrap_or("");
        name_a.cmp(name_b)
    });

    let mut toc_entries = Vec::new();
    let mut content_sections = Vec::new();

    for (id, item) in &items {
        if *id == &crate_data.root {
            continue;
        }

        if !include_private && !is_public(item) {
            continue;
        }

        if let Some(section) = format_item(item, crate_data) {
            if let Some(name) = &item.name {
                let anchor = name.to_lowercase().replace("::", "-");
                toc_entries.push(format!("- [{}](#{})", name, anchor));
                content_sections.push(section);
            }
        }
    }

    output.push_str(&toc_entries.join("\n"));
    output.push_str("\n\n---\n\n");
    output.push_str(&content_sections.join("\n\n"));

    Ok(output)
}

fn is_public(item: &Item) -> bool {
    matches!(item.visibility, Visibility::Public)
}

fn format_item(item: &Item, crate_data: &Crate) -> Option<String> {
    let name = item.name.as_ref()?;
    let mut output = String::new();

    match &item.inner {
        ItemEnum::Struct(s) => {
            output.push_str(&format!("## {}\n\n", name));
            output.push_str("**Type:** Struct\n\n");

            if let Some(docs) = &item.docs {
                output.push_str(&format!("{}\n\n", docs));
            }

            if !s.generics.params.is_empty() {
                output.push_str("**Generic Parameters:**\n");
                for param in &s.generics.params {
                    output.push_str(&format!("- {}\n", format_generic_param(param)));
                }
                output.push_str("\n");
            }

            match &s.kind {
                rustdoc_types::StructKind::Plain { fields, .. } => {
                    if !fields.is_empty() {
                        output.push_str("**Fields:**\n\n");
                        for field_id in fields {
                            if let Some(field) = crate_data.index.get(field_id) {
                                if let Some(field_name) = &field.name {
                                    output.push_str(&format!("- `{}`", field_name));
                                    if let Some(field_docs) = &field.docs {
                                        output.push_str(&format!(": {}", field_docs.lines().next().unwrap_or("")));
                                    }
                                    output.push_str("\n");
                                }
                            }
                        }
                        output.push_str("\n");
                    }
                }
                rustdoc_types::StructKind::Tuple(fields) => {
                    output.push_str(&format!("**Tuple Struct** with {} field(s)\n\n", fields.len()));
                }
                rustdoc_types::StructKind::Unit => {
                    output.push_str("**Unit Struct**\n\n");
                }
            }
        }
        ItemEnum::Enum(e) => {
            output.push_str(&format!("## {}\n\n", name));
            output.push_str("**Type:** Enum\n\n");

            if let Some(docs) = &item.docs {
                output.push_str(&format!("{}\n\n", docs));
            }

            if !e.generics.params.is_empty() {
                output.push_str("**Generic Parameters:**\n");
                for param in &e.generics.params {
                    output.push_str(&format!("- {}\n", format_generic_param(param)));
                }
                output.push_str("\n");
            }

            if !e.variants.is_empty() {
                output.push_str("**Variants:**\n\n");
                for variant_id in &e.variants {
                    if let Some(variant) = crate_data.index.get(variant_id) {
                        if let Some(variant_name) = &variant.name {
                            output.push_str(&format!("- `{}`", variant_name));
                            if let Some(variant_docs) = &variant.docs {
                                output.push_str(&format!(": {}", variant_docs.lines().next().unwrap_or("")));
                            }
                            output.push_str("\n");
                        }
                    }
                }
                output.push_str("\n");
            }
        }
        ItemEnum::Function(f) => {
            output.push_str(&format!("## {}\n\n", name));
            output.push_str("**Type:** Function\n\n");

            if let Some(docs) = &item.docs {
                output.push_str(&format!("{}\n\n", docs));
            }

            output.push_str("```rust\n");
            output.push_str(&format!("fn {}", name));

            if !f.generics.params.is_empty() {
                output.push_str("<");
                let params: Vec<String> = f.generics.params.iter()
                    .map(format_generic_param)
                    .collect();
                output.push_str(&params.join(", "));
                output.push_str(">");
            }

            output.push_str("(");
            let inputs: Vec<String> = f.sig.inputs.iter()
                .map(|(name, _type)| name.clone())
                .collect();
            output.push_str(&inputs.join(", "));
            output.push_str(")");

            output.push_str("\n```\n\n");
        }
        ItemEnum::Trait(t) => {
            output.push_str(&format!("## {}\n\n", name));
            output.push_str("**Type:** Trait\n\n");

            if let Some(docs) = &item.docs {
                output.push_str(&format!("{}\n\n", docs));
            }

            if !t.items.is_empty() {
                output.push_str("**Methods:**\n\n");
                for method_id in &t.items {
                    if let Some(method) = crate_data.index.get(method_id) {
                        if let Some(method_name) = &method.name {
                            output.push_str(&format!("- `{}`", method_name));
                            if let Some(method_docs) = &method.docs {
                                output.push_str(&format!(": {}", method_docs.lines().next().unwrap_or("")));
                            }
                            output.push_str("\n");
                        }
                    }
                }
                output.push_str("\n");
            }
        }
        ItemEnum::Module(_) => {
            output.push_str(&format!("## Module: {}\n\n", name));

            if let Some(docs) = &item.docs {
                output.push_str(&format!("{}\n\n", docs));
            }
        }
        ItemEnum::Constant { .. } => {
            output.push_str(&format!("## {}\n\n", name));
            output.push_str("**Type:** Constant\n\n");

            if let Some(docs) = &item.docs {
                output.push_str(&format!("{}\n\n", docs));
            }
        }
        ItemEnum::TypeAlias(_) => {
            output.push_str(&format!("## {}\n\n", name));
            output.push_str("**Type:** Type Alias\n\n");

            if let Some(docs) = &item.docs {
                output.push_str(&format!("{}\n\n", docs));
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
            format!("'{}", param.name)
        }
        rustdoc_types::GenericParamDefKind::Type { .. } => {
            param.name.clone()
        }
        rustdoc_types::GenericParamDefKind::Const { .. } => {
            format!("const {}", param.name)
        }
    }
}
