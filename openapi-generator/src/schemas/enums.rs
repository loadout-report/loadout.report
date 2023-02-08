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
                        $(if v.description.clone().map(|s| s.to_lowercase().contains("deprecated")).unwrap_or_default() {
                            #[deprecated]
                        })
                        $(v.identifier.clone()) = $(v.numeric_value.parse::<i32>().unwrap()),
                        $['\r']
                    })
            .collect();

        quote! {
            $['\r']
            $['\r']
            $(format_description(self.description.clone()))
            $(if self.is_bitmask {
                $(format!("/// todo: bitmask"))
            })
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr)]
            #[repr($(to_rust_repr(&self.format)))]
            pub enum $name {
                $variants
            }
        }
    }
}

fn to_rust_repr(format: &str) -> String {
    match format {
        "int32" => "u32",
        "int64" => "u64",
        "byte" => "u8",
        _ => unimplemented!("Unknown format: {}", format),
    }.to_string()
}
