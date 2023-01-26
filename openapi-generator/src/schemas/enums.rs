use genco::lang::rust::Tokens;
use genco::quote;
use crate::model::{EnumValue, Schema};
use crate::schemas::Render;
use crate::format_description;

pub struct Enum {
    format: String,
    description: Option<String>,
    enum_values: Vec<EnumValue>,
    is_bitmask: bool,
}

impl From<Schema> for Enum {
    fn from(value: Schema) -> Self {
        Enum {
            format: value.format.unwrap(),
            description: value.description,
            enum_values: value.enum_values.unwrap(),
            is_bitmask: value.enum_is_bitmask,
        }
    }
}

impl Render for Enum {
    fn render(&self, name: String) -> Tokens {
        let variants: Tokens = self.enum_values.iter()
            .flat_map(|v| quote! {
                        $(format_description(v.description.clone()))
                        $['\r']
                        $(v.identifier.clone()) = $(v.numeric_value.parse::<i32>().unwrap()),
                        $['\r']
                    })
            .collect();

        quote! {
            $['\r']
            $(format_description(self.description.clone()))
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr)]
            pub enum $name {
                $variants
            }
        }
    }
}
