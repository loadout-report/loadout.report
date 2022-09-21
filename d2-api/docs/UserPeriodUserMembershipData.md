# UserPeriodUserMembershipData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**destiny_memberships** | Option<[**Vec<crate::models::GroupsV2PeriodGroupUserInfoCard>**](GroupsV2.GroupUserInfoCard.md)> | this allows you to see destiny memberships that are visible and linked to this account (regardless of whether or not they have characters on the world server) | [optional]
**primary_membership_id** | Option<**i64**> | If this property is populated, it will have the membership ID of the account considered to be \"primary\" in this user's cross save relationship.   If null, this user has no cross save relationship, nor primary account. | [optional]
**bungie_net_user** | Option<[**crate::models::UserPeriodGeneralUser**](User.GeneralUser.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


