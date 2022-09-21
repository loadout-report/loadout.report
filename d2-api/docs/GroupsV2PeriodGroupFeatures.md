# GroupsV2PeriodGroupFeatures

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**maximum_members** | Option<**i32**> |  | [optional]
**maximum_memberships_of_group_type** | Option<**i32**> | Maximum number of groups of this type a typical membership may join. For example, a user may join about 50 General groups with their Bungie.net account. They may join one clan per Destiny membership. | [optional]
**capabilities** | Option<**i32**> |  | [optional]
**membership_types** | Option<**Vec<i32>**> |  | [optional]
**invite_permission_override** | Option<**bool**> | Minimum Member Level allowed to invite new members to group  Always Allowed: Founder, Acting Founder  True means admins have this power, false means they don't  Default is false for clans, true for groups. | [optional]
**update_culture_permission_override** | Option<**bool**> | Minimum Member Level allowed to update group culture  Always Allowed: Founder, Acting Founder  True means admins have this power, false means they don't  Default is false for clans, true for groups. | [optional]
**host_guided_game_permission_override** | Option<**i32**> | Minimum Member Level allowed to host guided games  Always Allowed: Founder, Acting Founder, Admin  Allowed Overrides: None, Member, Beginner  Default is Member for clans, None for groups, although this means nothing for groups. | [optional]
**update_banner_permission_override** | Option<**bool**> | Minimum Member Level allowed to update banner  Always Allowed: Founder, Acting Founder  True means admins have this power, false means they don't  Default is false for clans, true for groups. | [optional]
**join_level** | Option<**i32**> | Level to join a member at when accepting an invite, application, or joining an open clan  Default is Beginner. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


