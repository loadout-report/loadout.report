use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};
use serde_with::{serde_as, DisplayFromStr};

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContentPreview {

    /// No documentation provided.
    pub item_in_set: bool,
    /// No documentation provided.
    pub name: String,
    /// No documentation provided.
    pub path: String,
    /// No documentation provided.
    pub set_nesting: i32,
    /// No documentation provided.
    pub set_tag: String,
    /// No documentation provided.
    pub use_set_id: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum ContentPropertyDataTypeEnum {
    /// No documentation provided.
    None = 0,
    /// No documentation provided.
    Plaintext = 1,
    /// No documentation provided.
    Html = 2,
    /// No documentation provided.
    Dropdown = 3,
    /// No documentation provided.
    List = 4,
    /// No documentation provided.
    Json = 5,
    /// No documentation provided.
    Content = 6,
    /// No documentation provided.
    Representation = 7,
    /// No documentation provided.
    Set = 8,
    /// No documentation provided.
    File = 9,
    /// No documentation provided.
    FolderSet = 10,
    /// No documentation provided.
    Date = 11,
    /// No documentation provided.
    MultilinePlaintext = 12,
    /// No documentation provided.
    DestinyContent = 13,
    /// No documentation provided.
    Color = 14,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContentTypeDefaultValue {

    /// No documentation provided.
    pub default_value: String,
    /// No documentation provided.
    pub when_clause: String,
    /// No documentation provided.
    pub when_value: String,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContentTypeDescription {

    /// No documentation provided.
    pub allow_comments: bool,
    /// No documentation provided.
    pub auto_english_property_fallback: bool,
    /// No documentation provided.
    pub bind_identifier_to_property: String,
    /// No documentation provided.
    pub bound_regex: String,
    /// No documentation provided.
    pub bulk_uploadable: bool,
    /// No documentation provided.
    pub c_type: String,
    /// No documentation provided.
    pub content_description: String,
    /// No documentation provided.
    pub force_identifier_binding: bool,
    /// No documentation provided.
    pub name: String,
    /// No documentation provided.
    pub preview_image: String,
    /// No documentation provided.
    pub previews: Vec<crate::generated::models::content::models::ContentPreview>,
    /// No documentation provided.
    pub priority: i32,
    /// No documentation provided.
    pub properties: Vec<crate::generated::models::content::models::ContentTypeProperty>,
    /// No documentation provided.
    pub property_sections: Vec<crate::generated::models::content::models::ContentTypePropertySection>,
    /// No documentation provided.
    pub reminder: String,
    /// No documentation provided.
    pub show_in_content_editor: bool,
    /// No documentation provided.
    pub suppress_cms_path: bool,
    /// No documentation provided.
    pub tag_metadata: Vec<crate::generated::models::content::models::TagMetadataDefinition>,
    /// No documentation provided.
    pub tag_metadata_items: i32,
    /// No documentation provided.
    pub type_of: String,
    /// No documentation provided.
    pub usage_examples: Vec<String>,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContentTypeProperty {

    /// No documentation provided.
    pub attributes: i32,
    /// No documentation provided.
    pub bind_to_property: String,
    /// No documentation provided.
    pub bound_regex: String,
    /// No documentation provided.
    pub child_properties: Vec<crate::generated::models::content::models::ContentTypeProperty>,
    /// No documentation provided.
    pub content_type_allowed: String,
    /// No documentation provided.
    pub datatype: crate::generated::models::content::models::ContentPropertyDataTypeEnum,
    /// No documentation provided.
    pub default_values: Vec<crate::generated::models::content::models::ContentTypeDefaultValue>,
    /// No documentation provided.
    pub enabled: bool,
    /// No documentation provided.
    pub entitytype: String,
    /// No documentation provided.
    pub fallback: bool,
    /// No documentation provided.
    pub is_combo: bool,
    /// No documentation provided.
    pub is_external_allowed: bool,
    /// No documentation provided.
    pub is_image: bool,
    /// No documentation provided.
    pub is_title: bool,
    /// No documentation provided.
    pub is_video: bool,
    /// No documentation provided.
    pub legal_content_types: Vec<String>,
    /// No documentation provided.
    pub localizable: bool,
    /// No documentation provided.
    pub max_byte_length: i32,
    /// No documentation provided.
    pub max_file_size: i32,
    /// No documentation provided.
    pub max_height: i32,
    /// No documentation provided.
    pub max_length: i32,
    /// No documentation provided.
    pub max_width: i32,
    /// No documentation provided.
    pub min_height: i32,
    /// No documentation provided.
    pub min_width: i32,
    /// No documentation provided.
    pub name: String,
    /// No documentation provided.
    pub order: i32,
    /// No documentation provided.
    pub property_description: String,
    /// No documentation provided.
    pub property_section: String,
    /// No documentation provided.
    pub readable_name: String,
    /// No documentation provided.
    pub regexp: String,
    /// No documentation provided.
    pub representation_selection: i32,
    /// No documentation provided.
    pub representation_validation_string: String,
    /// No documentation provided.
    pub required: bool,
    /// No documentation provided.
    pub root_property_name: String,
    /// No documentation provided.
    pub rss_attribute: String,
    /// No documentation provided.
    pub suppress_property: bool,
    /// No documentation provided.
    pub validate_as: String,
    /// No documentation provided.
    pub value: String,
    /// No documentation provided.
    pub visible: bool,
    /// No documentation provided.
    pub visible_dependency: String,
    /// No documentation provided.
    pub visible_on: String,
    /// No documentation provided.
    pub weight: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContentTypePropertySection {

    /// No documentation provided.
    pub collapsed: bool,
    /// No documentation provided.
    pub name: String,
    /// No documentation provided.
    pub readable_name: String,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TagMetadataDefinition {

    /// No documentation provided.
    pub datatype: String,
    /// No documentation provided.
    pub description: String,
    /// No documentation provided.
    pub is_required: bool,
    /// No documentation provided.
    pub items: Vec<crate::generated::models::content::models::TagMetadataItem>,
    /// No documentation provided.
    pub name: String,
    /// No documentation provided.
    pub order: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TagMetadataItem {

    /// No documentation provided.
    pub description: String,
    /// No documentation provided.
    pub groups: Vec<String>,
    /// No documentation provided.
    pub is_default: bool,
    /// No documentation provided.
    pub name: String,
    /// No documentation provided.
    pub tag_text: String,
}
