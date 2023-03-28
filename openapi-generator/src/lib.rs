#![feature(is_some_and)]
extern crate core;

use std::collections::HashMap;
use std::fs::{create_dir_all, File};
use std::io::Write;
use convert_case::{Case, Casing};
use genco::lang::{rust::Tokens};
use genco::{quote};
use crate::model::{Schema, Spec};
use crate::schemas::{Render, Type};
use crate::schemas::reference::resolve;

mod model;
mod schemas;

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
    let header = generate_header_bytes();
    for (namespace, schemas) in &sorted_schemas {
        // dir is all but the last segment of namespace
        let namespace = namespace.clone();

        let dir = get_dir_for_namespace(&output, &namespace);
        let name = namespace.split('/').last().unwrap();
        // "" "" -> models.rs -> find all with empty namespace
        // "" "destiny" -> models/destiny.rs -> find all with destiny namespace
        // "destiny" "definitions" -> models/destiny/definitions.rs
        // top level children
        let children: Vec<_> = get_children(&sorted_schemas, &namespace);
        if children.len() > 0 {
            println!("{} has children: {:?}", namespace, children);
        }
        let modules = generate_module_bytes(&children);
        create_dir_all(&dir).unwrap();
        let mut file = format!("{}/{}.rs", dir, name);
        if file.ends_with("/.rs") {
            file = file.replace("/.rs", ".rs");
        }
        let mut file = File::create(file).unwrap();
        let tokens: Tokens = schemas.into_iter().flat_map(|(name, schema)| {
            render_schema(name.clone(), schema.clone())
        }).collect();
        file.write_all(&modules).unwrap();
        file.write_all(&header).unwrap();
        file.write_all(tokens.to_file_string()
            .unwrap().as_bytes())
            .unwrap();
        let mut segments: Vec<_> = namespace.split('/').collect();
        segments.pop();
        let mut current = String::new();
        for segment in segments {
            current.push_str(segment);
            let dir = get_dir_for_namespace(&output, &current);
            let name = segment.to_case(Case::Snake);
            let mut file = format!("{}/{}.rs", dir, name);
            if file.ends_with("/.rs") {
                file = file.replace("/.rs", ".rs");
            }
            if std::path::Path::new(&file).exists() {
                current.push('/');
                continue;
            }

            // module does not exist, create it
            let children: Vec<_> = get_children(&sorted_schemas, &current);
            if children.len() == 0 {
                current.push('/');
                continue;
            }

            let modules = generate_module_bytes(&children);

            let mut file = File::create(file).unwrap();
            file.write_all(&modules)
                .unwrap();
            current.push('/');
        }
    }
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
        $['\n']
        $children
        $['\n']
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
