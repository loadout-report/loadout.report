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
pub struct GroupsV2PeriodGroupResponse {
    #[serde(rename = "detail", skip_serializing_if = "Option::is_none")]
    pub detail: Option<Box<crate::models::GroupsV2PeriodGroupV2>>,
    #[serde(rename = "founder", skip_serializing_if = "Option::is_none")]
    pub founder: Option<Box<crate::models::GroupsV2PeriodGroupMember>>,
    #[serde(rename = "alliedIds", skip_serializing_if = "Option::is_none")]
    pub allied_ids: Option<Vec<i64>>,
    #[serde(rename = "parentGroup", skip_serializing_if = "Option::is_none")]
    pub parent_group: Option<Box<crate::models::GroupsV2PeriodGroupV2>>,
    #[serde(rename = "allianceStatus", skip_serializing_if = "Option::is_none")]
    pub alliance_status: Option<i32>,
    #[serde(rename = "groupJoinInviteCount", skip_serializing_if = "Option::is_none")]
    pub group_join_invite_count: Option<i32>,
    /// A convenience property that indicates if every membership you (the current user) have that is a part of this group are part of an account that is considered inactive - for example, overridden accounts in Cross Save.
    #[serde(rename = "currentUserMembershipsInactiveForDestiny", skip_serializing_if = "Option::is_none")]
    pub current_user_memberships_inactive_for_destiny: Option<bool>,
    /// This property will be populated if the authenticated user is a member of the group. Note that because of account linking, a user can sometimes be part of a clan more than once. As such, this returns the highest member type available.
    #[serde(rename = "currentUserMemberMap", skip_serializing_if = "Option::is_none")]
    pub current_user_member_map: Option<::std::collections::HashMap<String, crate::models::GroupsV2PeriodGroupMember>>,
    /// This property will be populated if the authenticated user is an applicant or has an outstanding invitation to join. Note that because of account linking, a user can sometimes be part of a clan more than once.
    #[serde(rename = "currentUserPotentialMemberMap", skip_serializing_if = "Option::is_none")]
    pub current_user_potential_member_map: Option<::std::collections::HashMap<String, crate::models::GroupsV2PeriodGroupPotentialMember>>,
}

impl GroupsV2PeriodGroupResponse {
    pub fn new() -> GroupsV2PeriodGroupResponse {
        GroupsV2PeriodGroupResponse {
            detail: None,
            founder: None,
            allied_ids: None,
            parent_group: None,
            alliance_status: None,
            group_join_invite_count: None,
            current_user_memberships_inactive_for_destiny: None,
            current_user_member_map: None,
            current_user_potential_member_map: None,
        }
    }
}


