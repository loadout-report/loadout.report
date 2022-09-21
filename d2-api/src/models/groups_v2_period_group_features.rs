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
pub struct GroupsV2PeriodGroupFeatures {
    #[serde(rename = "maximumMembers", skip_serializing_if = "Option::is_none")]
    pub maximum_members: Option<i32>,
    /// Maximum number of groups of this type a typical membership may join. For example, a user may join about 50 General groups with their Bungie.net account. They may join one clan per Destiny membership.
    #[serde(rename = "maximumMembershipsOfGroupType", skip_serializing_if = "Option::is_none")]
    pub maximum_memberships_of_group_type: Option<i32>,
    #[serde(rename = "capabilities", skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<i32>,
    #[serde(rename = "membershipTypes", skip_serializing_if = "Option::is_none")]
    pub membership_types: Option<Vec<i32>>,
    /// Minimum Member Level allowed to invite new members to group  Always Allowed: Founder, Acting Founder  True means admins have this power, false means they don't  Default is false for clans, true for groups.
    #[serde(rename = "invitePermissionOverride", skip_serializing_if = "Option::is_none")]
    pub invite_permission_override: Option<bool>,
    /// Minimum Member Level allowed to update group culture  Always Allowed: Founder, Acting Founder  True means admins have this power, false means they don't  Default is false for clans, true for groups.
    #[serde(rename = "updateCulturePermissionOverride", skip_serializing_if = "Option::is_none")]
    pub update_culture_permission_override: Option<bool>,
    /// Minimum Member Level allowed to host guided games  Always Allowed: Founder, Acting Founder, Admin  Allowed Overrides: None, Member, Beginner  Default is Member for clans, None for groups, although this means nothing for groups.
    #[serde(rename = "hostGuidedGamePermissionOverride", skip_serializing_if = "Option::is_none")]
    pub host_guided_game_permission_override: Option<i32>,
    /// Minimum Member Level allowed to update banner  Always Allowed: Founder, Acting Founder  True means admins have this power, false means they don't  Default is false for clans, true for groups.
    #[serde(rename = "updateBannerPermissionOverride", skip_serializing_if = "Option::is_none")]
    pub update_banner_permission_override: Option<bool>,
    /// Level to join a member at when accepting an invite, application, or joining an open clan  Default is Beginner.
    #[serde(rename = "joinLevel", skip_serializing_if = "Option::is_none")]
    pub join_level: Option<i32>,
}

impl GroupsV2PeriodGroupFeatures {
    pub fn new() -> GroupsV2PeriodGroupFeatures {
        GroupsV2PeriodGroupFeatures {
            maximum_members: None,
            maximum_memberships_of_group_type: None,
            capabilities: None,
            membership_types: None,
            invite_permission_override: None,
            update_culture_permission_override: None,
            host_guided_game_permission_override: None,
            update_banner_permission_override: None,
            join_level: None,
        }
    }
}


