use genco::lang::rust::Tokens;
use genco::quote;
use crate::model::Schema;
use crate::schemas::reference::resolve;
use crate::schemas::Render;

pub struct Enum {
    is_bitmask: bool,
    reference: String,
}

impl From<Schema> for Enum {
    fn from(value: Schema) -> Self {
        let is_bitmask = value.enum_is_bitmask;
        let reference = value.enum_reference.unwrap().ref_.unwrap();
        Enum {
            is_bitmask,
            reference,
        }
    }
}

impl Render for Enum {
    fn render(&self, name: String) -> Tokens {
        // todo: bitmask
        quote! {
            $(enum_reference_to_rust_type(&self.reference))
        }
    }
}

pub fn enum_reference_to_rust_type(reference: &str) -> String {
    println!("enum reference: {}", reference);
    let (namespace, reference) = resolve(reference);
    let namespace = namespace.replace('/', "::");
    let namespace = namespace.trim_start_matches("::");
    let reference = format!("crate::generated::models::{}::{}", namespace, reference);
    let reference = reference.replace("::::", "::");
    println!("  to reference: {}", reference);
    reference
}
