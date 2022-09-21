# GroupsV2PeriodGroupResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**detail** | Option<[**crate::models::GroupsV2PeriodGroupV2**](GroupsV2.GroupV2.md)> |  | [optional]
**founder** | Option<[**crate::models::GroupsV2PeriodGroupMember**](GroupsV2.GroupMember.md)> |  | [optional]
**allied_ids** | Option<**Vec<i64>**> |  | [optional]
**parent_group** | Option<[**crate::models::GroupsV2PeriodGroupV2**](GroupsV2.GroupV2.md)> |  | [optional]
**alliance_status** | Option<**i32**> |  | [optional]
**group_join_invite_count** | Option<**i32**> |  | [optional]
**current_user_memberships_inactive_for_destiny** | Option<**bool**> | A convenience property that indicates if every membership you (the current user) have that is a part of this group are part of an account that is considered inactive - for example, overridden accounts in Cross Save. | [optional]
**current_user_member_map** | Option<[**::std::collections::HashMap<String, crate::models::GroupsV2PeriodGroupMember>**](GroupsV2.GroupMember.md)> | This property will be populated if the authenticated user is a member of the group. Note that because of account linking, a user can sometimes be part of a clan more than once. As such, this returns the highest member type available. | [optional]
**current_user_potential_member_map** | Option<[**::std::collections::HashMap<String, crate::models::GroupsV2PeriodGroupPotentialMember>**](GroupsV2.GroupPotentialMember.md)> | This property will be populated if the authenticated user is an applicant or has an outstanding invitation to join. Note that because of account linking, a user can sometimes be part of a clan more than once. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


