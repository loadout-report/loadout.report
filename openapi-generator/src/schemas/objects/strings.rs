use genco::prelude::rust::Tokens;
use genco::quote;
use crate::model::Schema;
use crate::schemas::Render;

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

impl Render for StringType {
    fn render(&self, _: String) -> Tokens {
        if self.format.is_some() && self.format.as_ref().unwrap() == "date-time" {
            quote!(chrono::DateTime<chrono::Utc>)
        } else {
            quote!(String)
        }
    }
}
