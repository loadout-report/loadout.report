use crate::model::Schema;

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
