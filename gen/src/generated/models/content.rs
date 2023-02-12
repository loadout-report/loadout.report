pub mod models;

use serde::{Serialize, Deserialize};


/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NewsArticleRssResponse {

    /// No documentation provided.
    pub next_pagination_token: Option<i32>,
    /// No documentation provided.
    pub category_filter: String,
    /// No documentation provided.
    pub current_pagination_token: i32,
    /// No documentation provided.
    pub result_count_this_page: i32,
    /// No documentation provided.
    pub news_articles: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommentSummary {

    /// No documentation provided.
    #[serde(with = "crate::unfuck_js::stringified_numbers")]
    pub topic_id: i64,
    /// No documentation provided.
    pub comment_count: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NewsArticleRssItem {

    /// No documentation provided.
    pub optional_mobile_image_path: String,
    /// No documentation provided.
    pub pub_date: chrono::DateTime<chrono::Utc>,
    /// No documentation provided.
    pub link: String,
    /// No documentation provided.
    pub image_path: String,
    /// No documentation provided.
    pub html_content: String,
    /// No documentation provided.
    pub description: String,
    /// No documentation provided.
    pub title: String,
    /// No documentation provided.
    pub unique_identifier: String,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContentItemPublicContract {

    /// Firehose content is really a collection of metadata and "properties", which are the potentially-but-not-strictly localizable data that comprises the meat of whatever content is being shown.
/// As Cole Porter would have crooned, "Anything Goes" with Firehose properties. They are most often strings, but they can theoretically be anything. They are JSON encoded, and could be JSON structures, simple strings, numbers etc... The Content Type of the item (cType) will describe the properties, and thus how they ought to be deserialized.
    pub properties: i32,
    /// No documentation provided.
    pub comment_summary: i32,
    /// No documentation provided.
    #[serde(with = "crate::unfuck_js::stringified_numbers")]
    pub content_id: i64,
    /// No documentation provided.
    pub allow_comments: bool,
    /// NOTE: Tags will always be lower case.
    pub tags: i32,
    /// No documentation provided.
    pub creation_date: chrono::DateTime<chrono::Utc>,
    /// No documentation provided.
    pub author: i32,
    /// No documentation provided.
    pub cms_path: String,
    /// No documentation provided.
    pub minimum_age: i32,
    /// No documentation provided.
    pub auto_english_property_fallback: bool,
    /// No documentation provided.
    pub c_type: String,
    /// No documentation provided.
    pub modify_date: chrono::DateTime<chrono::Utc>,
    /// No documentation provided.
    pub rating_image_path: String,
    /// No documentation provided.
    pub representations: i32,
    /// No documentation provided.
    pub has_age_gate: bool,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContentRepresentation {

    /// No documentation provided.
    pub path: String,
    /// No documentation provided.
    pub validation_string: String,
    /// No documentation provided.
    pub name: String,
}
