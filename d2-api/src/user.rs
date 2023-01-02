use crate::BungieCredentialType;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::str::FromStr;

/// A wrapper around a membership ID. Provides helpful utilities like automatic conversion
/// from and to a string, as well as strong typing for membership Ids.
/// This is NOT a Destiny Membership Id, but rather a Bungie.net membership ID.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct MembershipId(i64);

impl FromStr for MembershipId {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<i64>()
            .map(MembershipId)
            .map_err(|e| e.to_string())
    }
}

impl Display for MembershipId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Membership {
    /// Type of the membership. Not necessarily the native type.
    #[serde(rename = "membershipType")]
    pub type_: MembershipType,
    /// Membership ID as they user is known in the Accounts service
    #[serde(with = "crate::util::serde::long", rename = "membershipId")]
    pub id: MembershipId,
    /// Display Name the player has chosen for themselves.
    /// The display name is optional when the data type is used as input to a platform API.
    #[serde(deserialize_with = "crate::util::serde::empty_string_as_none")]
    pub display_name: Option<String>,
    /// The bungie global display name, if set.
    /// The sdk automatically coerces this value to an empty string if None, and vice versa.
    // todo: replace with wrapper struct
    #[serde(rename = "bungieGlobalDisplayName")]
    #[serde(deserialize_with = "crate::util::serde::empty_string_as_none")]
    pub global_display_name: Option<String>,
    /// The bungie global display name code, if set.
    #[serde(rename = "bungieGlobalDisplayNameCode")]
    pub global_display_name_code: Option<i16>,
}

/// The types of membership the Accounts system supports.
/// This is the external facing enum used in place of the internal-only
/// Bungie.SharedDefinitions.MembershipType.
#[derive(Clone, Serialize, Deserialize, Debug)]
// #[serde(rename_all = "PascalCase")]
pub enum MembershipType {
    None = 0,
    TigerXbox = 1,
    TigerPsn = 2,
    TigerSteam = 3,
    TigerBlizzard = 4,
    TigerStadia = 5,
    TigerEgs = 6,
    TigerDemon = 10,
    BungieNext = 254,
    /// "All" is only valid for searching capabilities:
    /// you need to pass the actual matching BungieMembershipType
    /// for any query where you pass a known membershipId.
    All = -1,
}

/// Very basic info about a user as returned by the Account server,
/// but including CrossSave information. Do NOT use as a request contract.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CrossSaveUserMembership {
    #[serde(flatten)]
    pub membership: Membership,
    /// If there is a cross save override in effect, this value will tell you the
    /// type that is overriding this one.
    pub cross_save_override: Option<MembershipType>,
    /// The list of Membership Types indicating
    /// the platforms on which this Membership can be used.
    ///
    /// Not in Cross Save = its original membership type.
    /// Cross Save Primary = Any membership types it is overriding,
    ///                      and its original membership type
    /// Cross Save Overridden = Empty list
    pub applicable_membership_types: Vec<MembershipType>,
    /// If True, this is a public user membership.
    pub is_public: bool,
}

/// This contract supplies basic information commonly used
/// to display a minimal amount of information about a user.
///
/// Take care to not add more properties here unless the property applies in all
/// (or at least the majority) of the situations where UserInfoCard is used.
/// Avoid adding game specific or platform specific details here.
/// In cases where UserInfoCard is a subset of the data needed in a contract,
/// use UserInfoCard as a property of other contracts.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct InfoCard {
    #[serde(flatten)]
    pub membership: CrossSaveUserMembership,
    /// A platform specific additional display name - ex: psn Real Name, bnet Unique Name, etc.
    pub supplemental_display_name: String,
    /// URL of the Icon if available.
    pub icon_path: String,
}

// todo: this struct needs some serious doc enhancements
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct GeneralUser {
    /// Bungie.net membership ID.
    pub membership_id: MembershipId,
    /// Unique name consisting of a username and discriminator.
    // todo: create a struct that captures both of these and can deserialise either representation
    pub unique_name: String,
    /// Seemingly optional? Not sure what this is.
    pub normalized_name: Option<String>,
    pub display_name: String,
    // todo: make id
    #[serde(deserialize_with = "crate::util::serde::zero_as_none")]
    pub profile_picture: Option<i32>,
    // todo: make id
    #[serde(deserialize_with = "crate::util::serde::zero_as_none")]
    pub profile_theme: Option<i32>,
    #[serde(deserialize_with = "crate::util::serde::zero_as_none")]
    pub user_title: Option<i32>,
    /// No idea what this is. Seems to be a string of a number?
    #[serde(with = "crate::util::serde::long")]
    pub success_message_flags: i64,
    pub is_deleted: bool,
    pub about: String,
    pub first_access: Option<chrono::DateTime<chrono::Utc>>,
    pub last_update: Option<chrono::DateTime<chrono::Utc>>,
    /// No idea what this is. Seems to be optional.
    pub legacy_portal_uid: Option<i64>,
    /// No idea what this is. Seems to be optional.
    pub context: Option<UserToUserContext>,
    pub show_activity: bool,
    pub locale: String,
    pub locale_inherit_default: bool,
    pub last_ban_report_id: i32,
    pub show_group_messaging: bool,
    pub profile_picture_path: String,
    /// No idea what this is. Seems to be optional.
    pub profile_picture_wide_path: Option<String>,
    pub profile_theme_name: String,
    pub user_title_display: String,
    #[serde(deserialize_with = "crate::util::serde::empty_string_as_none")]
    pub status_text: Option<String>,
    // todo: zero date as none
    #[serde(deserialize_with = "crate::util::serde::zero_date_as_none")]
    pub status_date: Option<chrono::DateTime<chrono::Utc>>,
    pub profile_ban_expire: Option<chrono::DateTime<chrono::Utc>>,
    pub cached_bungie_global_display_name: String,
    pub cached_bungie_global_display_name_code: i16,

    // display names
    // ig I could move this to its own struct as well
    pub psn_display_name: Option<String>,
    pub xbox_display_name: Option<String>,
    pub fb_display_name: Option<String>,
    pub blizzard_display_name: Option<String>,
    pub steam_display_name: Option<String>,
    pub stadia_display_name: Option<String>,
    pub twitch_display_name: Option<String>,
    pub egs_display_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct UserToUserContext {
    pub is_following: bool,
    pub ignore_status: crate::ignores::model::IgnoreResponse,
    pub global_ignore_end_date: Option<chrono::DateTime<chrono::Utc>>,
}

/// Originally Users.Models.GetCredentialTypesForAccountResponse but flattened because it's the
/// only struct in its namespace.
pub struct GetCredentialTypesForAccountResponse {
    pub credential_type: BungieCredentialType,
    pub credential_display_name: String,
    pub is_public: bool,
    pub credential_as_string: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct MembershipData {
    /// this allows you to see destiny memberships that are visible and linked to this account
    /// (regardless of whether or not they have characters on the world server)
    pub destiny_memberships: Vec<crate::groupsv2::model::GroupUserInfoCard>,
    /// If this property is populated, it will have the membership ID of the account considered
    /// to be "primary" in this user's cross save relationship.
    /// If null, this user has no cross save relationship, nor primary account.
    #[serde(deserialize_with = "crate::util::serde::zero_as_none")]
    pub primary_membership_id: Option<MembershipId>,
    pub bungie_net_user: GeneralUser,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct HardLinkedUserMembership {
    pub membership_type: MembershipType,
    pub membership_id: MembershipId,
    pub cross_save_overridden_type: Option<MembershipType>,
    pub cross_save_overridden_membership_id: Option<MembershipId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct SearchResponse {
    pub search_results: Vec<SearchResponseDetail>,
    pub page: i32,
    pub has_more: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct SearchResponseDetail {
    // todo: replace with wrapper struct
    #[serde(rename = "bungieGlobalDisplayName")]
    #[serde(deserialize_with = "crate::util::serde::empty_string_as_none")]
    pub global_display_name: Option<String>,
    #[serde(rename = "bungieGlobalDisplayNameCode")]
    pub global_display_name_code: Option<i16>,

    pub bungie_net_membership_id: Option<MembershipId>,
    pub destiny_memberships: Vec<InfoCard>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct SearchPrefixRequest {
    pub display_name_prefix: String,
}
