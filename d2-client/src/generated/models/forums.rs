use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};
use serde_with::{serde_as, DisplayFromStr};

/// No documentation provided.
/// todo: bitmask
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum ForumFlagsEnum {
    /// No documentation provided.
    None = 0,
    /// No documentation provided.
    BungieStaffPost = 1,
    /// No documentation provided.
    ForumNinjaPost = 2,
    /// No documentation provided.
    ForumMentorPost = 4,
    /// No documentation provided.
    TopicBungieStaffPosted = 8,
    /// No documentation provided.
    TopicBungieVolunteerPosted = 16,
    /// No documentation provided.
    QuestionAnsweredByBungie = 32,
    /// No documentation provided.
    QuestionAnsweredByNinja = 64,
    /// No documentation provided.
    CommunityContent = 128,
}

/// No documentation provided.
/// todo: bitmask
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum ForumPostCategoryEnums {
    /// No documentation provided.
    None = 0,
    /// No documentation provided.
    TextOnly = 1,
    /// No documentation provided.
    Media = 2,
    /// No documentation provided.
    Link = 4,
    /// No documentation provided.
    Poll = 8,
    /// No documentation provided.
    Question = 16,
    /// No documentation provided.
    Answered = 32,
    /// No documentation provided.
    Announcement = 64,
    /// No documentation provided.
    ContentComment = 128,
    /// No documentation provided.
    BungieOfficial = 256,
    /// No documentation provided.
    NinjaOfficial = 512,
    /// No documentation provided.
    Recruitment = 1024,
}
