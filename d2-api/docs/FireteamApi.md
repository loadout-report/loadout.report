# \FireteamApi

All URIs are relative to *https://www.bungie.net/Platform*

Method | HTTP request | Description
------------- | ------------- | -------------
[**fireteam_period_get_active_private_clan_fireteam_count**](FireteamApi.md#fireteam_period_get_active_private_clan_fireteam_count) | **GET** /Fireteam/Clan/{groupId}/ActiveCount/ | 
[**fireteam_period_get_available_clan_fireteams**](FireteamApi.md#fireteam_period_get_available_clan_fireteams) | **GET** /Fireteam/Clan/{groupId}/Available/{platform}/{activityType}/{dateRange}/{slotFilter}/{publicOnly}/{page}/ | 
[**fireteam_period_get_clan_fireteam**](FireteamApi.md#fireteam_period_get_clan_fireteam) | **GET** /Fireteam/Clan/{groupId}/Summary/{fireteamId}/ | 
[**fireteam_period_get_my_clan_fireteams**](FireteamApi.md#fireteam_period_get_my_clan_fireteams) | **GET** /Fireteam/Clan/{groupId}/My/{platform}/{includeClosed}/1/ | 
[**fireteam_period_search_public_available_clan_fireteams**](FireteamApi.md#fireteam_period_search_public_available_clan_fireteams) | **GET** /Fireteam/Search/Available/{platform}/{activityType}/{dateRange}/{slotFilter}/{page}/ | 



## fireteam_period_get_active_private_clan_fireteam_count

> crate::models::GroupV2EditGroup200Response fireteam_period_get_active_private_clan_fireteam_count(group_id)


Gets a count of all active non-public fireteams for the specified clan. Maximum value returned is 25.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **i64** | The group id of the clan. | [required] |

### Return type

[**crate::models::GroupV2EditGroup200Response**](GroupV2_EditGroup_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fireteam_period_get_available_clan_fireteams

> crate::models::FireteamGetAvailableClanFireteams200Response fireteam_period_get_available_clan_fireteams(activity_type, date_range, group_id, page, platform, public_only, slot_filter, lang_filter)


Gets a listing of all of this clan's fireteams that are have available slots. Caller is not checked for join criteria so caching is maximized.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**activity_type** | **i32** | The activity type to filter by. | [required] |
**date_range** | **i32** | The date range to grab available fireteams. | [required] |
**group_id** | **i64** | The group id of the clan. | [required] |
**page** | **i32** | Zero based page | [required] |
**platform** | **i32** | The platform filter. | [required] |
**public_only** | **i32** | Determines public/private filtering. | [required] |
**slot_filter** | **i32** | Filters based on available slots | [required] |
**lang_filter** | Option<**String**> | An optional language filter. |  |

### Return type

[**crate::models::FireteamGetAvailableClanFireteams200Response**](Fireteam_GetAvailableClanFireteams_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fireteam_period_get_clan_fireteam

> crate::models::FireteamGetClanFireteam200Response fireteam_period_get_clan_fireteam(fireteam_id, group_id)


Gets a specific fireteam.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fireteam_id** | **i64** | The unique id of the fireteam. | [required] |
**group_id** | **i64** | The group id of the clan. | [required] |

### Return type

[**crate::models::FireteamGetClanFireteam200Response**](Fireteam_GetClanFireteam_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fireteam_period_get_my_clan_fireteams

> crate::models::FireteamGetMyClanFireteams200Response fireteam_period_get_my_clan_fireteams(group_id, include_closed, platform, group_filter, lang_filter)


Gets a listing of all fireteams that caller is an applicant, a member, or an alternate of.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **i64** | The group id of the clan. (This parameter is ignored unless the optional query parameter groupFilter is true). | [required] |
**include_closed** | **bool** | If true, return fireteams that have been closed. | [required] |
**platform** | **i32** | The platform filter. | [required] |
**group_filter** | Option<**bool**> | If true, filter by clan. Otherwise, ignore the clan and show all of the user's fireteams. |  |
**lang_filter** | Option<**String**> | An optional language filter. |  |

### Return type

[**crate::models::FireteamGetMyClanFireteams200Response**](Fireteam_GetMyClanFireteams_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fireteam_period_search_public_available_clan_fireteams

> crate::models::FireteamGetAvailableClanFireteams200Response fireteam_period_search_public_available_clan_fireteams(activity_type, date_range, page, platform, slot_filter, lang_filter)


Gets a listing of all public fireteams starting now with open slots. Caller is not checked for join criteria so caching is maximized.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**activity_type** | **i32** | The activity type to filter by. | [required] |
**date_range** | **i32** | The date range to grab available fireteams. | [required] |
**page** | **i32** | Zero based page | [required] |
**platform** | **i32** | The platform filter. | [required] |
**slot_filter** | **i32** | Filters based on available slots | [required] |
**lang_filter** | Option<**String**> | An optional language filter. |  |

### Return type

[**crate::models::FireteamGetAvailableClanFireteams200Response**](Fireteam_GetAvailableClanFireteams_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

