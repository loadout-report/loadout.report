# \UserApi

All URIs are relative to *https://www.bungie.net/Platform*

Method | HTTP request | Description
------------- | ------------- | -------------
[**user_period_get_available_themes**](UserApi.md#user_period_get_available_themes) | **GET** /User/GetAvailableThemes/ | 
[**user_period_get_bungie_net_user_by_id**](UserApi.md#user_period_get_bungie_net_user_by_id) | **GET** /User/GetBungieNetUserById/{id}/ | 
[**user_period_get_credential_types_for_target_account**](UserApi.md#user_period_get_credential_types_for_target_account) | **GET** /User/GetCredentialTypesForTargetAccount/{membershipId}/ | 
[**user_period_get_membership_data_by_id**](UserApi.md#user_period_get_membership_data_by_id) | **GET** /User/GetMembershipsById/{membershipId}/{membershipType}/ | 
[**user_period_get_membership_data_for_current_user**](UserApi.md#user_period_get_membership_data_for_current_user) | **GET** /User/GetMembershipsForCurrentUser/ | 
[**user_period_get_membership_from_hard_linked_credential**](UserApi.md#user_period_get_membership_from_hard_linked_credential) | **GET** /User/GetMembershipFromHardLinkedCredential/{crType}/{credential}/ | 
[**user_period_get_sanitized_platform_display_names**](UserApi.md#user_period_get_sanitized_platform_display_names) | **GET** /User/GetSanitizedPlatformDisplayNames/{membershipId}/ | 
[**user_period_search_by_global_name_post**](UserApi.md#user_period_search_by_global_name_post) | **POST** /User/Search/GlobalName/{page}/ | 
[**user_period_search_by_global_name_prefix**](UserApi.md#user_period_search_by_global_name_prefix) | **GET** /User/Search/Prefix/{displayNamePrefix}/{page}/ | 



## user_period_get_available_themes

> crate::models::UserGetAvailableThemes200Response user_period_get_available_themes()


Returns a list of all available user themes.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::UserGetAvailableThemes200Response**](User_GetAvailableThemes_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_period_get_bungie_net_user_by_id

> crate::models::UserGetBungieNetUserById200Response user_period_get_bungie_net_user_by_id(id)


Loads a bungienet user by membership id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | The requested Bungie.net membership id. | [required] |

### Return type

[**crate::models::UserGetBungieNetUserById200Response**](User_GetBungieNetUserById_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_period_get_credential_types_for_target_account

> crate::models::UserGetCredentialTypesForTargetAccount200Response user_period_get_credential_types_for_target_account(membership_id)


Returns a list of credential types attached to the requested account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**membership_id** | **i64** | The user's membership id | [required] |

### Return type

[**crate::models::UserGetCredentialTypesForTargetAccount200Response**](User_GetCredentialTypesForTargetAccount_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_period_get_membership_data_by_id

> crate::models::UserGetMembershipDataById200Response user_period_get_membership_data_by_id(membership_id, membership_type)


Returns a list of accounts associated with the supplied membership ID and membership type. This will include all linked accounts (even when hidden) if supplied credentials permit it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**membership_id** | **i64** | The membership ID of the target user. | [required] |
**membership_type** | **i32** | Type of the supplied membership ID. | [required] |

### Return type

[**crate::models::UserGetMembershipDataById200Response**](User_GetMembershipDataById_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_period_get_membership_data_for_current_user

> crate::models::UserGetMembershipDataById200Response user_period_get_membership_data_for_current_user()


Returns a list of accounts associated with signed in user. This is useful for OAuth implementations that do not give you access to the token response.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::UserGetMembershipDataById200Response**](User_GetMembershipDataById_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_period_get_membership_from_hard_linked_credential

> crate::models::UserGetMembershipFromHardLinkedCredential200Response user_period_get_membership_from_hard_linked_credential(credential, cr_type)


Gets any hard linked membership given a credential. Only works for credentials that are public (just SteamID64 right now). Cross Save aware.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**credential** | **String** | The credential to look up. Must be a valid SteamID64. | [required] |
**cr_type** | **i32** | The credential type. 'SteamId' is the only valid value at present. | [required] |

### Return type

[**crate::models::UserGetMembershipFromHardLinkedCredential200Response**](User_GetMembershipFromHardLinkedCredential_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_period_get_sanitized_platform_display_names

> crate::models::UserGetSanitizedPlatformDisplayNames200Response user_period_get_sanitized_platform_display_names(membership_id)


Gets a list of all display names linked to this membership id but sanitized (profanity filtered). Obeys all visibility rules of calling user and is heavily cached.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**membership_id** | **i64** | The requested membership id to load. | [required] |

### Return type

[**crate::models::UserGetSanitizedPlatformDisplayNames200Response**](User_GetSanitizedPlatformDisplayNames_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_period_search_by_global_name_post

> crate::models::UserSearchByGlobalNamePrefix200Response user_period_search_by_global_name_post(page)


Given the prefix of a global display name, returns all users who share that name.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | **i32** | The zero-based page of results you desire. | [required] |

### Return type

[**crate::models::UserSearchByGlobalNamePrefix200Response**](User_SearchByGlobalNamePrefix_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_period_search_by_global_name_prefix

> crate::models::UserSearchByGlobalNamePrefix200Response user_period_search_by_global_name_prefix(display_name_prefix, page)


[OBSOLETE] Do not use this to search users, use SearchByGlobalNamePost instead.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**display_name_prefix** | **String** | The display name prefix you're looking for. | [required] |
**page** | **i32** | The zero-based page of results you desire. | [required] |

### Return type

[**crate::models::UserSearchByGlobalNamePrefix200Response**](User_SearchByGlobalNamePrefix_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

