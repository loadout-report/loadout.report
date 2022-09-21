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
pub struct GroupsV2PeriodGroupOptionsEditAction {
    /// Minimum Member Level allowed to invite new members to group  Always Allowed: Founder, Acting Founder  True means admins have this power, false means they don't  Default is false for clans, true for groups.
    #[serde(rename = "InvitePermissionOverride", skip_serializing_if = "Option::is_none")]
    pub invite_permission_override: Option<bool>,
    /// Minimum Member Level allowed to update group culture  Always Allowed: Founder, Acting Founder  True means admins have this power, false means they don't  Default is false for clans, true for groups.
    #[serde(rename = "UpdateCulturePermissionOverride", skip_serializing_if = "Option::is_none")]
    pub update_culture_permission_override: Option<bool>,
    /// Minimum Member Level allowed to host guided games  Always Allowed: Founder, Acting Founder, Admin  Allowed Overrides: None, Member, Beginner  Default is Member for clans, None for groups, although this means nothing for groups.
    #[serde(rename = "HostGuidedGamePermissionOverride", skip_serializing_if = "Option::is_none")]
    pub host_guided_game_permission_override: Option<HostGuidedGamePermissionOverride>,
    /// Minimum Member Level allowed to update banner  Always Allowed: Founder, Acting Founder  True means admins have this power, false means they don't  Default is false for clans, true for groups.
    #[serde(rename = "UpdateBannerPermissionOverride", skip_serializing_if = "Option::is_none")]
    pub update_banner_permission_override: Option<bool>,
    /// Level to join a member at when accepting an invite, application, or joining an open clan  Default is Beginner.
    #[serde(rename = "JoinLevel", skip_serializing_if = "Option::is_none")]
    pub join_level: Option<JoinLevel>,
}

impl GroupsV2PeriodGroupOptionsEditAction {
    pub fn new() -> GroupsV2PeriodGroupOptionsEditAction {
        GroupsV2PeriodGroupOptionsEditAction {
            invite_permission_override: None,
            update_culture_permission_override: None,
            host_guided_game_permission_override: None,
            update_banner_permission_override: None,
            join_level: None,
        }
    }
}

/// Minimum Member Level allowed to host guided games  Always Allowed: Founder, Acting Founder, Admin  Allowed Overrides: None, Member, Beginner  Default is Member for clans, None for groups, although this means nothing for groups.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum HostGuidedGamePermissionOverride {
    #[serde(rename = "0")]
    Variant0,
    #[serde(rename = "1")]
    Variant1,
    #[serde(rename = "2")]
    Variant2,
}

impl Default for HostGuidedGamePermissionOverride {
    fn default() -> HostGuidedGamePermissionOverride {
        Self::Variant0
    }
}
/// Level to join a member at when accepting an invite, application, or joining an open clan  Default is Beginner.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum JoinLevel {
    #[serde(rename = "0")]
    Variant0,
    #[serde(rename = "1")]
    Variant1,
    #[serde(rename = "2")]
    Variant2,
    #[serde(rename = "3")]
    Variant3,
    #[serde(rename = "4")]
    Variant4,
    #[serde(rename = "5")]
    Variant5,
}

impl Default for JoinLevel {
    fn default() -> JoinLevel {
        Self::Variant0
    }
}

