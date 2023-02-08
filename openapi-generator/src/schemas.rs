mod enums;
mod objects;
pub mod reference;
mod dictionaries;
mod arrays;

use genco::lang::rust::Tokens;
use genco::quote;
use crate::model;
use crate::model::Schema;

pub enum Type {
    Enum(enums::Enum),
    Object(objects::Object),
    Dictionary(dictionaries::Dictionary),
    Array(arrays::Array),
    Any
}

impl From<Schema> for Type {
    fn from(value: Schema) -> Self {
        return match &value.enum_ {
            Some(_) => Type::Enum(From::from(value)),
            None => {
                match &value.type_ {
                    Some(type_) => match type_.as_str() {
                        "object" => match &value.dictionary_key {
                            Some(_) => Type::Dictionary(From::from(value)),
                            None => match &value.properties {
                                Some(_) => Type::Object(From::from(value)),
                                None => Type::Any
                            }
                        },
                        "array" => Type::Array(From::from(value)),
                        _ => unreachable!()
                    },
                    None => unreachable!(),
                }
            }
        };
        unreachable!()
    }
}

impl Render for Type {
    fn render(&self, name: String) -> Tokens {
        let tokens = match self {
            Type::Enum(e) => e.render(name),
            Type::Object(s) => s.render(name),
            Type::Dictionary(d) => d.render(name),
            Type::Array(a) => a.render(name),
            Type::Any => render_any(name),
        };
        quote! {
            $['\n']
            $['\n']
            $tokens
        }
    }
}

pub trait Render {
    fn render(&self, name: String) -> Tokens;
}

pub fn render_any(name: String) -> Tokens {
    quote! {
        pub type $name = serde_json::Value;
    }
}
