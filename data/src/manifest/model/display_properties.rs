use serde::Deserialize;

#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DisplayProperties {
    description: String,
    name: String,
    icon: String,
    has_icon: bool,
}