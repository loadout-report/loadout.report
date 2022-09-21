# \GroupV2Api

All URIs are relative to *https://www.bungie.net/Platform*

Method | HTTP request | Description
------------- | ------------- | -------------
[**group_v2_period_abdicate_foundership**](GroupV2Api.md#group_v2_period_abdicate_foundership) | **POST** /GroupV2/{groupId}/Admin/AbdicateFoundership/{membershipType}/{founderIdNew}/ | 
[**group_v2_period_add_optional_conversation**](GroupV2Api.md#group_v2_period_add_optional_conversation) | **POST** /GroupV2/{groupId}/OptionalConversations/Add/ | 
[**group_v2_period_approve_all_pending**](GroupV2Api.md#group_v2_period_approve_all_pending) | **POST** /GroupV2/{groupId}/Members/ApproveAll/ | 
[**group_v2_period_approve_pending**](GroupV2Api.md#group_v2_period_approve_pending) | **POST** /GroupV2/{groupId}/Members/Approve/{membershipType}/{membershipId}/ | 
[**group_v2_period_approve_pending_for_list**](GroupV2Api.md#group_v2_period_approve_pending_for_list) | **POST** /GroupV2/{groupId}/Members/ApproveList/ | 
[**group_v2_period_ban_member**](GroupV2Api.md#group_v2_period_ban_member) | **POST** /GroupV2/{groupId}/Members/{membershipType}/{membershipId}/Ban/ | 
[**group_v2_period_deny_all_pending**](GroupV2Api.md#group_v2_period_deny_all_pending) | **POST** /GroupV2/{groupId}/Members/DenyAll/ | 
[**group_v2_period_deny_pending_for_list**](GroupV2Api.md#group_v2_period_deny_pending_for_list) | **POST** /GroupV2/{groupId}/Members/DenyList/ | 
[**group_v2_period_edit_clan_banner**](GroupV2Api.md#group_v2_period_edit_clan_banner) | **POST** /GroupV2/{groupId}/EditClanBanner/ | 
[**group_v2_period_edit_founder_options**](GroupV2Api.md#group_v2_period_edit_founder_options) | **POST** /GroupV2/{groupId}/EditFounderOptions/ | 
[**group_v2_period_edit_group**](GroupV2Api.md#group_v2_period_edit_group) | **POST** /GroupV2/{groupId}/Edit/ | 
[**group_v2_period_edit_group_membership**](GroupV2Api.md#group_v2_period_edit_group_membership) | **POST** /GroupV2/{groupId}/Members/{membershipType}/{membershipId}/SetMembershipType/{memberType}/ | 
[**group_v2_period_edit_optional_conversation**](GroupV2Api.md#group_v2_period_edit_optional_conversation) | **POST** /GroupV2/{groupId}/OptionalConversations/Edit/{conversationId}/ | 
[**group_v2_period_get_admins_and_founder_of_group**](GroupV2Api.md#group_v2_period_get_admins_and_founder_of_group) | **GET** /GroupV2/{groupId}/AdminsAndFounder/ | 
[**group_v2_period_get_available_avatars**](GroupV2Api.md#group_v2_period_get_available_avatars) | **GET** /GroupV2/GetAvailableAvatars/ | 
[**group_v2_period_get_available_themes**](GroupV2Api.md#group_v2_period_get_available_themes) | **GET** /GroupV2/GetAvailableThemes/ | 
[**group_v2_period_get_banned_members_of_group**](GroupV2Api.md#group_v2_period_get_banned_members_of_group) | **GET** /GroupV2/{groupId}/Banned/ | 
[**group_v2_period_get_group**](GroupV2Api.md#group_v2_period_get_group) | **GET** /GroupV2/{groupId}/ | 
[**group_v2_period_get_group_by_name**](GroupV2Api.md#group_v2_period_get_group_by_name) | **GET** /GroupV2/Name/{groupName}/{groupType}/ | 
[**group_v2_period_get_group_by_name_v2**](GroupV2Api.md#group_v2_period_get_group_by_name_v2) | **POST** /GroupV2/NameV2/ | 
[**group_v2_period_get_group_optional_conversations**](GroupV2Api.md#group_v2_period_get_group_optional_conversations) | **GET** /GroupV2/{groupId}/OptionalConversations/ | 
[**group_v2_period_get_groups_for_member**](GroupV2Api.md#group_v2_period_get_groups_for_member) | **GET** /GroupV2/User/{membershipType}/{membershipId}/{filter}/{groupType}/ | 
[**group_v2_period_get_invited_individuals**](GroupV2Api.md#group_v2_period_get_invited_individuals) | **GET** /GroupV2/{groupId}/Members/InvitedIndividuals/ | 
[**group_v2_period_get_members_of_group**](GroupV2Api.md#group_v2_period_get_members_of_group) | **GET** /GroupV2/{groupId}/Members/ | 
[**group_v2_period_get_pending_memberships**](GroupV2Api.md#group_v2_period_get_pending_memberships) | **GET** /GroupV2/{groupId}/Members/Pending/ | 
[**group_v2_period_get_potential_groups_for_member**](GroupV2Api.md#group_v2_period_get_potential_groups_for_member) | **GET** /GroupV2/User/Potential/{membershipType}/{membershipId}/{filter}/{groupType}/ | 
[**group_v2_period_get_recommended_groups**](GroupV2Api.md#group_v2_period_get_recommended_groups) | **POST** /GroupV2/Recommended/{groupType}/{createDateRange}/ | 
[**group_v2_period_get_user_clan_invite_setting**](GroupV2Api.md#group_v2_period_get_user_clan_invite_setting) | **GET** /GroupV2/GetUserClanInviteSetting/{mType}/ | 
[**group_v2_period_group_search**](GroupV2Api.md#group_v2_period_group_search) | **POST** /GroupV2/Search/ | 
[**group_v2_period_individual_group_invite**](GroupV2Api.md#group_v2_period_individual_group_invite) | **POST** /GroupV2/{groupId}/Members/IndividualInvite/{membershipType}/{membershipId}/ | 
[**group_v2_period_individual_group_invite_cancel**](GroupV2Api.md#group_v2_period_individual_group_invite_cancel) | **POST** /GroupV2/{groupId}/Members/IndividualInviteCancel/{membershipType}/{membershipId}/ | 
[**group_v2_period_kick_member**](GroupV2Api.md#group_v2_period_kick_member) | **POST** /GroupV2/{groupId}/Members/{membershipType}/{membershipId}/Kick/ | 
[**group_v2_period_recover_group_for_founder**](GroupV2Api.md#group_v2_period_recover_group_for_founder) | **GET** /GroupV2/Recover/{membershipType}/{membershipId}/{groupType}/ | 
[**group_v2_period_unban_member**](GroupV2Api.md#group_v2_period_unban_member) | **POST** /GroupV2/{groupId}/Members/{membershipType}/{membershipId}/Unban/ | 



## group_v2_period_abdicate_foundership

> crate::models::GroupV2GetUserClanInviteSetting200Response group_v2_period_abdicate_foundership(founder_id_new, group_id, membership_type)


An administrative method to allow the founder of a group or clan to give up their position to another admin permanently.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**founder_id_new** | **i64** | The new founder for this group. Must already be a group admin. | [required] |
**group_id** | **i64** | The target group id. | [required] |
**membership_type** | **i32** | Membership type of the provided founderIdNew. | [required] |

### Return type

[**crate::models::GroupV2GetUserClanInviteSetting200Response**](GroupV2_GetUserClanInviteSetting_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## group_v2_period_add_optional_conversation

> crate::models::ForumGetTopicForContent200Response group_v2_period_add_optional_conversation(group_id)


Add a new optional conversation/chat channel. Requires admin permissions to the group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **i64** | Group ID of the group to edit. | [required] |

### Return type

[**crate::models::ForumGetTopicForContent200Response**](Forum_GetTopicForContent_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## group_v2_period_approve_all_pending

> crate::models::GroupV2ApproveAllPending200Response group_v2_period_approve_all_pending(group_id)


Approve all of the pending users for the given group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **i64** | ID of the group. | [required] |

### Return type

[**crate::models::GroupV2ApproveAllPending200Response**](GroupV2_ApproveAllPending_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## group_v2_period_approve_pending

> crate::models::GroupV2GetUserClanInviteSetting200Response group_v2_period_approve_pending(group_id, membership_id, membership_type)


Approve the given membershipId to join the group/clan as long as they have applied.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **i64** | ID of the group. | [required] |
**membership_id** | **i64** | The membership id being approved. | [required] |
**membership_type** | **i32** | Membership type of the supplied membership ID. | [required] |

### Return type

[**crate::models::GroupV2GetUserClanInviteSetting200Response**](GroupV2_GetUserClanInviteSetting_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## group_v2_period_approve_pending_for_list

> crate::models::GroupV2ApproveAllPending200Response group_v2_period_approve_pending_for_list(group_id)


Approve all of the pending users for the given group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **i64** | ID of the group. | [required] |

### Return type

[**crate::models::GroupV2ApproveAllPending200Response**](GroupV2_ApproveAllPending_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## group_v2_period_ban_member

> crate::models::GroupV2EditGroup200Response group_v2_period_ban_member(group_id, membership_id, membership_type)


Bans the requested member from the requested group for the specified period of time.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **i64** | Group ID that has the member to ban. | [required] |
**membership_id** | **i64** | Membership ID of the member to ban from the group. | [required] |
**membership_type** | **i32** | Membership type of the provided membership ID. | [required] |

### Return type

[**crate::models::GroupV2EditGroup200Response**](GroupV2_EditGroup_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## group_v2_period_deny_all_pending

> crate::models::GroupV2ApproveAllPending200Response group_v2_period_deny_all_pending(group_id)


Deny all of the pending users for the given group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **i64** | ID of the group. | [required] |

### Return type

[**crate::models::GroupV2ApproveAllPending200Response**](GroupV2_ApproveAllPending_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## group_v2_period_deny_pending_for_list

> crate::models::GroupV2ApproveAllPending200Response group_v2_period_deny_pending_for_list(group_id)


Deny all of the pending users for the given group that match the passed-in .

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **i64** | ID of the group. | [required] |

### Return type

[**crate::models::GroupV2ApproveAllPending200Response**](GroupV2_ApproveAllPending_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## group_v2_period_edit_clan_banner

> crate::models::GroupV2EditGroup200Response group_v2_period_edit_clan_banner(group_id)


Edit an existing group's clan banner. You must have suitable permissions in the group to perform this operation. All fields are required.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **i64** | Group ID of the group to edit. | [required] |

### Return type

[**crate::models::GroupV2EditGroup200Response**](GroupV2_EditGroup_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## group_v2_period_edit_founder_options

> crate::models::GroupV2EditGroup200Response group_v2_period_edit_founder_options(group_id)


Edit group options only available to a founder. You must have suitable permissions in the group to perform this operation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **i64** | Group ID of the group to edit. | [required] |

### Return type

[**crate::models::GroupV2EditGroup200Response**](GroupV2_EditGroup_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## group_v2_period_edit_group

> crate::models::GroupV2EditGroup200Response group_v2_period_edit_group(group_id)


Edit an existing group. You must have suitable permissions in the group to perform this operation. This latest revision will only edit the fields you pass in - pass null for properties you want to leave unaltered.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **i64** | Group ID of the group to edit. | [required] |

### Return type

[**crate::models::GroupV2EditGroup200Response**](GroupV2_EditGroup_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## group_v2_period_edit_group_membership

> crate::models::GroupV2EditGroup200Response group_v2_period_edit_group_membership(group_id, membership_id, membership_type, member_type)


Edit the membership type of a given member. You must have suitable permissions in the group to perform this operation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **i64** | ID of the group to which the member belongs. | [required] |
**membership_id** | **i64** | Membership ID to modify. | [required] |
**membership_type** | **i32** | Membership type of the provide membership ID. | [required] |
**member_type** | **i32** | New membertype for the specified member. | [required] |

### Return type

[**crate::models::GroupV2EditGroup200Response**](GroupV2_EditGroup_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## group_v2_period_edit_optional_conversation

> crate::models::ForumGetTopicForContent200Response group_v2_period_edit_optional_conversation(conversation_id, group_id)


Edit the settings of an optional conversation/chat channel. Requires admin permissions to the group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **i64** | Conversation Id of the channel being edited. | [required] |
**group_id** | **i64** | Group ID of the group to edit. | [required] |

### Return type

[**crate::models::ForumGetTopicForContent200Response**](Forum_GetTopicForContent_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## group_v2_period_get_admins_and_founder_of_group

> crate::models::GroupV2GetMembersOfGroup200Response group_v2_period_get_admins_and_founder_of_group(currentpage, group_id)


Get the list of members in a given group who are of admin level or higher.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**currentpage** | **i32** | Page number (starting with 1). Each page has a fixed size of 50 items per page. | [required] |
**group_id** | **i64** | The ID of the group. | [required] |

### Return type

[**crate::models::GroupV2GetMembersOfGroup200Response**](GroupV2_GetMembersOfGroup_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## group_v2_period_get_available_avatars

> crate::models::UserGetSanitizedPlatformDisplayNames200Response group_v2_period_get_available_avatars()


Returns a list of all available group avatars for the signed-in user.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::UserGetSanitizedPlatformDisplayNames200Response**](User_GetSanitizedPlatformDisplayNames_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## group_v2_period_get_available_themes

> crate::models::GroupV2GetAvailableThemes200Response group_v2_period_get_available_themes()


Returns a list of all available group themes.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GroupV2GetAvailableThemes200Response**](GroupV2_GetAvailableThemes_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## group_v2_period_get_banned_members_of_group

> crate::models::GroupV2GetBannedMembersOfGroup200Response group_v2_period_get_banned_members_of_group(currentpage, group_id)


Get the list of banned members in a given group. Only accessible to group Admins and above. Not applicable to all groups. Check group features.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**currentpage** | **i32** | Page number (starting with 1). Each page has a fixed size of 50 entries. | [required] |
**group_id** | **i64** | Group ID whose banned members you are fetching | [required] |

### Return type

[**crate::models::GroupV2GetBannedMembersOfGroup200Response**](GroupV2_GetBannedMembersOfGroup_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## group_v2_period_get_group

> crate::models::GroupV2GetGroup200Response group_v2_period_get_group(group_id)


Get information about a specific group of the given ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **i64** | Requested group's id. | [required] |

### Return type

[**crate::models::GroupV2GetGroup200Response**](GroupV2_GetGroup_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## group_v2_period_get_group_by_name

> crate::models::GroupV2GetGroup200Response group_v2_period_get_group_by_name(group_name, group_type)


Get information about a specific group with the given name and type.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_name** | **String** | Exact name of the group to find. | [required] |
**group_type** | **i32** | Type of group to find. | [required] |

### Return type

[**crate::models::GroupV2GetGroup200Response**](GroupV2_GetGroup_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## group_v2_period_get_group_by_name_v2

> crate::models::GroupV2GetGroup200Response group_v2_period_get_group_by_name_v2()


Get information about a specific group with the given name and type. The POST version.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GroupV2GetGroup200Response**](GroupV2_GetGroup_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## group_v2_period_get_group_optional_conversations

> crate::models::GroupV2GetGroupOptionalConversations200Response group_v2_period_get_group_optional_conversations(group_id)


Gets a list of available optional conversation channels and their settings.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **i64** | Requested group's id. | [required] |

### Return type

[**crate::models::GroupV2GetGroupOptionalConversations200Response**](GroupV2_GetGroupOptionalConversations_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## group_v2_period_get_groups_for_member

> crate::models::GroupV2GetGroupsForMember200Response group_v2_period_get_groups_for_member(filter, group_type, membership_id, membership_type)


Get information about the groups that a given member has joined.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | **i32** | Filter apply to list of joined groups. | [required] |
**group_type** | **i32** | Type of group the supplied member founded. | [required] |
**membership_id** | **i64** | Membership ID to for which to find founded groups. | [required] |
**membership_type** | **i32** | Membership type of the supplied membership ID. | [required] |

### Return type

[**crate::models::GroupV2GetGroupsForMember200Response**](GroupV2_GetGroupsForMember_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## group_v2_period_get_invited_individuals

> crate::models::GroupV2GetPendingMemberships200Response group_v2_period_get_invited_individuals(currentpage, group_id)


Get the list of users who have been invited into the group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**currentpage** | **i32** | Page number (starting with 1). Each page has a fixed size of 50 items per page. | [required] |
**group_id** | **i64** | ID of the group. | [required] |

### Return type

[**crate::models::GroupV2GetPendingMemberships200Response**](GroupV2_GetPendingMemberships_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## group_v2_period_get_members_of_group

> crate::models::GroupV2GetMembersOfGroup200Response group_v2_period_get_members_of_group(currentpage, group_id, member_type, name_search)


Get the list of members in a given group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**currentpage** | **i32** | Page number (starting with 1). Each page has a fixed size of 50 items per page. | [required] |
**group_id** | **i64** | The ID of the group. | [required] |
**member_type** | Option<**i32**> | Filter out other member types. Use None for all members. |  |
**name_search** | Option<**String**> | The name fragment upon which a search should be executed for members with matching display or unique names. |  |

### Return type

[**crate::models::GroupV2GetMembersOfGroup200Response**](GroupV2_GetMembersOfGroup_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## group_v2_period_get_pending_memberships

> crate::models::GroupV2GetPendingMemberships200Response group_v2_period_get_pending_memberships(currentpage, group_id)


Get the list of users who are awaiting a decision on their application to join a given group. Modified to include application info.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**currentpage** | **i32** | Page number (starting with 1). Each page has a fixed size of 50 items per page. | [required] |
**group_id** | **i64** | ID of the group. | [required] |

### Return type

[**crate::models::GroupV2GetPendingMemberships200Response**](GroupV2_GetPendingMemberships_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## group_v2_period_get_potential_groups_for_member

> crate::models::GroupV2GetPotentialGroupsForMember200Response group_v2_period_get_potential_groups_for_member(filter, group_type, membership_id, membership_type)


Get information about the groups that a given member has applied to or been invited to.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | **i32** | Filter apply to list of potential joined groups. | [required] |
**group_type** | **i32** | Type of group the supplied member applied. | [required] |
**membership_id** | **i64** | Membership ID to for which to find applied groups. | [required] |
**membership_type** | **i32** | Membership type of the supplied membership ID. | [required] |

### Return type

[**crate::models::GroupV2GetPotentialGroupsForMember200Response**](GroupV2_GetPotentialGroupsForMember_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## group_v2_period_get_recommended_groups

> crate::models::GroupV2GetRecommendedGroups200Response group_v2_period_get_recommended_groups(create_date_range, group_type)


Gets groups recommended for you based on the groups to whom those you follow belong.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_date_range** | **i32** | Requested range in which to pull recommended groups | [required] |
**group_type** | **i32** | Type of groups requested | [required] |

### Return type

[**crate::models::GroupV2GetRecommendedGroups200Response**](GroupV2_GetRecommendedGroups_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## group_v2_period_get_user_clan_invite_setting

> crate::models::GroupV2GetUserClanInviteSetting200Response group_v2_period_get_user_clan_invite_setting(m_type)


Gets the state of the user's clan invite preferences for a particular membership type - true if they wish to be invited to clans, false otherwise.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**m_type** | **i32** | The Destiny membership type of the account we wish to access settings. | [required] |

### Return type

[**crate::models::GroupV2GetUserClanInviteSetting200Response**](GroupV2_GetUserClanInviteSetting_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## group_v2_period_group_search

> crate::models::GroupV2GroupSearch200Response group_v2_period_group_search()


Search for Groups.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GroupV2GroupSearch200Response**](GroupV2_GroupSearch_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## group_v2_period_individual_group_invite

> crate::models::GroupV2IndividualGroupInvite200Response group_v2_period_individual_group_invite(group_id, membership_id, membership_type)


Invite a user to join this group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **i64** | ID of the group you would like to join. | [required] |
**membership_id** | **i64** | Membership id of the account being invited. | [required] |
**membership_type** | **i32** | MembershipType of the account being invited. | [required] |

### Return type

[**crate::models::GroupV2IndividualGroupInvite200Response**](GroupV2_IndividualGroupInvite_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## group_v2_period_individual_group_invite_cancel

> crate::models::GroupV2IndividualGroupInvite200Response group_v2_period_individual_group_invite_cancel(group_id, membership_id, membership_type)


Cancels a pending invitation to join a group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **i64** | ID of the group you would like to join. | [required] |
**membership_id** | **i64** | Membership id of the account being cancelled. | [required] |
**membership_type** | **i32** | MembershipType of the account being cancelled. | [required] |

### Return type

[**crate::models::GroupV2IndividualGroupInvite200Response**](GroupV2_IndividualGroupInvite_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## group_v2_period_kick_member

> crate::models::GroupV2KickMember200Response group_v2_period_kick_member(group_id, membership_id, membership_type)


Kick a member from the given group, forcing them to reapply if they wish to re-join the group. You must have suitable permissions in the group to perform this operation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **i64** | Group ID to kick the user from. | [required] |
**membership_id** | **i64** | Membership ID to kick. | [required] |
**membership_type** | **i32** | Membership type of the provided membership ID. | [required] |

### Return type

[**crate::models::GroupV2KickMember200Response**](GroupV2_KickMember_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## group_v2_period_recover_group_for_founder

> crate::models::GroupV2RecoverGroupForFounder200Response group_v2_period_recover_group_for_founder(group_type, membership_id, membership_type)


Allows a founder to manually recover a group they can see in game but not on bungie.net

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_type** | **i32** | Type of group the supplied member founded. | [required] |
**membership_id** | **i64** | Membership ID to for which to find founded groups. | [required] |
**membership_type** | **i32** | Membership type of the supplied membership ID. | [required] |

### Return type

[**crate::models::GroupV2RecoverGroupForFounder200Response**](GroupV2_RecoverGroupForFounder_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## group_v2_period_unban_member

> crate::models::GroupV2EditGroup200Response group_v2_period_unban_member(group_id, membership_id, membership_type)


Unbans the requested member, allowing them to re-apply for membership.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **i64** |  | [required] |
**membership_id** | **i64** | Membership ID of the member to unban from the group | [required] |
**membership_type** | **i32** | Membership type of the provided membership ID. | [required] |

### Return type

[**crate::models::GroupV2EditGroup200Response**](GroupV2_EditGroup_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

