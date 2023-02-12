use genco::lang::rust::Tokens;
use genco::quote;
use crate::model::Schema;
use crate::schemas::objects::PropertyType;
use crate::schemas::Render;

pub struct Array {
    items: PropertyType
}

impl From<Schema> for Array {
    fn from(value: Schema) -> Self {
        Array {
            items: From::from(*value.items.unwrap())
        }
    }
}

impl Render for Array {
    fn render(&self, name: String) -> Tokens {
        quote! {
            // todo: properly resolve items
            // type $name = Vec<()>;
        }
    }
}
