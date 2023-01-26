extern crate core;

use std::collections::HashMap;
use std::fs::{create_dir_all, File};
use std::io::Write;
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
        .map(|l| format!("/// {}", l)).collect::<Vec<_>>().join("\r")
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
        sorted_schemas.entry(namespace)
            .or_insert_with(|| Vec::new())
            .push((name, schema));
    }
    for (namespace, schemas) in sorted_schemas {
        // dir is all but the last segment of namespace
        let path = format!("{}/models/{}", output, namespace);
        let dir = path.split('/').collect::<Vec<_>>();
        let dir = dir[..dir.len() - 1].join("/");
        let name = namespace.split('/').last().unwrap();
        create_dir_all(&dir).unwrap();
        let file = format!("{}/{}.rs", dir, name);
        let mut file = File::create(file).unwrap();
        let tokens: Tokens = schemas.into_iter().flat_map(|(name, schema)| {
            render_schema(name, schema)
        }).collect();
        file.write_all(tokens.to_file_string()
            .unwrap().as_bytes())
            .unwrap();
    }
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
