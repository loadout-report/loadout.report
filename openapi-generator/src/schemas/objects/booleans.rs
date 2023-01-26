use crate::model::Schema;

pub struct BooleanType {
}

impl From<Schema> for BooleanType {
    fn from(_: Schema) -> Self {
        BooleanType {
        }
    }
}
