use genco::lang::rust::Tokens;
use genco::quote;
use crate::model::Schema;
use crate::schemas::Render;

pub struct Array {
    items: Box<super::PropertyType>,
    mapped_definition: Option<Box<Schema>>,
}

impl From<Schema> for Array {
    fn from(value: Schema) -> Self {
        Array {
            items: Box::new(From::from(*value.items.unwrap())),
            mapped_definition: value.mapped_definition,
        }
    }
}

impl Render for Array {
    fn render(&self, name: String) -> Tokens {
        match self.mapped_definition {
            Some(ref mapped_definition) => {
                quote! {
                    Vec<$(super::render_id_reference(&mapped_definition.ref_.clone().unwrap()))>
                }
            }
            None => quote! {
                Vec<$(self.items.render(name.clone()))>
            },
        }

    }
}
