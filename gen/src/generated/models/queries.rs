use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};
use serde_with::{serde_as, DisplayFromStr};

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PagedQuery {

    /// No documentation provided.
    pub request_continuation_token: String,
    /// No documentation provided.
    pub current_page: i32,
    /// No documentation provided.
    pub items_per_page: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SearchResult {

    /// If useTotalResults is true, then totalResults represents an accurate count.
/// If False, it does not, and may be estimated/only the size of the current page.
/// Either way, you should probably always only trust hasMore.
/// This is a long-held historical throwback to when we used to do paging with known total results. Those queries toasted our database, and we were left to hastily alter our endpoints and create backward- compatible shims, of which useTotalResults is one.
    pub use_total_results: bool,
    /// No documentation provided.
    pub has_more: bool,
    /// No documentation provided.
    pub total_results: i32,
    /// No documentation provided.
    pub query: crate::generated::models::queries::PagedQuery,
    /// No documentation provided.
    pub replacement_continuation_token: String,
}
