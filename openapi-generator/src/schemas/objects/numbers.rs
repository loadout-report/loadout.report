use genco::lang::rust::Tokens;
use genco::quote;
use crate::model::Schema;
use crate::schemas::Render;

pub struct NumberType {
    format: String,
}

impl From<Schema> for NumberType {
    fn from(value: Schema) -> Self {
        NumberType {
            format: value.format.unwrap(),
        }
    }
}

impl Render for NumberType {
    fn render(&self, name: String) -> Tokens {
        if name.eq("hash") {
            return quote! {
                crate::id::Id<$(self.openapi_nonsense_to_rust_type()), Self>
            }
        }
        quote! {
            $(self.openapi_nonsense_to_rust_type())
        }
    }
}

impl NumberType {
    pub fn is_fucked(&self) -> bool {
        let format: &str = &self.format;
        match format {
            "int64" => true,
            _ => false,
        }
    }

    pub fn openapi_nonsense_to_rust_type(&self) -> &str {
        openapi_nonsense_to_rust_type(&self.format)
    }
}

pub fn openapi_nonsense_to_rust_type(format: &str) -> &str {
    match format {
        "float" => "f32",
        "double" => "f64",
        "int32" => "i32",
        "int64" => "i64",
        "uint32" => "u32",
        "int16" => "i16",
        "byte" => "u8",
        _ => unimplemented!("Unknown format: {}", format),
    }
}
