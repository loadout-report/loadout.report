# \PreviewApi

All URIs are relative to *https://www.bungie.net/Platform*

Method | HTTP request | Description
------------- | ------------- | -------------
[**destiny2_period_get_clan_aggregate_stats**](PreviewApi.md#destiny2_period_get_clan_aggregate_stats) | **GET** /Destiny2/Stats/AggregateClanStats/{groupId}/ | 
[**destiny2_period_get_clan_leaderboards**](PreviewApi.md#destiny2_period_get_clan_leaderboards) | **GET** /Destiny2/Stats/Leaderboards/Clans/{groupId}/ | 
[**destiny2_period_get_leaderboards**](PreviewApi.md#destiny2_period_get_leaderboards) | **GET** /Destiny2/{membershipType}/Account/{destinyMembershipId}/Stats/Leaderboards/ | 
[**destiny2_period_get_leaderboards_for_character**](PreviewApi.md#destiny2_period_get_leaderboards_for_character) | **GET** /Destiny2/Stats/Leaderboards/{membershipType}/{destinyMembershipId}/{characterId}/ | 
[**destiny2_period_get_public_vendors**](PreviewApi.md#destiny2_period_get_public_vendors) | **GET** /Destiny2/Vendors/ | 
[**destiny2_period_insert_socket_plug**](PreviewApi.md#destiny2_period_insert_socket_plug) | **POST** /Destiny2/Actions/Items/InsertSocketPlug/ | 
[**destiny2_period_insert_socket_plug_free**](PreviewApi.md#destiny2_period_insert_socket_plug_free) | **POST** /Destiny2/Actions/Items/InsertSocketPlugFree/ | 



## destiny2_period_get_clan_aggregate_stats

> crate::models::Destiny2GetClanAggregateStats200Response destiny2_period_get_clan_aggregate_stats(group_id, modes)


Gets aggregated stats for a clan using the same categories as the clan leaderboards. PREVIEW: This endpoint is still in beta, and may experience rough edges. The schema is in final form, but there may be bugs that prevent desirable operation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **i64** | Group ID of the clan whose leaderboards you wish to fetch. | [required] |
**modes** | Option<**String**> | List of game modes for which to get leaderboards. See the documentation for DestinyActivityModeType for valid values, and pass in string representation, comma delimited. |  |

### Return type

[**crate::models::Destiny2GetClanAggregateStats200Response**](Destiny2_GetClanAggregateStats_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## destiny2_period_get_clan_leaderboards

> crate::models::Destiny2GetClanLeaderboards200Response destiny2_period_get_clan_leaderboards(group_id, maxtop, modes, statid)


Gets leaderboards with the signed in user's friends and the supplied destinyMembershipId as the focus. PREVIEW: This endpoint is still in beta, and may experience rough edges. The schema is in final form, but there may be bugs that prevent desirable operation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **i64** | Group ID of the clan whose leaderboards you wish to fetch. | [required] |
**maxtop** | Option<**i32**> | Maximum number of top players to return. Use a large number to get entire leaderboard. |  |
**modes** | Option<**String**> | List of game modes for which to get leaderboards. See the documentation for DestinyActivityModeType for valid values, and pass in string representation, comma delimited. |  |
**statid** | Option<**String**> | ID of stat to return rather than returning all Leaderboard stats. |  |

### Return type

[**crate::models::Destiny2GetClanLeaderboards200Response**](Destiny2_GetClanLeaderboards_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## destiny2_period_get_leaderboards

> crate::models::Destiny2GetClanLeaderboards200Response destiny2_period_get_leaderboards(destiny_membership_id, membership_type, maxtop, modes, statid)


Gets leaderboards with the signed in user's friends and the supplied destinyMembershipId as the focus. PREVIEW: This endpoint has not yet been implemented. It is being returned for a preview of future functionality, and for public comment/suggestion/preparation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**destiny_membership_id** | **i64** | The Destiny membershipId of the user to retrieve. | [required] |
**membership_type** | **i32** | A valid non-BungieNet membership type. | [required] |
**maxtop** | Option<**i32**> | Maximum number of top players to return. Use a large number to get entire leaderboard. |  |
**modes** | Option<**String**> | List of game modes for which to get leaderboards. See the documentation for DestinyActivityModeType for valid values, and pass in string representation, comma delimited. |  |
**statid** | Option<**String**> | ID of stat to return rather than returning all Leaderboard stats. |  |

### Return type

[**crate::models::Destiny2GetClanLeaderboards200Response**](Destiny2_GetClanLeaderboards_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## destiny2_period_get_leaderboards_for_character

> crate::models::Destiny2GetClanLeaderboards200Response destiny2_period_get_leaderboards_for_character(character_id, destiny_membership_id, membership_type, maxtop, modes, statid)


Gets leaderboards with the signed in user's friends and the supplied destinyMembershipId as the focus. PREVIEW: This endpoint is still in beta, and may experience rough edges. The schema is in final form, but there may be bugs that prevent desirable operation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**character_id** | **i64** | The specific character to build the leaderboard around for the provided Destiny Membership. | [required] |
**destiny_membership_id** | **i64** | The Destiny membershipId of the user to retrieve. | [required] |
**membership_type** | **i32** | A valid non-BungieNet membership type. | [required] |
**maxtop** | Option<**i32**> | Maximum number of top players to return. Use a large number to get entire leaderboard. |  |
**modes** | Option<**String**> | List of game modes for which to get leaderboards. See the documentation for DestinyActivityModeType for valid values, and pass in string representation, comma delimited. |  |
**statid** | Option<**String**> | ID of stat to return rather than returning all Leaderboard stats. |  |

### Return type

[**crate::models::Destiny2GetClanLeaderboards200Response**](Destiny2_GetClanLeaderboards_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## destiny2_period_get_public_vendors

> crate::models::Destiny2GetPublicVendors200Response destiny2_period_get_public_vendors(components)


Get items available from vendors where the vendors have items for sale that are common for everyone. If any portion of the Vendor's available inventory is character or account specific, we will be unable to return their data from this endpoint due to the way that available inventory is computed. As I am often guilty of saying: 'It's a long story...'

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**components** | Option<[**Vec<i32>**](i32.md)> | A comma separated list of components to return (as strings or numeric values). See the DestinyComponentType enum for valid components to request. You must request at least one component to receive results. |  |

### Return type

[**crate::models::Destiny2GetPublicVendors200Response**](Destiny2_GetPublicVendors_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## destiny2_period_insert_socket_plug

> crate::models::Destiny2InsertSocketPlug200Response destiny2_period_insert_socket_plug()


Insert a plug into a socketed item. I know how it sounds, but I assure you it's much more G-rated than you might be guessing. We haven't decided yet whether this will be able to insert plugs that have side effects, but if we do it will require special scope permission for an application attempting to do so. You must have a valid Destiny Account, and either be in a social space, in orbit, or offline. Request must include proof of permission for 'InsertPlugs' from the account owner.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::Destiny2InsertSocketPlug200Response**](Destiny2_InsertSocketPlug_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## destiny2_period_insert_socket_plug_free

> crate::models::Destiny2InsertSocketPlug200Response destiny2_period_insert_socket_plug_free()


Insert a 'free' plug into an item's socket. This does not require 'Advanced Write Action' authorization and is available to 3rd-party apps, but will only work on 'free and reversible' socket actions (Perks, Armor Mods, Shaders, Ornaments, etc.). You must have a valid Destiny Account, and the character must either be in a social space, in orbit, or offline.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::Destiny2InsertSocketPlug200Response**](Destiny2_InsertSocketPlug_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

