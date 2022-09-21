/*
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * The version of the OpenAPI document: 2.16.0
 * Contact: support@bungie.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ContentPeriodModelsPeriodContentTypeDescription {
    #[serde(rename = "cType", skip_serializing_if = "Option::is_none")]
    pub c_type: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "contentDescription", skip_serializing_if = "Option::is_none")]
    pub content_description: Option<String>,
    #[serde(rename = "previewImage", skip_serializing_if = "Option::is_none")]
    pub preview_image: Option<String>,
    #[serde(rename = "priority", skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[serde(rename = "reminder", skip_serializing_if = "Option::is_none")]
    pub reminder: Option<String>,
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<Vec<crate::models::ContentPeriodModelsPeriodContentTypeProperty>>,
    #[serde(rename = "tagMetadata", skip_serializing_if = "Option::is_none")]
    pub tag_metadata: Option<Vec<crate::models::ContentPeriodModelsPeriodTagMetadataDefinition>>,
    #[serde(rename = "tagMetadataItems", skip_serializing_if = "Option::is_none")]
    pub tag_metadata_items: Option<::std::collections::HashMap<String, crate::models::ContentPeriodModelsPeriodTagMetadataItem>>,
    #[serde(rename = "usageExamples", skip_serializing_if = "Option::is_none")]
    pub usage_examples: Option<Vec<String>>,
    #[serde(rename = "showInContentEditor", skip_serializing_if = "Option::is_none")]
    pub show_in_content_editor: Option<bool>,
    #[serde(rename = "typeOf", skip_serializing_if = "Option::is_none")]
    pub type_of: Option<String>,
    #[serde(rename = "bindIdentifierToProperty", skip_serializing_if = "Option::is_none")]
    pub bind_identifier_to_property: Option<String>,
    #[serde(rename = "boundRegex", skip_serializing_if = "Option::is_none")]
    pub bound_regex: Option<String>,
    #[serde(rename = "forceIdentifierBinding", skip_serializing_if = "Option::is_none")]
    pub force_identifier_binding: Option<bool>,
    #[serde(rename = "allowComments", skip_serializing_if = "Option::is_none")]
    pub allow_comments: Option<bool>,
    #[serde(rename = "autoEnglishPropertyFallback", skip_serializing_if = "Option::is_none")]
    pub auto_english_property_fallback: Option<bool>,
    #[serde(rename = "bulkUploadable", skip_serializing_if = "Option::is_none")]
    pub bulk_uploadable: Option<bool>,
    #[serde(rename = "previews", skip_serializing_if = "Option::is_none")]
    pub previews: Option<Vec<crate::models::ContentPeriodModelsPeriodContentPreview>>,
    #[serde(rename = "suppressCmsPath", skip_serializing_if = "Option::is_none")]
    pub suppress_cms_path: Option<bool>,
    #[serde(rename = "propertySections", skip_serializing_if = "Option::is_none")]
    pub property_sections: Option<Vec<crate::models::ContentPeriodModelsPeriodContentTypePropertySection>>,
}

impl ContentPeriodModelsPeriodContentTypeDescription {
    pub fn new() -> ContentPeriodModelsPeriodContentTypeDescription {
        ContentPeriodModelsPeriodContentTypeDescription {
            c_type: None,
            name: None,
            content_description: None,
            preview_image: None,
            priority: None,
            reminder: None,
            properties: None,
            tag_metadata: None,
            tag_metadata_items: None,
            usage_examples: None,
            show_in_content_editor: None,
            type_of: None,
            bind_identifier_to_property: None,
            bound_regex: None,
            force_identifier_binding: None,
            allow_comments: None,
            auto_english_property_fallback: None,
            bulk_uploadable: None,
            previews: None,
            suppress_cms_path: None,
            property_sections: None,
        }
    }
}


