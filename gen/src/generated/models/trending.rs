

use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrendingEntryCommunityCreation {

    /// No documentation provided.
    #[serde(with = "crate::unfuck_js::stringified_numbers")]
    pub post_id: i64,
    /// No documentation provided.
    pub upvotes: i32,
    /// No documentation provided.
    pub media: String,
    /// No documentation provided.
    pub title: String,
    /// No documentation provided.
    #[serde(with = "crate::unfuck_js::stringified_numbers")]
    pub author_membership_id: i64,
    /// No documentation provided.
    pub author: String,
    /// No documentation provided.
    pub body: String,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrendingEntrySupportArticle {

    /// No documentation provided.
    pub article: i32,
}

/// The list entry view for trending items. Returns just enough to show the item on the trending page.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrendingEntry {

    /// No documentation provided.
    pub image: String,
    /// The weighted score of this trending item.
    pub weight: f64,
    /// If the entry has a date at which it was created, this is that date.
    pub creation_date: Option<chrono::DateTime<chrono::Utc>>,
    /// No documentation provided.
    pub link: String,
    /// No documentation provided.
    pub is_featured: bool,
    /// If the item is of entityType TrendingEntryType.Container, it may have items - also Trending Entries - contained within it. This is the ordered list of those to display under the Container's header.
    pub items: i32,
    /// No documentation provided.
    pub end_date: Option<chrono::DateTime<chrono::Utc>>,
    /// If this is populated, the entry has a related MP4 video to show. I am 100% certain I am going to regret putting this directly on TrendingEntry, but it will work so yolo
    pub mp_4_video: String,
    /// The localized "display name/article title/'primary localized identifier'" of the entity.
    pub display_name: String,
    /// If the entity has a localized tagline/subtitle/motto/whatever, that is found here.
    pub tagline: String,
    /// If isFeatured, this image will be populated with whatever the featured image is. Note that this will likely be a very large image, so don't use it all the time.
    pub feature_image: String,
    /// If this is populated, the entry has a related WebM video to show. I am 100% certain I am going to regret putting this directly on TrendingEntry, but it will work so yolo
    pub webm_video: String,
    /// An enum - unfortunately - dictating all of the possible kinds of trending items that you might get in your result set, in case you want to do custom rendering or call to get the details of the item.
    pub entity_type: i32,
    /// No documentation provided.
    pub start_date: Option<chrono::DateTime<chrono::Utc>>,
    /// We don't know whether the identifier will be a string, a uint, or a long... so we're going to cast it all to a string. But either way, we need any trending item created to have a single unique identifier for its type.
    pub identifier: String,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrendingCategories {

    /// No documentation provided.
    pub categories: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrendingEntryDestinyActivity {

    /// No documentation provided.
    pub activity_hash: u32,
    /// No documentation provided.
    pub status: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrendingEntryNews {

    /// No documentation provided.
    pub article: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrendingCategory {

    /// No documentation provided.
    pub category_id: String,
    /// No documentation provided.
    pub entries: i32,
    /// No documentation provided.
    pub category_name: String,
}

/// The known entity types that you can have returned from Trending.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum TrendingEntryType {
    /// No documentation provided.
    News = 0,
    /// No documentation provided.
    DestinyItem = 1,
    /// No documentation provided.
    DestinyActivity = 2,
    /// No documentation provided.
    DestinyRitual = 3,
    /// No documentation provided.
    SupportArticle = 4,
    /// No documentation provided.
    Creation = 5,
    /// No documentation provided.
    Stream = 6,
    /// No documentation provided.
    Update = 7,
    /// No documentation provided.
    Link = 8,
    /// No documentation provided.
    ForumTag = 9,
    /// No documentation provided.
    Container = 10,
    /// No documentation provided.
    Release = 11,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrendingDetail {

    /// No documentation provided.
    pub creation: i32,
    /// No documentation provided.
    pub destiny_activity: i32,
    /// No documentation provided.
    pub destiny_ritual: i32,
    /// No documentation provided.
    pub identifier: String,
    /// No documentation provided.
    pub support: i32,
    /// No documentation provided.
    pub destiny_item: i32,
    /// No documentation provided.
    pub news: i32,
    /// No documentation provided.
    pub entity_type: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrendingEntryDestinyRitual {

    /// A destiny event does not necessarily have a related Milestone, but if it does the details will be returned here.
    pub milestone_details: i32,
    /// No documentation provided.
    pub title: String,
    /// No documentation provided.
    pub date_end: Option<chrono::DateTime<chrono::Utc>>,
    /// No documentation provided.
    pub date_start: Option<chrono::DateTime<chrono::Utc>>,
    /// A destiny event will not necessarily have milestone "custom content", but if it does the details will be here.
    pub event_content: i32,
    /// No documentation provided.
    pub image: String,
    /// No documentation provided.
    pub icon: String,
    /// No documentation provided.
    pub subtitle: String,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrendingEntryDestinyItem {

    /// No documentation provided.
    pub item_hash: u32,
}
