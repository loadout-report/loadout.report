use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};
use serde_with::{serde_as, DisplayFromStr};

/// No documentation provided.
/// todo: bitmask
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum Capabilities {
    /// No documentation provided.
    None = 0,
    /// No documentation provided.
    Leaderboards = 1,
    /// No documentation provided.
    Callsign = 2,
    /// No documentation provided.
    OptionalConversations = 4,
    /// No documentation provided.
    ClanBanner = 8,
    /// No documentation provided.
    D2InvestmentData = 16,
    /// No documentation provided.
    Tags = 32,
    /// No documentation provided.
    Alliances = 64,
}

/// No documentation provided.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum ChatSecuritySetting {
    /// No documentation provided.
    Group = 0,
    /// No documentation provided.
    Admins = 1,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClanBanner {

    /// No documentation provided.
    pub decal_background_color_id: u32,
    /// No documentation provided.
    pub decal_color_id: u32,
    /// No documentation provided.
    pub decal_id: u32,
    /// No documentation provided.
    pub gonfalon_color_id: u32,
    /// No documentation provided.
    pub gonfalon_detail_color_id: u32,
    /// No documentation provided.
    pub gonfalon_detail_id: u32,
    /// No documentation provided.
    pub gonfalon_id: u32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetGroupsForMemberResponse {

    /// A convenience property that indicates if every membership this user has that is a part of this group are part of an account that is considered inactive - for example, overridden accounts in Cross Save.
///  The key is the Group ID for the group being checked, and the value is true if the users' memberships for that group are all inactive.
    pub are_all_memberships_inactive: i32,
    /// No documentation provided.
    pub has_more: bool,
    /// No documentation provided.
    pub query: crate::generated::models::queries::PagedQuery,
    /// No documentation provided.
    pub replacement_continuation_token: String,
    /// No documentation provided.
    pub results: Vec<crate::generated::models::groups_v_2::GroupMembership>,
    /// No documentation provided.
    pub total_results: i32,
    /// If useTotalResults is true, then totalResults represents an accurate count.
/// If False, it does not, and may be estimated/only the size of the current page.
/// Either way, you should probably always only trust hasMore.
/// This is a long-held historical throwback to when we used to do paging with known total results. Those queries toasted our database, and we were left to hastily alter our endpoints and create backward- compatible shims, of which useTotalResults is one.
    pub use_total_results: bool,
}

/// No documentation provided.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum GroupAllianceStatus {
    /// No documentation provided.
    Unallied = 0,
    /// No documentation provided.
    Parent = 1,
    /// No documentation provided.
    Child = 2,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GroupApplicationListRequest {

    /// No documentation provided.
    pub memberships: Vec<crate::generated::models::user::UserMembership>,
    /// No documentation provided.
    pub message: String,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GroupApplicationRequest {

    /// No documentation provided.
    pub message: String,
}

/// No documentation provided.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum GroupApplicationResolveState {
    /// No documentation provided.
    Unresolved = 0,
    /// No documentation provided.
    Accepted = 1,
    /// No documentation provided.
    Denied = 2,
    /// No documentation provided.
    Rescinded = 3,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GroupApplicationResponse {

    /// No documentation provided.
    pub resolution: crate::generated::models::groups_v_2::GroupApplicationResolveState,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GroupBan {

    /// No documentation provided.
    pub bungie_net_user_info: crate::generated::models::user::UserInfoCard,
    /// No documentation provided.
    pub comment: String,
    /// No documentation provided.
    pub created_by: crate::generated::models::user::UserInfoCard,
    /// No documentation provided.
    pub date_banned: chrono::DateTime<chrono::Utc>,
    /// No documentation provided.
    pub date_expires: chrono::DateTime<chrono::Utc>,
    /// No documentation provided.
    pub destiny_user_info: crate::generated::models::groups_v_2::GroupUserInfoCard,
    /// No documentation provided.
    #[serde(with = "crate::unfuck_js::stringified_numbers")]
    pub group_id: i64,
    /// No documentation provided.
    pub last_modified_by: crate::generated::models::user::UserInfoCard,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GroupBanRequest {

    /// No documentation provided.
    pub comment: String,
    /// No documentation provided.
    pub length: crate::generated::models::ignores::IgnoreLength,
}

/// No documentation provided.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum GroupDateRange {
    /// No documentation provided.
    All = 0,
    /// No documentation provided.
    PastDay = 1,
    /// No documentation provided.
    PastWeek = 2,
    /// No documentation provided.
    PastMonth = 3,
    /// No documentation provided.
    PastYear = 4,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GroupEditAction {

    /// No documentation provided.
    pub about: String,
    /// No documentation provided.
    pub allow_chat: Option<bool>,
    /// No documentation provided.
    pub avatar_image_index: Option<i32>,
    /// No documentation provided.
    pub callsign: String,
    /// No documentation provided.
    pub chat_security: Option<i32>,
    /// No documentation provided.
    pub default_publicity: Option<i32>,
    /// No documentation provided.
    pub enable_invitation_messaging_for_admins: Option<bool>,
    /// No documentation provided.
    pub homepage: Option<i32>,
    /// No documentation provided.
    pub is_public: Option<bool>,
    /// No documentation provided.
    pub is_public_topic_admin_only: Option<bool>,
    /// No documentation provided.
    pub locale: String,
    /// No documentation provided.
    pub membership_option: Option<i32>,
    /// No documentation provided.
    pub motto: String,
    /// No documentation provided.
    pub name: String,
    /// No documentation provided.
    pub tags: String,
    /// No documentation provided.
    pub theme: String,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GroupFeatures {

    /// No documentation provided.
    pub capabilities: crate::generated::models::groups_v_2::Capabilities,
    /// Minimum Member Level allowed to host guided games
/// Always Allowed: Founder, Acting Founder, Admin
/// Allowed Overrides: None, Member, Beginner
/// Default is Member for clans, None for groups, although this means nothing for groups.
    pub host_guided_game_permission_override: crate::generated::models::groups_v_2::HostGuidedGamesPermissionLevel,
    /// Minimum Member Level allowed to invite new members to group
/// Always Allowed: Founder, Acting Founder
/// True means admins have this power, false means they don't
/// Default is false for clans, true for groups.
    pub invite_permission_override: bool,
    /// Level to join a member at when accepting an invite, application, or joining an open clan
/// Default is Beginner.
    pub join_level: crate::generated::models::groups_v_2::RuntimeGroupMemberType,
    /// No documentation provided.
    pub maximum_members: i32,
    /// Maximum number of groups of this type a typical membership may join. For example, a user may join about 50 General groups with their Bungie.net account. They may join one clan per Destiny membership.
    pub maximum_memberships_of_group_type: i32,
    /// No documentation provided.
    pub membership_types: Vec<crate::generated::models::BungieMembershipType>,
    /// Minimum Member Level allowed to update banner
/// Always Allowed: Founder, Acting Founder
/// True means admins have this power, false means they don't
/// Default is false for clans, true for groups.
    pub update_banner_permission_override: bool,
    /// Minimum Member Level allowed to update group culture
/// Always Allowed: Founder, Acting Founder
/// True means admins have this power, false means they don't
/// Default is false for clans, true for groups.
    pub update_culture_permission_override: bool,
}

/// No documentation provided.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum GroupHomepage {
    /// No documentation provided.
    Wall = 0,
    /// No documentation provided.
    Forum = 1,
    /// No documentation provided.
    AllianceForum = 2,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GroupMember {

    /// No documentation provided.
    pub bungie_net_user_info: crate::generated::models::user::UserInfoCard,
    /// No documentation provided.
    pub destiny_user_info: crate::generated::models::groups_v_2::GroupUserInfoCard,
    /// No documentation provided.
    #[serde(with = "crate::unfuck_js::stringified_numbers")]
    pub group_id: i64,
    /// No documentation provided.
    pub is_online: bool,
    /// No documentation provided.
    pub join_date: chrono::DateTime<chrono::Utc>,
    /// No documentation provided.
    #[serde(with = "crate::unfuck_js::stringified_numbers")]
    pub last_online_status_change: i64,
    /// No documentation provided.
    pub member_type: crate::generated::models::groups_v_2::RuntimeGroupMemberType,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GroupMemberApplication {

    /// No documentation provided.
    pub bungie_net_user_info: crate::generated::models::user::UserInfoCard,
    /// No documentation provided.
    pub creation_date: chrono::DateTime<chrono::Utc>,
    /// No documentation provided.
    pub destiny_user_info: crate::generated::models::groups_v_2::GroupUserInfoCard,
    /// No documentation provided.
    #[serde(with = "crate::unfuck_js::stringified_numbers")]
    pub group_id: i64,
    /// No documentation provided.
    pub request_message: String,
    /// No documentation provided.
    pub resolve_date: Option<chrono::DateTime<chrono::Utc>>,
    /// No documentation provided.
    pub resolve_message: String,
    /// No documentation provided.
    pub resolve_state: crate::generated::models::groups_v_2::GroupApplicationResolveState,
    /// No documentation provided.
    #[serde(with = "crate::unfuck_js::nullable_stringified_numbers")]
    pub resolved_by_membership_id: Option<i64>,
}

/// No documentation provided.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum GroupMemberCountFilter {
    /// No documentation provided.
    All = 0,
    /// No documentation provided.
    OneToTen = 1,
    /// No documentation provided.
    ElevenToOneHundred = 2,
    /// No documentation provided.
    GreaterThanOneHundred = 3,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GroupMemberLeaveResult {

    /// No documentation provided.
    pub group: crate::generated::models::groups_v_2::GroupV2,
    /// No documentation provided.
    pub group_deleted: bool,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GroupMembership {

    /// No documentation provided.
    pub group: crate::generated::models::groups_v_2::GroupV2,
    /// No documentation provided.
    pub member: crate::generated::models::groups_v_2::GroupMember,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GroupMembershipBase {

    /// No documentation provided.
    pub group: crate::generated::models::groups_v_2::GroupV2,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GroupMembershipSearchResponse {

    /// No documentation provided.
    pub has_more: bool,
    /// No documentation provided.
    pub query: crate::generated::models::queries::PagedQuery,
    /// No documentation provided.
    pub replacement_continuation_token: String,
    /// No documentation provided.
    pub results: Vec<crate::generated::models::groups_v_2::GroupMembership>,
    /// No documentation provided.
    pub total_results: i32,
    /// If useTotalResults is true, then totalResults represents an accurate count.
/// If False, it does not, and may be estimated/only the size of the current page.
/// Either way, you should probably always only trust hasMore.
/// This is a long-held historical throwback to when we used to do paging with known total results. Those queries toasted our database, and we were left to hastily alter our endpoints and create backward- compatible shims, of which useTotalResults is one.
    pub use_total_results: bool,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GroupNameSearchRequest {

    /// No documentation provided.
    pub group_name: String,
    /// No documentation provided.
    pub group_type: crate::generated::models::groups_v_2::GroupType,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GroupOptionalConversation {

    /// No documentation provided.
    pub chat_enabled: bool,
    /// No documentation provided.
    pub chat_name: String,
    /// No documentation provided.
    pub chat_security: crate::generated::models::groups_v_2::ChatSecuritySetting,
    /// No documentation provided.
    #[serde(with = "crate::unfuck_js::stringified_numbers")]
    pub conversation_id: i64,
    /// No documentation provided.
    #[serde(with = "crate::unfuck_js::stringified_numbers")]
    pub group_id: i64,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GroupOptionalConversationAddRequest {

    /// No documentation provided.
    pub chat_name: String,
    /// No documentation provided.
    pub chat_security: crate::generated::models::groups_v_2::ChatSecuritySetting,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GroupOptionalConversationEditRequest {

    /// No documentation provided.
    pub chat_enabled: Option<bool>,
    /// No documentation provided.
    pub chat_name: String,
    /// No documentation provided.
    pub chat_security: Option<i32>,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GroupOptionsEditAction {

    /// Minimum Member Level allowed to host guided games
/// Always Allowed: Founder, Acting Founder, Admin
/// Allowed Overrides: None, Member, Beginner
/// Default is Member for clans, None for groups, although this means nothing for groups.
    pub host_guided_game_permission_override: Option<i32>,
    /// Minimum Member Level allowed to invite new members to group
/// Always Allowed: Founder, Acting Founder
/// True means admins have this power, false means they don't
/// Default is false for clans, true for groups.
    pub invite_permission_override: Option<bool>,
    /// Level to join a member at when accepting an invite, application, or joining an open clan
/// Default is Beginner.
    pub join_level: Option<i32>,
    /// Minimum Member Level allowed to update banner
/// Always Allowed: Founder, Acting Founder
/// True means admins have this power, false means they don't
/// Default is false for clans, true for groups.
    pub update_banner_permission_override: Option<bool>,
    /// Minimum Member Level allowed to update group culture
/// Always Allowed: Founder, Acting Founder
/// True means admins have this power, false means they don't
/// Default is false for clans, true for groups.
    pub update_culture_permission_override: Option<bool>,
}

/// No documentation provided.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum GroupPostPublicity {
    /// No documentation provided.
    Public = 0,
    /// No documentation provided.
    Alliance = 1,
    /// No documentation provided.
    Private = 2,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GroupPotentialMember {

    /// No documentation provided.
    pub bungie_net_user_info: crate::generated::models::user::UserInfoCard,
    /// No documentation provided.
    pub destiny_user_info: crate::generated::models::groups_v_2::GroupUserInfoCard,
    /// No documentation provided.
    #[serde(with = "crate::unfuck_js::stringified_numbers")]
    pub group_id: i64,
    /// No documentation provided.
    pub join_date: chrono::DateTime<chrono::Utc>,
    /// No documentation provided.
    pub potential_status: crate::generated::models::groups_v_2::GroupPotentialMemberStatus,
}

/// No documentation provided.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum GroupPotentialMemberStatus {
    /// No documentation provided.
    None = 0,
    /// No documentation provided.
    Applicant = 1,
    /// No documentation provided.
    Invitee = 2,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GroupPotentialMembership {

    /// No documentation provided.
    pub group: crate::generated::models::groups_v_2::GroupV2,
    /// No documentation provided.
    pub member: crate::generated::models::groups_v_2::GroupPotentialMember,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GroupPotentialMembershipSearchResponse {

    /// No documentation provided.
    pub has_more: bool,
    /// No documentation provided.
    pub query: crate::generated::models::queries::PagedQuery,
    /// No documentation provided.
    pub replacement_continuation_token: String,
    /// No documentation provided.
    pub results: Vec<crate::generated::models::groups_v_2::GroupPotentialMembership>,
    /// No documentation provided.
    pub total_results: i32,
    /// If useTotalResults is true, then totalResults represents an accurate count.
/// If False, it does not, and may be estimated/only the size of the current page.
/// Either way, you should probably always only trust hasMore.
/// This is a long-held historical throwback to when we used to do paging with known total results. Those queries toasted our database, and we were left to hastily alter our endpoints and create backward- compatible shims, of which useTotalResults is one.
    pub use_total_results: bool,
}

/// NOTE: GroupQuery, as of Destiny 2, has essentially two totally different and incompatible "modes".
/// If you are querying for a group, you can pass any of the properties below.
/// If you are querying for a Clan, you MUST NOT pass any of the following properties (they must be null or undefined in your request, not just empty string/default values):
/// - groupMemberCountFilter - localeFilter - tagText
/// If you pass these, you will get a useless InvalidParameters error.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GroupQuery {

    /// No documentation provided.
    pub creation_date: crate::generated::models::groups_v_2::GroupDateRange,
    /// No documentation provided.
    pub current_page: i32,
    /// No documentation provided.
    pub group_member_count_filter: Option<i32>,
    /// No documentation provided.
    pub group_type: crate::generated::models::groups_v_2::GroupType,
    /// No documentation provided.
    pub items_per_page: i32,
    /// No documentation provided.
    pub locale_filter: String,
    /// No documentation provided.
    pub name: String,
    /// No documentation provided.
    pub request_continuation_token: String,
    /// No documentation provided.
    pub sort_by: crate::generated::models::groups_v_2::GroupSortBy,
    /// No documentation provided.
    pub tag_text: String,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GroupResponse {

    /// No documentation provided.
    pub alliance_status: crate::generated::models::groups_v_2::GroupAllianceStatus,
    /// No documentation provided.
    pub allied_ids: Vec<i64>,
    /// This property will be populated if the authenticated user is a member of the group. Note that because of account linking, a user can sometimes be part of a clan more than once. As such, this returns the highest member type available.
    pub current_user_member_map: i32,
    /// A convenience property that indicates if every membership you (the current user) have that is a part of this group are part of an account that is considered inactive - for example, overridden accounts in Cross Save.
    pub current_user_memberships_inactive_for_destiny: bool,
    /// This property will be populated if the authenticated user is an applicant or has an outstanding invitation to join. Note that because of account linking, a user can sometimes be part of a clan more than once.
    pub current_user_potential_member_map: i32,
    /// No documentation provided.
    pub detail: crate::generated::models::groups_v_2::GroupV2,
    /// No documentation provided.
    pub founder: crate::generated::models::groups_v_2::GroupMember,
    /// No documentation provided.
    pub group_join_invite_count: i32,
    /// No documentation provided.
    pub parent_group: crate::generated::models::groups_v_2::GroupV2,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GroupSearchResponse {

    /// No documentation provided.
    pub has_more: bool,
    /// No documentation provided.
    pub query: crate::generated::models::queries::PagedQuery,
    /// No documentation provided.
    pub replacement_continuation_token: String,
    /// No documentation provided.
    pub results: Vec<crate::generated::models::groups_v_2::GroupV2Card>,
    /// No documentation provided.
    pub total_results: i32,
    /// If useTotalResults is true, then totalResults represents an accurate count.
/// If False, it does not, and may be estimated/only the size of the current page.
/// Either way, you should probably always only trust hasMore.
/// This is a long-held historical throwback to when we used to do paging with known total results. Those queries toasted our database, and we were left to hastily alter our endpoints and create backward- compatible shims, of which useTotalResults is one.
    pub use_total_results: bool,
}

/// No documentation provided.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum GroupSortBy {
    /// No documentation provided.
    Name = 0,
    /// No documentation provided.
    Date = 1,
    /// No documentation provided.
    Popularity = 2,
    /// No documentation provided.
    Id = 3,
}

/// No documentation provided.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum GroupType {
    /// No documentation provided.
    General = 0,
    /// No documentation provided.
    Clan = 1,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GroupUserBase {

    /// No documentation provided.
    pub bungie_net_user_info: crate::generated::models::user::UserInfoCard,
    /// No documentation provided.
    pub destiny_user_info: crate::generated::models::groups_v_2::GroupUserInfoCard,
    /// No documentation provided.
    #[serde(with = "crate::unfuck_js::stringified_numbers")]
    pub group_id: i64,
    /// No documentation provided.
    pub join_date: chrono::DateTime<chrono::Utc>,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GroupUserInfoCard {

    /// This will be the display name the clan server last saw the user as. If the account is an active cross save override, this will be the display name to use. Otherwise, this will match the displayName property.
    pub last_seen_display_name: String,
    /// The platform of the LastSeenDisplayName
    pub last_seen_display_name_type: crate::generated::models::BungieMembershipType,
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

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GroupV2 {

    /// No documentation provided.
    pub about: String,
    /// No documentation provided.
    pub allow_chat: bool,
    /// No documentation provided.
    pub avatar_image_index: i32,
    /// No documentation provided.
    pub avatar_path: String,
    /// No documentation provided.
    pub ban_expire_date: Option<chrono::DateTime<chrono::Utc>>,
    /// No documentation provided.
    pub banner_path: String,
    /// No documentation provided.
    pub chat_security: crate::generated::models::groups_v_2::ChatSecuritySetting,
    /// No documentation provided.
    pub clan_info: crate::generated::models::groups_v_2::GroupV2ClanInfoAndInvestment,
    /// No documentation provided.
    #[serde(with = "crate::unfuck_js::stringified_numbers")]
    pub conversation_id: i64,
    /// No documentation provided.
    pub creation_date: chrono::DateTime<chrono::Utc>,
    /// No documentation provided.
    pub default_publicity: crate::generated::models::groups_v_2::GroupPostPublicity,
    /// No documentation provided.
    pub enable_invitation_messaging_for_admins: bool,
    /// No documentation provided.
    pub features: crate::generated::models::groups_v_2::GroupFeatures,
    /// No documentation provided.
    #[serde(with = "crate::unfuck_js::stringified_numbers")]
    pub group_id: i64,
    /// No documentation provided.
    pub group_type: crate::generated::models::groups_v_2::GroupType,
    /// No documentation provided.
    pub homepage: crate::generated::models::groups_v_2::GroupHomepage,
    /// No documentation provided.
    pub is_default_post_public: bool,
    /// No documentation provided.
    pub is_public: bool,
    /// No documentation provided.
    pub is_public_topic_admin_only: bool,
    /// No documentation provided.
    pub locale: String,
    /// No documentation provided.
    pub member_count: i32,
    /// No documentation provided.
    #[serde(with = "crate::unfuck_js::stringified_numbers")]
    pub membership_id_created: i64,
    /// No documentation provided.
    pub membership_option: crate::generated::models::groups_v_2::MembershipOption,
    /// No documentation provided.
    pub modification_date: chrono::DateTime<chrono::Utc>,
    /// No documentation provided.
    pub motto: String,
    /// No documentation provided.
    pub name: String,
    /// No documentation provided.
    pub tags: Vec<String>,
    /// No documentation provided.
    pub theme: String,
}

/// A small infocard of group information, usually used for when a list of groups are returned
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GroupV2Card {

    /// No documentation provided.
    pub about: String,
    /// No documentation provided.
    pub avatar_path: String,
    /// No documentation provided.
    pub capabilities: crate::generated::models::groups_v_2::Capabilities,
    /// No documentation provided.
    pub clan_info: crate::generated::models::groups_v_2::GroupV2ClanInfo,
    /// No documentation provided.
    pub creation_date: chrono::DateTime<chrono::Utc>,
    /// No documentation provided.
    #[serde(with = "crate::unfuck_js::stringified_numbers")]
    pub group_id: i64,
    /// No documentation provided.
    pub group_type: crate::generated::models::groups_v_2::GroupType,
    /// No documentation provided.
    pub locale: String,
    /// No documentation provided.
    pub member_count: i32,
    /// No documentation provided.
    pub membership_option: crate::generated::models::groups_v_2::MembershipOption,
    /// No documentation provided.
    pub motto: String,
    /// No documentation provided.
    pub name: String,
    /// No documentation provided.
    pub theme: String,
}

/// This contract contains clan-specific group information. It does not include any investment data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GroupV2ClanInfo {

    /// No documentation provided.
    pub clan_banner_data: crate::generated::models::groups_v_2::ClanBanner,
    /// No documentation provided.
    pub clan_callsign: String,
}

/// The same as GroupV2ClanInfo, but includes any investment data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GroupV2ClanInfoAndInvestment {

    /// No documentation provided.
    pub clan_banner_data: crate::generated::models::groups_v_2::ClanBanner,
    /// No documentation provided.
    pub clan_callsign: String,
    /// No documentation provided.
    pub d_2_clan_progressions: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum GroupsForMemberFilter {
    /// No documentation provided.
    All = 0,
    /// No documentation provided.
    Founded = 1,
    /// No documentation provided.
    NonFounded = 2,
}

/// Used for setting the guided game permission level override (admins and founders can always host guided games).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum HostGuidedGamesPermissionLevel {
    /// No documentation provided.
    None = 0,
    /// No documentation provided.
    Beginner = 1,
    /// No documentation provided.
    Member = 2,
}

/// No documentation provided.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum MembershipOption {
    /// No documentation provided.
    Reviewed = 0,
    /// No documentation provided.
    Open = 1,
    /// No documentation provided.
    Closed = 2,
}

/// The member levels used by all V2 Groups API. Individual group types use their own mappings in their native storage (general uses BnetDbGroupMemberType and D2 clans use ClanMemberLevel), but they are all translated to this in the runtime api. These runtime values should NEVER be stored anywhere, so the values can be changed as necessary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum RuntimeGroupMemberType {
    /// No documentation provided.
    None = 0,
    /// No documentation provided.
    Beginner = 1,
    /// No documentation provided.
    Member = 2,
    /// No documentation provided.
    Admin = 3,
    /// No documentation provided.
    ActingFounder = 4,
    /// No documentation provided.
    Founder = 5,
}
