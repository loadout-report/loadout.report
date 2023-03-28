use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};
use serde_with::{serde_as, DisplayFromStr};

/// No documentation provided.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum FireteamDateRange {
    /// No documentation provided.
    All = 0,
    /// No documentation provided.
    Now = 1,
    /// No documentation provided.
    TwentyFourHours = 2,
    /// No documentation provided.
    FortyEightHours = 3,
    /// No documentation provided.
    ThisWeek = 4,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FireteamMember {

    /// No documentation provided.
    pub bungie_net_user_info: crate::generated::models::user::UserInfoCard,
    /// No documentation provided.
    #[serde(with = "crate::unfuck_js::stringified_numbers")]
    pub character_id: i64,
    /// No documentation provided.
    pub date_joined: chrono::DateTime<chrono::Utc>,
    /// No documentation provided.
    pub destiny_user_info: crate::generated::models::fireteam::FireteamUserInfoCard,
    /// No documentation provided.
    pub has_microphone: bool,
    /// No documentation provided.
    pub last_platform_invite_attempt_date: chrono::DateTime<chrono::Utc>,
    /// No documentation provided.
    pub last_platform_invite_attempt_result: crate::generated::models::fireteam::FireteamPlatformInviteResult,
}

/// No documentation provided.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum FireteamPlatform {
    /// No documentation provided.
    Any = 0,
    /// No documentation provided.
    Playstation4 = 1,
    /// No documentation provided.
    XboxOne = 2,
    /// No documentation provided.
    Blizzard = 3,
    /// No documentation provided.
    Steam = 4,
    /// No documentation provided.
    Stadia = 5,
    /// No documentation provided.
    Egs = 6,
}

/// No documentation provided.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum FireteamPlatformInviteResult {
    /// No documentation provided.
    None = 0,
    /// No documentation provided.
    Success = 1,
    /// No documentation provided.
    AlreadyInFireteam = 2,
    /// No documentation provided.
    Throttled = 3,
    /// No documentation provided.
    ServiceError = 4,
}

/// No documentation provided.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum FireteamPublicSearchOption {
    /// No documentation provided.
    PublicAndPrivate = 0,
    /// No documentation provided.
    PublicOnly = 1,
    /// No documentation provided.
    PrivateOnly = 2,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FireteamResponse {

    /// No documentation provided.
    pub alternates: Vec<crate::generated::models::fireteam::FireteamMember>,
    /// No documentation provided.
    pub members: Vec<crate::generated::models::fireteam::FireteamMember>,
    /// No documentation provided.
    pub summary: crate::generated::models::fireteam::FireteamSummary,
}

/// No documentation provided.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum FireteamSlotSearch {
    /// No documentation provided.
    NoSlotRestriction = 0,
    /// No documentation provided.
    HasOpenPlayerSlots = 1,
    /// No documentation provided.
    HasOpenPlayerOrAltSlots = 2,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FireteamSummary {

    /// No documentation provided.
    pub activity_type: i32,
    /// No documentation provided.
    pub alternate_slot_count: Option<i32>,
    /// No documentation provided.
    pub available_alternate_slot_count: i32,
    /// No documentation provided.
    pub available_player_slot_count: i32,
    /// No documentation provided.
    pub date_created: chrono::DateTime<chrono::Utc>,
    /// No documentation provided.
    pub date_modified: Option<chrono::DateTime<chrono::Utc>>,
    /// No documentation provided.
    pub date_player_modified: chrono::DateTime<chrono::Utc>,
    /// No documentation provided.
    #[serde(with = "crate::unfuck_js::stringified_numbers")]
    pub fireteam_id: i64,
    /// No documentation provided.
    #[serde(with = "crate::unfuck_js::stringified_numbers")]
    pub group_id: i64,
    /// No documentation provided.
    pub is_immediate: bool,
    /// No documentation provided.
    pub is_public: bool,
    /// No documentation provided.
    pub is_valid: bool,
    /// No documentation provided.
    pub locale: String,
    /// No documentation provided.
    pub owner_current_guardian_rank_snapshot: crate::id::Id<crate::generated::models::destiny::definitions::guardian_ranks::DestinyGuardianRankDefinition>,
    /// No documentation provided.
    pub owner_highest_lifetime_guardian_rank_snapshot: crate::id::Id<crate::generated::models::destiny::definitions::guardian_ranks::DestinyGuardianRankDefinition>,
    /// No documentation provided.
    #[serde(with = "crate::unfuck_js::stringified_numbers")]
    pub owner_membership_id: i64,
    /// No documentation provided.
    pub owner_total_commendation_score_snapshot: i32,
    /// No documentation provided.
    pub platform: crate::generated::models::fireteam::FireteamPlatform,
    /// No documentation provided.
    pub player_slot_count: i32,
    /// No documentation provided.
    pub scheduled_time: Option<chrono::DateTime<chrono::Utc>>,
    /// No documentation provided.
    pub title: String,
    /// No documentation provided.
    pub title_before_moderation: String,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FireteamUserInfoCard {

    /// No documentation provided.
    pub fireteam_display_name: String,
    /// No documentation provided.
    pub fireteam_membership_type: crate::generated::models::BungieMembershipType,
    /// The list of Membership Types indicating the platforms on which this Membership can be used.
///  Not in Cross Save = its original membership type. Cross Save Primary = Any membership types it is overridding, and its original membership type Cross Save Overridden = Empty list
    pub applicable_membership_types: Vec<crate::generated::models::BungieMembershipType>,
    /// The bungie global display name, if set.
    pub bungie_global_display_name: String,
    /// The bungie global display name code, if set.
    pub bungie_global_display_name_code: Option<i16>,
    /// If there is a cross save override in effect, this value will tell you the type that is overridding this one.
    pub cross_save_override: crate::generated::models::BungieMembershipType,
    /// Display Name the player has chosen for themselves. The display name is optional when the data type is used as input to a platform API.
    pub display_name: String,
    /// URL the Icon if available.
    pub icon_path: String,
    /// If True, this is a public user membership.
    pub is_public: bool,
    /// Membership ID as they user is known in the Accounts service
    #[serde(with = "crate::unfuck_js::stringified_numbers")]
    pub membership_id: i64,
    /// Type of the membership. Not necessarily the native type.
    pub membership_type: crate::generated::models::BungieMembershipType,
    /// A platform specific additional display name - ex: psn Real Name, bnet Unique Name, etc.
    pub supplemental_display_name: String,
}
