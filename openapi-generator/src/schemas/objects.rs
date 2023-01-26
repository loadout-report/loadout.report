mod enums;
mod arrays;
mod strings;
mod numbers;
mod booleans;
mod dictionary;

use std::collections::HashMap;
use genco::lang::rust::Tokens;
use genco::quote;
use crate::model::Schema;
use crate::schemas::{Render};
use crate::format_description;

pub struct Property {
    // name: String,
    description: Option<String>,
    type_: PropertyType,
    // for references, this is currently always true (ugh)
    nullable: bool,

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
    Any, // fuck
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
                    None => PropertyType::Number(From::from(value)),
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
            }
        }
    }
}

pub fn is_object(schema: &Schema) -> bool {
    schema.type_.is_some() && schema.type_.as_ref().unwrap() == "object"
}
