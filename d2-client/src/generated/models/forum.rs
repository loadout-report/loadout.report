use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};
use serde_with::{serde_as, DisplayFromStr};

/// No documentation provided.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum CommunityContentSortMode {
    /// No documentation provided.
    Trending = 0,
    /// No documentation provided.
    Latest = 1,
    /// No documentation provided.
    HighestRated = 2,
}

/// No documentation provided.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum ForumMediaType {
    /// No documentation provided.
    None = 0,
    /// No documentation provided.
    Image = 1,
    /// No documentation provided.
    Video = 2,
    /// No documentation provided.
    Youtube = 3,
}

/// No documentation provided.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum ForumPostPopularity {
    /// No documentation provided.
    Empty = 0,
    /// No documentation provided.
    Default = 1,
    /// No documentation provided.
    Discussed = 2,
    /// No documentation provided.
    CoolStory = 3,
    /// No documentation provided.
    HeatingUp = 4,
    /// No documentation provided.
    Hot = 5,
}

/// No documentation provided.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum ForumPostSortEnum {
    /// No documentation provided.
    Default = 0,
    /// No documentation provided.
    OldestFirst = 1,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ForumRecruitmentDetail {

    /// No documentation provided.
    pub fireteam: Vec<crate::generated::models::user::GeneralUser>,
    /// No documentation provided.
    pub approved: bool,
    /// No documentation provided.
    #[serde(with = "crate::unfuck_js::nullable_stringified_numbers")]
    pub conversation_id: Option<i64>,
    /// No documentation provided.
    pub intensity: crate::generated::models::forum::ForumRecruitmentIntensityLabel,
    /// No documentation provided.
    pub kicked_player_ids: Vec<i64>,
    /// No documentation provided.
    pub microphone_required: bool,
    /// No documentation provided.
    pub player_slots_remaining: i32,
    /// No documentation provided.
    pub player_slots_total: i32,
    /// No documentation provided.
    pub tone: crate::generated::models::forum::ForumRecruitmentToneLabel,
    /// No documentation provided.
    #[serde(with = "crate::unfuck_js::stringified_numbers")]
    pub topic_id: i64,
}

/// No documentation provided.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ForumRecruitmentIntensityLabel {
    /// No documentation provided.
    None = 0,
    /// No documentation provided.
    Casual = 1,
    /// No documentation provided.
    Professional = 2,
}

/// No documentation provided.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ForumRecruitmentToneLabel {
    /// No documentation provided.
    None = 0,
    /// No documentation provided.
    FamilyFriendly = 1,
    /// No documentation provided.
    Rowdy = 2,
}

/// No documentation provided.
/// todo: bitmask
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum ForumTopicsCategoryFiltersEnum {
    /// No documentation provided.
    None = 0,
    /// No documentation provided.
    Links = 1,
    /// No documentation provided.
    Questions = 2,
    /// No documentation provided.
    AnsweredQuestions = 4,
    /// No documentation provided.
    Media = 8,
    /// No documentation provided.
    TextOnly = 16,
    /// No documentation provided.
    Announcement = 32,
    /// No documentation provided.
    BungieOfficial = 64,
    /// No documentation provided.
    Polls = 128,
}

/// No documentation provided.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum ForumTopicsQuickDateEnum {
    /// No documentation provided.
    All = 0,
    /// No documentation provided.
    LastYear = 1,
    /// No documentation provided.
    LastMonth = 2,
    /// No documentation provided.
    LastWeek = 3,
    /// No documentation provided.
    LastDay = 4,
}

/// No documentation provided.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ForumTopicsSortEnum {
    /// No documentation provided.
    Default = 0,
    /// No documentation provided.
    LastReplied = 1,
    /// No documentation provided.
    MostReplied = 2,
    /// No documentation provided.
    Popularity = 3,
    /// No documentation provided.
    Controversiality = 4,
    /// No documentation provided.
    Liked = 5,
    /// No documentation provided.
    HighestRated = 6,
    /// No documentation provided.
    MostUpvoted = 7,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PollResponse {

    /// No documentation provided.
    pub results: Vec<crate::generated::models::forum::PollResult>,
    /// No documentation provided.
    #[serde(with = "crate::unfuck_js::stringified_numbers")]
    pub topic_id: i64,
    /// No documentation provided.
    pub total_votes: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PollResult {

    /// No documentation provided.
    pub answer_slot: i32,
    /// No documentation provided.
    pub answer_text: String,
    /// No documentation provided.
    pub last_vote_date: chrono::DateTime<chrono::Utc>,
    /// No documentation provided.
    pub requesting_user_voted: bool,
    /// No documentation provided.
    pub votes: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostResponse {

    /// No documentation provided.
    pub is_pinned: bool,
    /// No documentation provided.
    pub ignore_status: crate::generated::models::ignores::IgnoreResponse,
    /// No documentation provided.
    pub is_active: bool,
    /// No documentation provided.
    pub is_announcement: bool,
    /// No documentation provided.
    pub last_reply_timestamp: chrono::DateTime<chrono::Utc>,
    /// No documentation provided.
    #[serde(with = "crate::unfuck_js::stringified_numbers")]
    pub latest_reply_author_id: i64,
    /// No documentation provided.
    #[serde(with = "crate::unfuck_js::stringified_numbers")]
    pub latest_reply_post_id: i64,
    /// No documentation provided.
    pub locale: String,
    /// No documentation provided.
    pub popularity: crate::generated::models::forum::ForumPostPopularity,
    /// No documentation provided.
    pub thumbnail: String,
    /// No documentation provided.
    pub url_media_type: crate::generated::models::forum::ForumMediaType,
    /// No documentation provided.
    pub user_has_muted_post: bool,
    /// No documentation provided.
    pub user_has_rated: bool,
    /// No documentation provided.
    pub user_rating: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostSearchResponse {

    /// No documentation provided.
    pub authors: Vec<crate::generated::models::user::GeneralUser>,
    /// No documentation provided.
    pub available_pages: Option<i32>,
    /// No documentation provided.
    pub groups: Vec<crate::generated::models::groups_v_2::GroupResponse>,
    /// No documentation provided.
    pub has_more: bool,
    /// No documentation provided.
    pub polls: Vec<crate::generated::models::forum::PollResponse>,
    /// No documentation provided.
    pub query: crate::generated::models::queries::PagedQuery,
    /// No documentation provided.
    pub recruitment_details: Vec<crate::generated::models::forum::ForumRecruitmentDetail>,
    /// No documentation provided.
    pub related_posts: Vec<crate::generated::models::forum::PostResponse>,
    /// No documentation provided.
    pub replacement_continuation_token: String,
    /// No documentation provided.
    pub results: Vec<crate::generated::models::forum::PostResponse>,
    /// No documentation provided.
    pub searched_tags: Vec<crate::generated::models::tags::models::contracts::TagResponse>,
    /// No documentation provided.
    pub total_results: i32,
    /// If useTotalResults is true, then totalResults represents an accurate count.
/// If False, it does not, and may be estimated/only the size of the current page.
/// Either way, you should probably always only trust hasMore.
/// This is a long-held historical throwback to when we used to do paging with known total results. Those queries toasted our database, and we were left to hastily alter our endpoints and create backward- compatible shims, of which useTotalResults is one.
    pub use_total_results: bool,
}
