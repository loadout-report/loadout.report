use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};
use serde_with::{serde_as, DisplayFromStr};
pub mod models;
/// Very basic info about a user as returned by the Account server, but including CrossSave information. Do NOT use as a request contract.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CrossSaveUserMembership {

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
    /// If True, this is a public user membership.
    pub is_public: bool,
    /// Membership ID as they user is known in the Accounts service
    #[serde(with = "crate::unfuck_js::stringified_numbers")]
    pub membership_id: i64,
    /// Type of the membership. Not necessarily the native type.
    pub membership_type: crate::generated::models::BungieMembershipType,
}

/// Localized text relevant to a given EMail setting in a given localization.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EMailSettingLocalization {

    /// No documentation provided.
    pub description: String,
    /// No documentation provided.
    pub title: String,
}

/// Localized text relevant to a given EMail setting in a given localization. Extra settings specifically for subscriptions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EMailSettingSubscriptionLocalization {

    /// No documentation provided.
    pub description: String,
    /// No documentation provided.
    pub known_user_action_text: String,
    /// No documentation provided.
    pub registered_user_description: String,
    /// No documentation provided.
    pub title: String,
    /// No documentation provided.
    pub unknown_user_action_text: String,
    /// No documentation provided.
    pub unknown_user_description: String,
    /// No documentation provided.
    pub unregistered_user_description: String,
}

/// Defines a single opt-in category: a wide-scoped permission to send emails for the subject related to the opt-in.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EmailOptInDefinition {

    /// Information about the dependent subscriptions for this opt-in.
    pub dependent_subscriptions: Vec<crate::generated::models::user::EmailSubscriptionDefinition>,
    /// The unique identifier for this opt-in category.
    pub name: String,
    /// If true, this opt-in setting should be set by default in situations where accounts are created without explicit choices about what they're opting into.
    pub set_by_default: bool,
    /// The flag value for this opt-in category. For historical reasons, this is defined as a flags enum.
    pub value: crate::generated::models::user::OptInFlags,
}

/// The set of all email subscription/opt-in settings and definitions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EmailSettings {

    /// Keyed by the name identifier of the opt-in definition.
    pub opt_in_definitions: i32,
    /// Keyed by the name identifier of the Subscription definition.
    pub subscription_definitions: i32,
    /// Keyed by the name identifier of the View definition.
    pub views: i32,
}

/// Defines a single subscription: permission to send emails for a specific, focused subject (generally timeboxed, such as for a specific release of a product or feature).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EmailSubscriptionDefinition {

    /// A dictionary of localized text for the EMail Opt-in setting, keyed by the locale.
    pub localization: i32,
    /// The unique identifier for this subscription.
    pub name: String,
    /// The bitflag value for this subscription. Should be a unique power of two value.
    #[serde(with = "crate::unfuck_js::stringified_numbers")]
    pub value: i64,
}

/// Represents a data-driven view for Email settings. Web/Mobile UI can use this data to show new EMail settings consistently without further manual work.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EmailViewDefinition {

    /// The identifier for this view.
    pub name: String,
    /// The ordered list of settings to show in this view.
    pub view_settings: Vec<crate::generated::models::user::EmailViewDefinitionSetting>,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EmailViewDefinitionSetting {

    /// A dictionary of localized text for the EMail setting, keyed by the locale.
    pub localization: i32,
    /// The identifier for this UI Setting, which can be used to relate it to custom strings or other data as desired.
    pub name: String,
    /// The OptInFlags value to set or clear if this setting is set or cleared in the UI. It is the aggregate of all underlying opt-in flags related to this setting.
    pub opt_in_aggregate_value: crate::generated::models::user::OptInFlags,
    /// If true, this setting should be set by default if the user hasn't chosen whether it's set or cleared yet.
    pub set_by_default: bool,
    /// The subscriptions to show as children of this setting, if any.
    pub subscriptions: Vec<crate::generated::models::user::EmailSubscriptionDefinition>,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExactSearchRequest {

    /// No documentation provided.
    pub display_name: String,
    /// No documentation provided.
    pub display_name_code: i16,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeneralUser {

    /// No documentation provided.
    pub about: String,
    /// No documentation provided.
    pub blizzard_display_name: String,
    /// No documentation provided.
    pub cached_bungie_global_display_name: String,
    /// No documentation provided.
    pub cached_bungie_global_display_name_code: Option<i16>,
    /// No documentation provided.
    pub context: crate::generated::models::user::UserToUserContext,
    /// No documentation provided.
    pub display_name: String,
    /// No documentation provided.
    pub egs_display_name: String,
    /// No documentation provided.
    pub fb_display_name: String,
    /// No documentation provided.
    pub first_access: Option<chrono::DateTime<chrono::Utc>>,
    /// No documentation provided.
    pub is_deleted: bool,
    /// No documentation provided.
    #[serde(with = "crate::unfuck_js::nullable_stringified_numbers")]
    pub last_ban_report_id: Option<i64>,
    /// No documentation provided.
    pub last_update: Option<chrono::DateTime<chrono::Utc>>,
    /// No documentation provided.
    #[serde(with = "crate::unfuck_js::nullable_stringified_numbers")]
    pub legacy_portal_uid: Option<i64>,
    /// No documentation provided.
    pub locale: String,
    /// No documentation provided.
    pub locale_inherit_default: bool,
    /// No documentation provided.
    #[serde(with = "crate::unfuck_js::stringified_numbers")]
    pub membership_id: i64,
    /// No documentation provided.
    pub normalized_name: String,
    /// No documentation provided.
    pub profile_ban_expire: Option<chrono::DateTime<chrono::Utc>>,
    /// No documentation provided.
    pub profile_picture: i32,
    /// No documentation provided.
    pub profile_picture_path: String,
    /// No documentation provided.
    pub profile_picture_wide_path: String,
    /// No documentation provided.
    pub profile_theme: i32,
    /// No documentation provided.
    pub profile_theme_name: String,
    /// No documentation provided.
    pub psn_display_name: String,
    /// No documentation provided.
    pub show_activity: Option<bool>,
    /// No documentation provided.
    pub show_group_messaging: bool,
    /// No documentation provided.
    pub stadia_display_name: String,
    /// No documentation provided.
    pub status_date: chrono::DateTime<chrono::Utc>,
    /// No documentation provided.
    pub status_text: String,
    /// No documentation provided.
    pub steam_display_name: String,
    /// No documentation provided.
    #[serde(with = "crate::unfuck_js::stringified_numbers")]
    pub success_message_flags: i64,
    /// No documentation provided.
    pub twitch_display_name: String,
    /// No documentation provided.
    pub unique_name: String,
    /// No documentation provided.
    pub user_title: i32,
    /// No documentation provided.
    pub user_title_display: String,
    /// No documentation provided.
    pub xbox_display_name: String,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HardLinkedUserMembership {

    /// No documentation provided.
    #[serde(with = "crate::unfuck_js::nullable_stringified_numbers")]
    pub cross_save_overridden_membership_id: Option<i64>,
    /// No documentation provided.
    pub cross_save_overridden_type: crate::generated::models::BungieMembershipType,
    /// No documentation provided.
    #[serde(with = "crate::unfuck_js::stringified_numbers")]
    pub membership_id: i64,
    /// No documentation provided.
    pub membership_type: crate::generated::models::BungieMembershipType,
}

/// No documentation provided.
/// todo: bitmask
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr)]
#[repr(i64)]
pub enum OptInFlags {
    /// No documentation provided.
    None = 0,
    /// No documentation provided.
    Newsletter = 1,
    /// No documentation provided.
    System = 2,
    /// No documentation provided.
    Marketing = 4,
    /// No documentation provided.
    UserResearch = 8,
    /// No documentation provided.
    CustomerService = 16,
    /// No documentation provided.
    Social = 32,
    /// No documentation provided.
    PlayTests = 64,
    /// No documentation provided.
    PlayTestsLocal = 128,
    /// No documentation provided.
    Careers = 256,
}

/// This contract supplies basic information commonly used to display a minimal amount of information about a user. Take care to not add more properties here unless the property applies in all (or at least the majority) of the situations where UserInfoCard is used. Avoid adding game specific or platform specific details here. In cases where UserInfoCard is a subset of the data needed in a contract, use UserInfoCard as a property of other contracts.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserInfoCard {

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

/// Very basic info about a user as returned by the Account server.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserMembership {

    /// The bungie global display name, if set.
    pub bungie_global_display_name: String,
    /// The bungie global display name code, if set.
    pub bungie_global_display_name_code: Option<i16>,
    /// Display Name the player has chosen for themselves. The display name is optional when the data type is used as input to a platform API.
    pub display_name: String,
    /// Membership ID as they user is known in the Accounts service
    #[serde(with = "crate::unfuck_js::stringified_numbers")]
    pub membership_id: i64,
    /// Type of the membership. Not necessarily the native type.
    pub membership_type: crate::generated::models::BungieMembershipType,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserMembershipData {

    /// No documentation provided.
    pub bungie_net_user: crate::generated::models::user::GeneralUser,
    /// this allows you to see destiny memberships that are visible and linked to this account (regardless of whether or not they have characters on the world server)
    pub destiny_memberships: Vec<crate::generated::models::groups_v_2::GroupUserInfoCard>,
    /// If this property is populated, it will have the membership ID of the account considered to be "primary" in this user's cross save relationship.
///  If null, this user has no cross save relationship, nor primary account.
    #[serde(with = "crate::unfuck_js::nullable_stringified_numbers")]
    pub primary_membership_id: Option<i64>,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserSearchPrefixRequest {

    /// No documentation provided.
    pub display_name_prefix: String,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserSearchResponse {

    /// No documentation provided.
    pub has_more: bool,
    /// No documentation provided.
    pub page: i32,
    /// No documentation provided.
    pub search_results: Vec<crate::generated::models::user::UserSearchResponseDetail>,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserSearchResponseDetail {

    /// No documentation provided.
    pub bungie_global_display_name: String,
    /// No documentation provided.
    pub bungie_global_display_name_code: Option<i16>,
    /// No documentation provided.
    #[serde(with = "crate::unfuck_js::nullable_stringified_numbers")]
    pub bungie_net_membership_id: Option<i64>,
    /// No documentation provided.
    pub destiny_memberships: Vec<crate::generated::models::user::UserInfoCard>,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserToUserContext {

    /// No documentation provided.
    pub global_ignore_end_date: Option<chrono::DateTime<chrono::Utc>>,
    /// No documentation provided.
    pub ignore_status: crate::generated::models::ignores::IgnoreResponse,
    /// No documentation provided.
    pub is_following: bool,
}
