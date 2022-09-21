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
pub struct ContentPeriodContentItemPublicContract {
    #[serde(rename = "contentId", skip_serializing_if = "Option::is_none")]
    pub content_id: Option<i64>,
    #[serde(rename = "cType", skip_serializing_if = "Option::is_none")]
    pub c_type: Option<String>,
    #[serde(rename = "cmsPath", skip_serializing_if = "Option::is_none")]
    pub cms_path: Option<String>,
    #[serde(rename = "creationDate", skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    #[serde(rename = "modifyDate", skip_serializing_if = "Option::is_none")]
    pub modify_date: Option<String>,
    #[serde(rename = "allowComments", skip_serializing_if = "Option::is_none")]
    pub allow_comments: Option<bool>,
    #[serde(rename = "hasAgeGate", skip_serializing_if = "Option::is_none")]
    pub has_age_gate: Option<bool>,
    #[serde(rename = "minimumAge", skip_serializing_if = "Option::is_none")]
    pub minimum_age: Option<i32>,
    #[serde(rename = "ratingImagePath", skip_serializing_if = "Option::is_none")]
    pub rating_image_path: Option<String>,
    #[serde(rename = "author", skip_serializing_if = "Option::is_none")]
    pub author: Option<Box<crate::models::UserPeriodGeneralUser>>,
    #[serde(rename = "autoEnglishPropertyFallback", skip_serializing_if = "Option::is_none")]
    pub auto_english_property_fallback: Option<bool>,
    /// Firehose content is really a collection of metadata and \"properties\", which are the potentially-but-not-strictly localizable data that comprises the meat of whatever content is being shown.  As Cole Porter would have crooned, \"Anything Goes\" with Firehose properties. They are most often strings, but they can theoretically be anything. They are JSON encoded, and could be JSON structures, simple strings, numbers etc... The Content Type of the item (cType) will describe the properties, and thus how they ought to be deserialized.
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<::std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "representations", skip_serializing_if = "Option::is_none")]
    pub representations: Option<Vec<crate::models::ContentPeriodContentRepresentation>>,
    /// NOTE: Tags will always be lower case.
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(rename = "commentSummary", skip_serializing_if = "Option::is_none")]
    pub comment_summary: Option<Box<crate::models::ContentPeriodCommentSummary>>,
}

impl ContentPeriodContentItemPublicContract {
    pub fn new() -> ContentPeriodContentItemPublicContract {
        ContentPeriodContentItemPublicContract {
            content_id: None,
            c_type: None,
            cms_path: None,
            creation_date: None,
            modify_date: None,
            allow_comments: None,
            has_age_gate: None,
            minimum_age: None,
            rating_image_path: None,
            author: None,
            auto_english_property_fallback: None,
            properties: None,
            representations: None,
            tags: None,
            comment_summary: None,
        }
    }
}


