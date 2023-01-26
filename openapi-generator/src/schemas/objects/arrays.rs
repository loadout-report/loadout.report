use crate::model::Schema;

pub struct Array {
    items: Box<super::PropertyType>,
}

impl From<Schema> for Array {
    fn from(value: Schema) -> Self {
        Array {
            items: Box::new(From::from(*value.items.unwrap())),
        }
    }
}
