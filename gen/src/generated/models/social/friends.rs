use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};
use serde_with::{serde_as, DisplayFromStr};

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BungieFriend {

    /// No documentation provided.
    pub bungie_global_display_name: String,
    /// No documentation provided.
    pub bungie_global_display_name_code: Option<i16>,
    /// No documentation provided.
    pub bungie_net_user: crate::generated::models::user::GeneralUser,
    /// No documentation provided.
    pub last_seen_as_bungie_membership_type: crate::generated::models::BungieMembershipType,
    /// No documentation provided.
    #[serde(with = "crate::unfuck_js::stringified_numbers")]
    pub last_seen_as_membership_id: i64,
    /// No documentation provided.
    pub online_status: crate::generated::models::social::friends::PresenceStatus,
    /// No documentation provided.
    pub online_title: crate::generated::models::social::friends::PresenceOnlineStateFlags,
    /// No documentation provided.
    pub relationship: crate::generated::models::social::friends::FriendRelationshipState,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BungieFriendListResponse {

    /// No documentation provided.
    pub friends: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BungieFriendRequestListResponse {

    /// No documentation provided.
    pub incoming_requests: i32,
    /// No documentation provided.
    pub outgoing_requests: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum FriendRelationshipState {
    /// No documentation provided.
    Unknown = 0,
    /// No documentation provided.
    Friend = 1,
    /// No documentation provided.
    IncomingRequest = 2,
    /// No documentation provided.
    OutgoingRequest = 3,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlatformFriend {

    /// No documentation provided.
    pub bungie_global_display_name: String,
    /// No documentation provided.
    pub bungie_global_display_name_code: Option<i16>,
    /// No documentation provided.
    #[serde(with = "crate::unfuck_js::nullable_stringified_numbers")]
    pub bungie_net_membership_id: Option<i64>,
    /// No documentation provided.
    #[serde(with = "crate::unfuck_js::nullable_stringified_numbers")]
    pub destiny_membership_id: Option<i64>,
    /// No documentation provided.
    pub destiny_membership_type: Option<i32>,
    /// No documentation provided.
    pub friend_platform: crate::generated::models::social::friends::PlatformFriendType,
    /// No documentation provided.
    pub platform_display_name: String,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlatformFriendResponse {

    /// No documentation provided.
    pub current_page: i32,
    /// No documentation provided.
    pub has_more: bool,
    /// No documentation provided.
    pub items_per_page: i32,
    /// No documentation provided.
    pub platform_friends: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum PlatformFriendType {
    /// No documentation provided.
    Unknown = 0,
    /// No documentation provided.
    Xbox = 1,
    /// No documentation provided.
    PSN = 2,
    /// No documentation provided.
    Steam = 3,
    /// No documentation provided.
    Egs = 4,
}

/// No documentation provided.
/// todo: bitmask
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum PresenceOnlineStateFlags {
    /// No documentation provided.
    None = 0,
    /// No documentation provided.
    Destiny1 = 1,
    /// No documentation provided.
    Destiny2 = 2,
}

/// No documentation provided.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum PresenceStatus {
    /// No documentation provided.
    OfflineOrUnknown = 0,
    /// No documentation provided.
    Online = 1,
}
