use genco::lang::rust::Tokens;
use genco::quote;
use crate::model::Schema;
use crate::schemas::objects::PropertyType;
use crate::schemas::Render;

pub struct Dictionary {
    additional_properties: Box<PropertyType>,
    key: Box<PropertyType>,
}

impl From<Schema> for Dictionary {
    fn from(value: Schema) -> Self {
        Dictionary {
            additional_properties: Box::new(From::from(*value.additional_properties.unwrap())),
            key: Box::new(From::from(*value.dictionary_key.unwrap())),
        }
    }
}

impl Render for Dictionary {
    fn render(&self, name: String) -> Tokens {
        quote!(HashMap<$(self.key.render(name.to_owned())), $(self.additional_properties.render(name.to_owned()))>)
    }
}
