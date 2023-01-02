use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ContentTypeDescription {
    pub content_type: String,
    pub name: String,
    #[serde(rename = "contentDescription")]
    pub description: String,

    pub preview_image: String,
    pub priority: i32,
    pub reminder: String,
    pub properties: Vec<ContentTypeProperty>,
    pub tag_metadata: Vec<TagMetadataDefinition>,
    pub tag_metadata_items: Vec<TagMetadataItem>,
    pub usage_examples: Vec<String>,
    pub show_in_content_editor: bool,
    pub type_of: String,
    pub bind_identifier_to_property: String,
    pub bound_regex: String,
    pub force_identifier_binding: bool,
    pub allow_comments: bool,
    pub auto_english_property_fallback: bool,
    pub bulk_uploadable: bool,
    pub previews: Vec<Preview>,
    pub suppress_cms_path: bool,
    pub property_sections: Vec<ContentTypePropertySection>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ContentTypeProperty {
    pub name: String,
    pub root_property_name: String,
    pub readable_name: String,
    pub value: String,
    pub property_description: String,
    pub localizable: bool,
    pub fallback: bool,
    pub enabled: bool,
    pub order: i32,
    pub visible: bool,
    pub is_title: bool,
    pub required: bool,
    pub max_length: i32,
    pub max_byte_length: i32,
    pub max_file_size: i32,
    pub regexp: String,
    pub validate_as: String,
    pub rss_attribute: String,
    pub visible_dependency: String,
    pub visible_on: String,
    pub datatype: String,
    pub attributes: HashMap<String, String>,
    pub child_properties: Vec<ContentTypeProperty>,
    pub content_type_allowed: String,
    pub bind_to_property: String,
    pub bound_regex: String,
    pub representation_selection: HashMap<String, String>,
    pub default_values: Vec<ContentTypeDefaultValue>,
    pub is_external_allowed: bool,
    pub property_section: String,
    pub weight: i32,
    pub entitytype: String,
    pub is_combo: bool,
    pub suppress_property: bool,
    pub legal_content_types: Vec<String>,
    pub representation_validation_string: String,
    pub min_width: i32,
    pub max_width: i32,
    pub max_height: i32,
    pub is_video: bool,
    pub is_image: bool,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum ContentPropertyDataType {
    None = 0,
    Plaintext = 1,
    Html = 2,
    Dropdown = 3,
    List = 4,
    Json = 5,
    Content = 6,
    Representation = 7,
    Set = 8,
    File = 9,
    FolderSet = 10,
    Date = 11,
    MultilinePlaintext = 12,
    DestinyContent = 13,
    Color = 14,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ContentTypeDefaultValue {
    pub when_clause: String,
    pub when_value: String,
    pub default_value: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct TagMetadataDefinition {
    pub description: String,
    pub order: i32,
    pub items: Vec<TagMetadataItem>,
    pub datatype: String,
    pub name: String,
    pub is_required: bool,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct TagMetadataItem {
    pub description: String,
    pub tag_text: String,
    pub groups: Vec<String>,
    pub is_default: bool,
    pub name: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Preview {
    pub name: String,
    pub path: String,
    pub item_in_set: bool,
    pub set_tag: String,
    pub set_nesting: i32,
    pub use_set_id: i32,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ContentTypePropertySection {
    pub name: String,
    pub readable_name: String,
    pub collapsed: bool,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ContentItemPublicContract {
    pub content_id: i64,
    #[serde(rename = "cType")]
    pub content_type: String,
    pub cms_path: String,
    pub creation_date: chrono::DateTime<chrono::Utc>,
    pub modify_date: chrono::DateTime<chrono::Utc>,
    pub allow_comments: bool,
    pub has_age_gate: bool,
    pub minimum_age: i32,
    pub rating_image_path: String,
    pub author: crate::user::model::GeneralUser,
    pub auto_english_property_fallback: bool,
    pub properties: HashMap<String, serde_json::Value>,
    pub representations: Vec<ContentRepresentation>,
    pub tags: Vec<String>,
    pub comment_summary: CommentSummary,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ContentRepresentation {
    pub name: String,
    pub path: String,
    pub validation_string: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CommentSummary {
    pub topic_id: i64,
    pub comment_count: i32,
}
