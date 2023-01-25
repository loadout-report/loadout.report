mod enums;
mod arrays;
mod strings;
mod numbers;
mod booleans;

use std::collections::HashMap;
use genco::lang::rust::Tokens;
use crate::model::Schema;
use crate::schemas::{Render};

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
}

impl From<Schema> for Object {
    fn from(value: Schema) -> Self {
        unimplemented!()
    }
}

impl Render for Object {
    fn render(&self, name: String) -> Tokens {
        unimplemented!()
    }
}

pub fn is_object(schema: &Schema) -> bool {
    schema.type_.is_some() && schema.type_.as_ref().unwrap() == "object"
}
