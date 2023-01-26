use crate::model::Schema;

pub struct StringType {
    format: Option<String>,
}

impl From<Schema> for StringType {
    fn from(value: Schema) -> Self {
        StringType {
            format: value.format,
        }
    }
}
