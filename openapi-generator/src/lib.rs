#![feature(is_some_and)]
extern crate core;

use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::{create_dir_all, File};
use std::io::{Read, Write};
use convert_case::{Case, Casing};
use genco::lang::{rust::Tokens};
use genco::{quote};
use crate::model::{Schema, Spec};
use crate::schemas::{Render, Type};
use crate::schemas::reference::resolve;
use crate::tree::TreeNode;

mod model;
mod schemas;
mod tree;

pub fn read_spec(path: &str) -> Spec {
    let file = File::open(path).unwrap();
    let spec: Spec = serde_json::from_reader(file).unwrap();
    spec
}

fn calculate_name(name: &str) -> (Vec<String>, String) {
    let parts = name.split('.');
    let mut parts = parts.collect::<Vec<&str>>();
    let name = parts.last().unwrap().to_string();
    parts.pop();
    let namespace = parts.iter_mut()
        .map(|s| s.to_lowercase())
        .collect::<Vec<_>>();
    (namespace, name)
}

pub fn format_description(description: Option<String>) -> String {
    description
        .unwrap_or_else(|| String::from("No documentation provided.")).lines()
        .map(|l| format!("/// {}", l)).collect::<Vec<_>>().join("\n")
}

fn render_schema(
    name: String,
    schema: Schema
) -> Tokens {
    Type::from(schema).render(name)
}

fn generate_node(path: &str, node: &TreeNode<String>, sorted_schemas: &HashMap<String, Vec<(String, Schema)>>, header: &[u8], output: &str) {
    let name = node.get_value();
    let namespace = format!("{}/{}", path, name);
    let namespace = namespace.strip_prefix("/").unwrap_or(&namespace);
    let dir = get_dir_for_namespace(&output, &namespace);
    let sorted_children = node.get_children().iter().map(|c| c.get_value().as_ref()).collect::<Vec<_>>();
    let sorted_children = sorted_by(sorted_children, |a: &&str, b: &&str| a.cmp(b));
    let module_bytes = generate_module_bytes(&sorted_children);

    create_dir_all(&dir).unwrap();

    let mut file_path = format!("{}/{}.rs", dir, name);
    if file_path.ends_with("/.rs") {
        file_path = file_path.replace("/.rs", ".rs");
    }

    let mut file = File::create(&file_path).unwrap();
    file.write_all(header).unwrap();
    file.write_all(&module_bytes).unwrap();
    let schemas = sorted_schemas.get(namespace);

    if let Some(schemas) = schemas {
        println!("Generating file: {} with namespace {} containing {} schemas", &file_path, namespace, schemas.len());
        let mut sorted_schemas = sorted_by(schemas.clone(), |(a, _), (b, _)| a.cmp(b));
        let tokens: Tokens = sorted_schemas.into_iter().flat_map(|(name, schema)| {
            render_schema(name.clone(), schema.clone())
        }).collect();
        file.write_all(tokens.to_file_string().unwrap().as_bytes()).unwrap();
    } else {
        println!("Generating file: {} with namespace {} containing 0 schemas", &file_path, namespace);
    }

    for child in node.get_children() {
        generate_node(&namespace, child, sorted_schemas, header, output);
    }
}

pub fn generate(spec: Spec, output: &str) {
    let schemas: Vec<_> = spec.components.schemas.into_iter().map(|(name, schema)| {
        let (namespace, name) = resolve(&name);
        (namespace, name, schema)
    }).collect();
    let mut sorted_schemas = HashMap::new();
    for (namespace, name, schema) in schemas {
        sorted_schemas.entry(namespace.to_case(Case::Snake))
            .or_insert_with(|| Vec::new())
            .push((name, schema));
    }
    let mut root: TreeNode<String> = TreeNode::new("".to_owned());
    for namespace in sorted_schemas.keys() {
        let segments = namespace.split('/').collect::<Vec<_>>();
        let mut current = &mut root;
        for segment in segments {
            if segment.is_empty() {
                continue;
            }
            current = current.get_or_create_child(segment.to_owned());
        }
    }
    let header = generate_header_bytes();
    generate_node("", &root, &sorted_schemas, &header, output);

    return;
}

fn sorted_by<T, F>(mut v: Vec<T>, mut compare: F) -> Vec<T>
    where F: FnMut(&T, &T) -> Ordering
{
    v.sort_by(compare);
    v
}

fn is_immediate_parent_of(parent: &str, child: &str) -> bool {
    let parent = parent.to_case(Case::Snake);
    let child = child.to_case(Case::Snake);
    if child.starts_with(&parent) {
        let child = child.strip_prefix(&parent).unwrap();
        if child.starts_with('/') {
            let child = child.strip_prefix('/').unwrap();
            if !child.contains('/') {
                return true;
            }
        }
    }
    false
}

fn get_children<'a>(sorted_schemas: &'a HashMap<String, Vec<(String, Schema)>>, current: &str) -> Vec<&'a str> {
    sorted_schemas.iter()
        .filter(|(ns, _)| ns.starts_with(&current))
        .flat_map(|(ns, _)| ns.strip_prefix(&current))
        .filter(|ns| !ns.is_empty())
        .map(|ns| {
            if ns.starts_with('/') {
                return ns.strip_prefix('/').unwrap()
            }
            ns
        })
        .filter(|ns| !ns.contains('/'))
        .filter(|ns| *ns != "s")
        .collect()
}

fn get_dir_for_namespace(output: &str, namespace: &str) -> String {
    let dir = format!("{}/models/{}", output, namespace);
    let dir = dir.split('/').collect::<Vec<_>>();
    let dir = dir[..dir.len() - 1].join("/");
    dir
}

fn generate_module_bytes(children: &Vec<&str>) -> Vec<u8> {
    generate_modules(children).to_file_string().unwrap().as_bytes().to_vec()
}

fn generate_modules(children: &Vec<&str>) -> Tokens {
    let children = children.into_iter().map(|child| {
        let child = child.to_case(Case::Snake);
        quote!{
            pub mod $child;
            $['\r']
            $['\r']
        }
    }).collect::<Vec<_>>();
    quote!(
        $['\r']
        $['\r']
        $children
        $['\r']
        $['\r']
    )
}

fn generate_header_bytes() -> Vec<u8> {
    generate_header().to_file_string().unwrap().as_bytes().to_vec()
}

fn generate_header() -> Tokens {
    quote!(
        use std::collections::HashMap;
        use serde::{Serialize, Deserialize};
        use serde_repr::{Serialize_repr, Deserialize_repr};
        use serde_with::{serde_as, DisplayFromStr};

        $['\r']
        $['\r']
    )
}

#[cfg(test)]
mod tests {
    use crate::{render_schema, schemas};

    #[test]
    fn read_spec() {
        let spec = super::read_spec("openapi.json");
        let schemas = &spec.components.schemas;
        println!("schemas: {:?}", schemas.len());
        let schema = schemas.get("Applications.ApplicationScopes").unwrap();

        let mut scope = codegen::Scope::new();
        let (namespace, name) = schemas::reference::resolve("Applications.ApplicationScopes");
        let schema = render_schema(name, schema.clone());
        println!("{:?}", schema.to_file_string().unwrap());
        // println!("{:?}", spec);
    }
}
