mod enums;
mod arrays;
mod strings;
mod numbers;
mod booleans;
mod dictionary;

use genco::prelude::*;
use std::collections::HashMap;
use convert_case::{Case, Casing};
use genco::lang::rust::Tokens;
use genco::quote;
use crate::model::Schema;
use crate::schemas::{Render};
use crate::{format_description, sorted_by};
use crate::schemas::reference::resolve;

pub struct Property {
    // name: String,
    description: Option<String>,
    type_: PropertyType,
    // for references, this is currently always true (ugh)
    nullable: bool,

}

impl Render for Property {
    fn render(&self, name: String) -> Tokens {
        let name_tokens = if name == "type" {
            quote!(r#type)
        } else {
            quote!($(name.to_case(Case::Snake)))
        };
        let signature = if self.nullable {
            quote!(Option<$(self.type_.render(name.clone()))>)
        } else {
            quote!($(self.type_.render(name.clone())))
        };
        match &self.type_ {
            PropertyType::Number(n) => quote! {
                $(format_description(self.description.clone()))
                $(if n.is_fucked() {
                    $(if self.nullable {
                        #[serde(with = "crate::unfuck_js::nullable_stringified_numbers")]
                    } else {
                        #[serde(with = "crate::unfuck_js::stringified_numbers")]
                    })
                    $['\r']
                })
                pub $name_tokens: $signature,
                $['\r']
            },
            _ => quote! {
                $(format_description(self.description.clone()))
                pub $name_tokens: $signature,
                $['\r']
            }
        }
    }
}

impl From<Schema> for Property {
    fn from(value: Schema) -> Self {
        let description = value.description.clone();
        let nullable = value.nullable.unwrap_or(false);
        let type_ = From::from(value);
        Property {
            description,
            type_,
            nullable,
        }
    }
}

pub enum PropertyType {
    Array(arrays::Array),
    Reference(String),
    String(strings::StringType),
    Number(numbers::NumberType),
    Enum(enums::Enum),
    Boolean(booleans::BooleanType),
    Dictionary(dictionary::Dictionary),
    IdReference(String, String),
    Any, // fuck
}

impl Render for PropertyType {
    fn render(&self, name: String) -> Tokens {
        match self {
            PropertyType::Array(a) => a.render(name), // todo: array support
            PropertyType::Reference(r) => render_reference(r),
            PropertyType::String(s) => s.render(name),
            PropertyType::Number(n) => n.render(name),
            PropertyType::Enum(e) => e.render(name),
            PropertyType::Boolean(_) => quote!(bool),
            PropertyType::Dictionary(d) => d.render(name), // todo: dictionary support
            PropertyType::IdReference(r, format) => render_id_reference(r, format), // todo: id reference support
            PropertyType::Any => quote!(serde_json::Value), // ugh
        }
    }
}

impl From<Schema> for PropertyType {
    fn from(value: Schema) -> Self {
        match &value.type_ {
            Some(type_) => match type_.as_str() {
                "array" => PropertyType::Array(From::from(value)),
                "string" => PropertyType::String(From::from(value)),
                "number" => PropertyType::Number(From::from(value)),
                "integer" => match value.enum_reference {
                    Some(_) => PropertyType::Enum(From::from(value)),
                    None => match value.mapped_definition {
                        Some(d) =>  {
                            PropertyType::IdReference(d.ref_.clone().unwrap(), value.format.clone().unwrap())
                        },
                        None => PropertyType::Number(From::from(value)),
                    },
                },
                "boolean" => PropertyType::Boolean(From::from(value)),
                "object" => match value.dictionary_key {
                    Some(_) => PropertyType::Dictionary(From::from(value)),
                    None => match &value.all_of {
                        Some(all_of) => {
                            if all_of.len() > 1 {
                                panic!("allOf with more than one schema is not supported");
                            }
                            let schema = all_of.first().unwrap();
                            PropertyType::Reference(schema.ref_.clone().unwrap())
                        }
                        None => match &value.ref_ {
                            Some(ref_) => PropertyType::Reference(ref_.clone()),
                            None => PropertyType::Any,
                        }
                    }
                },
                _ => unreachable!(),
            },
            None => PropertyType::Reference(value.ref_.unwrap()),
        }
    }
}

pub struct Object {
    properties: HashMap<String, Property>,
    description: Option<String>,
}

impl From<Schema> for Object {
    fn from(value: Schema) -> Self {
        let properties: HashMap<String, Property> = value.properties.unwrap().into_iter()
            .map(|(name, schema)| {
                (name, From::from(schema))
            })
            .collect();

        Object {
            properties,
            description: value.description,
        }
    }
}

impl Render for Object {
    fn render(&self, name: String) -> Tokens {
        quote! {
            $(format_description(self.description.clone()))
            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
            pub struct $name {
                // todo: properties
                $(render_properties(&self.properties))
            }
        }
    }
}

pub fn is_object(schema: &Schema) -> bool {
    schema.type_.is_some() && schema.type_.as_ref().unwrap() == "object"
}

pub fn render_properties(properties: &HashMap<String, Property>) -> Tokens {
    sorted_by(properties.into_iter().collect(), |(a, _), (b, _)| a.cmp(b))
        .into_iter()
        .flat_map(|(name, property)| property.render(name.clone()))
        .collect()
}

pub fn render_reference(reference: &str) -> Tokens {
    println!("rendering reference: {}", reference);
    let (namespace, reference) = resolve(reference);
    let namespace = namespace.replace('/', "::");
    let reference = format!("{}::{}", namespace, reference);
    let reference = reference.replace("::::", "::");
    let reference = reference.trim_start_matches("::");
    println!("resulting reference: {}", reference);
    quote!(crate::generated::models::$reference)
}

pub fn render_id_reference(reference: &str, format: &str) -> Tokens {
    println!("rendering id reference: {}", reference);
    let (namespace, reference) = resolve(reference);
    let namespace = namespace.replace('/', "::");
    let reference = format!("{}::{}", namespace, reference);
    let reference = reference.replace("::::", "::");
    let reference = reference.trim_start_matches("::");
    println!("resulting id reference: {}", reference);
    quote!(crate::id::Id<$(numbers::openapi_nonsense_to_rust_type(format)), crate::generated::models::$reference>)
}
