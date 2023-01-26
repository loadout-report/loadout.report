use crate::model::Schema;
use crate::schemas::objects::PropertyType;

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
