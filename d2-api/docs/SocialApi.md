# \SocialApi

All URIs are relative to *https://www.bungie.net/Platform*

Method | HTTP request | Description
------------- | ------------- | -------------
[**social_period_accept_friend_request**](SocialApi.md#social_period_accept_friend_request) | **POST** /Social/Friends/Requests/Accept/{membershipId}/ | 
[**social_period_decline_friend_request**](SocialApi.md#social_period_decline_friend_request) | **POST** /Social/Friends/Requests/Decline/{membershipId}/ | 
[**social_period_get_friend_list**](SocialApi.md#social_period_get_friend_list) | **GET** /Social/Friends/ | 
[**social_period_get_friend_request_list**](SocialApi.md#social_period_get_friend_request_list) | **GET** /Social/Friends/Requests/ | 
[**social_period_get_platform_friend_list**](SocialApi.md#social_period_get_platform_friend_list) | **GET** /Social/PlatformFriends/{friendPlatform}/{page}/ | 
[**social_period_issue_friend_request**](SocialApi.md#social_period_issue_friend_request) | **POST** /Social/Friends/Add/{membershipId}/ | 
[**social_period_remove_friend**](SocialApi.md#social_period_remove_friend) | **POST** /Social/Friends/Remove/{membershipId}/ | 
[**social_period_remove_friend_request**](SocialApi.md#social_period_remove_friend_request) | **POST** /Social/Friends/Requests/Remove/{membershipId}/ | 



## social_period_accept_friend_request

> crate::models::GroupV2GetUserClanInviteSetting200Response social_period_accept_friend_request(membership_id)


Accepts a friend relationship with the target user. The user must be on your incoming friend request list, though no error will occur if they are not.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**membership_id** | **String** | The membership id of the user you wish to accept. | [required] |

### Return type

[**crate::models::GroupV2GetUserClanInviteSetting200Response**](GroupV2_GetUserClanInviteSetting_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## social_period_decline_friend_request

> crate::models::GroupV2GetUserClanInviteSetting200Response social_period_decline_friend_request(membership_id)


Declines a friend relationship with the target user. The user must be on your incoming friend request list, though no error will occur if they are not.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**membership_id** | **String** | The membership id of the user you wish to decline. | [required] |

### Return type

[**crate::models::GroupV2GetUserClanInviteSetting200Response**](GroupV2_GetUserClanInviteSetting_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## social_period_get_friend_list

> crate::models::SocialGetFriendList200Response social_period_get_friend_list()


Returns your Bungie Friend list

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::SocialGetFriendList200Response**](Social_GetFriendList_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## social_period_get_friend_request_list

> crate::models::SocialGetFriendRequestList200Response social_period_get_friend_request_list()


Returns your friend request queue.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::SocialGetFriendRequestList200Response**](Social_GetFriendRequestList_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## social_period_get_platform_friend_list

> crate::models::SocialGetPlatformFriendList200Response social_period_get_platform_friend_list(friend_platform, page)


Gets the platform friend of the requested type, with additional information if they have Bungie accounts. Must have a recent login session with said platform.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**friend_platform** | **i32** | The platform friend type. | [required] |
**page** | **String** | The zero based page to return. Page size is 100. | [required] |

### Return type

[**crate::models::SocialGetPlatformFriendList200Response**](Social_GetPlatformFriendList_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## social_period_issue_friend_request

> crate::models::GroupV2GetUserClanInviteSetting200Response social_period_issue_friend_request(membership_id)


Requests a friend relationship with the target user. Any of the target user's linked membership ids are valid inputs.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**membership_id** | **String** | The membership id of the user you wish to add. | [required] |

### Return type

[**crate::models::GroupV2GetUserClanInviteSetting200Response**](GroupV2_GetUserClanInviteSetting_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## social_period_remove_friend

> crate::models::GroupV2GetUserClanInviteSetting200Response social_period_remove_friend(membership_id)


Remove a friend relationship with the target user. The user must be on your friend list, though no error will occur if they are not.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**membership_id** | **String** | The membership id of the user you wish to remove. | [required] |

### Return type

[**crate::models::GroupV2GetUserClanInviteSetting200Response**](GroupV2_GetUserClanInviteSetting_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## social_period_remove_friend_request

> crate::models::GroupV2GetUserClanInviteSetting200Response social_period_remove_friend_request(membership_id)


Remove a friend relationship with the target user. The user must be on your outgoing request friend list, though no error will occur if they are not.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**membership_id** | **String** | The membership id of the user you wish to remove. | [required] |

### Return type

[**crate::models::GroupV2GetUserClanInviteSetting200Response**](GroupV2_GetUserClanInviteSetting_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

