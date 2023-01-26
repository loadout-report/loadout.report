use genco::lang::rust::Tokens;
use genco::quote;
use crate::model::Schema;
use crate::schemas::objects::PropertyType;
use crate::schemas::Render;

pub struct Dictionary {
    additional_properties: PropertyType,
    key: PropertyType,
}

impl From<Schema> for Dictionary {
    fn from(value: Schema) -> Self {
        Dictionary {
            additional_properties: From::from(*value.additional_properties.unwrap()),
            key: From::from(*value.dictionary_key.unwrap()),
        }
    }
}

impl Render for Dictionary {
    fn render(&self, name: String) -> Tokens {
        quote! {
            // todo: implement $name
            pub type $name = HashMap<String, String>;
        }
    }
}
