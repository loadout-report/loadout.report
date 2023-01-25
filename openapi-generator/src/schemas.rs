mod enums;
mod objects;
pub mod reference;

use genco::lang::rust::Tokens;
use crate::model;
use crate::model::Schema;

pub enum Type {
    Enum(enums::Enum),
    Object(objects::Object),
}

impl From<Schema> for Type {
    fn from(value: Schema) -> Self {
        if value.enum_.is_some() {
            return Type::Enum(From::from(value));
        }
        if value.type_.is_some() && value.type_.as_ref().unwrap() == "object" {
            return Type::Object(From::from(value));
        }
        unreachable!()
    }
}

impl Render for Type {
    fn render(&self, name: String) -> Tokens {
        match self {
            Type::Enum(e) => e.render(name),
            Type::Object(s) => s.render(name),
        }
    }
}

pub trait Render {
    fn render(&self, name: String) -> Tokens;
}
