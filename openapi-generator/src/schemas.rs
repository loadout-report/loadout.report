
pub mod enums {
    use genco::lang::rust::Tokens;

    pub struct Enum {

    }

    pub fn is_enum(schema: &super::super::model::Schema) -> bool {
        schema.enum_.is_some()
    }

    pub fn parse_enum(schema: &super::super::model::Schema) -> Enum {
        todo!()
    }

    pub fn render_enum(enum_: Enum) -> Tokens {
        todo!()
    }

}

pub mod objects {

    pub fn is_object(schema: &super::super::model::Schema) -> bool {
        schema.type_.is_some() && schema.type_.as_ref().unwrap() == "object"
    }

}
