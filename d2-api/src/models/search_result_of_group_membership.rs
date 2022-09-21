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
pub struct SearchResultOfGroupMembership {
    #[serde(rename = "results", skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<crate::models::GroupsV2PeriodGroupMembership>>,
    #[serde(rename = "totalResults", skip_serializing_if = "Option::is_none")]
    pub total_results: Option<i32>,
    #[serde(rename = "hasMore", skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(rename = "query", skip_serializing_if = "Option::is_none")]
    pub query: Option<Box<crate::models::QueriesPeriodPagedQuery>>,
    #[serde(rename = "replacementContinuationToken", skip_serializing_if = "Option::is_none")]
    pub replacement_continuation_token: Option<String>,
    /// If useTotalResults is true, then totalResults represents an accurate count.  If False, it does not, and may be estimated/only the size of the current page.  Either way, you should probably always only trust hasMore.  This is a long-held historical throwback to when we used to do paging with known total results. Those queries toasted our database, and we were left to hastily alter our endpoints and create backward- compatible shims, of which useTotalResults is one.
    #[serde(rename = "useTotalResults", skip_serializing_if = "Option::is_none")]
    pub use_total_results: Option<bool>,
}

impl SearchResultOfGroupMembership {
    pub fn new() -> SearchResultOfGroupMembership {
        SearchResultOfGroupMembership {
            results: None,
            total_results: None,
            has_more: None,
            query: None,
            replacement_continuation_token: None,
            use_total_results: None,
        }
    }
}


