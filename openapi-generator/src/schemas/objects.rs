mod enums;
mod arrays;
mod strings;
mod numbers;
mod booleans;

use std::collections::HashMap;
use genco::lang::rust::Tokens;
use genco::quote;
use crate::model::Schema;
use crate::schemas::{Render};
use crate::format_description;

pub struct Property {
    name: String,
    description: Option<String>,
    type_: PropertyType,
    // for references, this is currently always true (ugh)
    nullable: bool,

}

pub enum PropertyType {
    Array(arrays::Array),
    Reference(String),
    String(strings::StringType),
    Number(numbers::NumberType),
    Enum(enums::Enum),
    Boolean(booleans::BooleanType),
}

pub struct Object {
    properties: Vec<Property>,
    description: Option<String>,
}

impl From<Schema> for Object {
    fn from(value: Schema) -> Self {
        let properties = value.properties.unwrap().into_iter()
            .map(|(name, schema)| {
                let description = schema.description.clone();
                let nullable = schema.nullable.unwrap_or(false);
                let type_ = match schema.type_.as_ref().unwrap().as_str() {
                    "array" => PropertyType::Array(From::from(schema)),
                    "string" => PropertyType::String(From::from(schema)),
                    "number" => PropertyType::Number(From::from(schema)),
                    "integer" => PropertyType::Number(From::from(schema)),
                    "boolean" => PropertyType::Boolean(From::from(schema)),
                    "object" => PropertyType::Reference(schema.ref_.unwrap()),
                    _ => unreachable!(),
                };
                Property {
                    name,
                    description,
                    type_,
                    nullable,
                }
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
            }
        }
    }
}

pub fn is_object(schema: &Schema) -> bool {
    schema.type_.is_some() && schema.type_.as_ref().unwrap() == "object"
}
