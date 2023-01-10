use std::fs::File;
use genco::lang::Rust;
use genco::{quote, Tokens};
use crate::model::{Schema, Spec};

mod model;
mod schemas;

fn read_spec(path: &str) -> Spec {
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

fn format_description(description: Option<String>) -> String {
    description
        .unwrap_or_else(|| String::from("No documentation provided.")).lines()
        .map(|l| format!("/// {}", l)).collect::<Vec<_>>().join("\r")
}

fn render_schema(
    namespace: Vec<String>,
    name: String,
    schema: &model::Schema
) -> Tokens<Rust> {
    println!("{:#?}", schema);
    if schemas::enums::is_enum(schema) {
        let parsed = schemas::enums::parse_enum(schema);
        return schemas::enums::render_enum(parsed);
    }
    if let Some(type_) = &schema.type_ {
        if type_ == "integer" {
            if let Some(enum_values) = &schema.enum_ {
                // new enum
                let variants = &schema.enum_values.as_ref().unwrap();
                let variants: Tokens<Rust> = variants.iter()
                    .flat_map(|v| quote! {
                        $(format_description(v.description.clone()))
                        $['\r']
                        $(v.identifier.clone()) = $(v.numeric_value.parse::<i32>().unwrap()),
                        $['\r']
                    })
                    .collect();

                let tokens: Tokens<Rust> = quote! {
                    $(format_description(schema.description.clone()))
                    $(if schema.enum_is_bitmask {
                        #[bitflags]
                        #[repr(u32)]
                        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr)]
                    } else {
                        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
                    })
                    pub enum $name {
                        $variants
                    }
                };
                return tokens;

            }
        }
    } else {
        // this should be a ref
    }
    unreachable!()
}


#[cfg(test)]
mod tests {
    use crate::render_schema;

    #[test]
    fn read_spec() {
        let spec = super::read_spec("openapi.json");
        let schemas = &spec.components.schemas;
        println!("schemas: {:?}", schemas.len());
        let schema = schemas.get("Applications.ApplicationScopes").unwrap();

        let mut scope = codegen::Scope::new();
        let (namespace, name) = super::calculate_name("Applications.ApplicationScopes");
        let schema = render_schema(namespace, name, schema);
        println!("{}", schema);
        // println!("{:?}", spec);
    }
}
