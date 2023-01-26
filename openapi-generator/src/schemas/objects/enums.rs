use crate::model::Schema;

pub struct Enum {
    is_bitmask: bool,
    reference: String,
}

impl From<Schema> for Enum {
    fn from(value: Schema) -> Self {
        let is_bitmask = value.enum_is_bitmask;
        let reference = value.enum_reference.unwrap().ref_.unwrap();
        Enum {
            is_bitmask,
            reference,
        }
    }
}
