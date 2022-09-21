# \Destiny2Api

All URIs are relative to *https://www.bungie.net/Platform*

Method | HTTP request | Description
------------- | ------------- | -------------
[**destiny2_period_awa_get_action_token**](Destiny2Api.md#destiny2_period_awa_get_action_token) | **GET** /Destiny2/Awa/GetActionToken/{correlationId}/ | 
[**destiny2_period_awa_initialize_request**](Destiny2Api.md#destiny2_period_awa_initialize_request) | **POST** /Destiny2/Awa/Initialize/ | 
[**destiny2_period_awa_provide_authorization_result**](Destiny2Api.md#destiny2_period_awa_provide_authorization_result) | **POST** /Destiny2/Awa/AwaProvideAuthorizationResult/ | 
[**destiny2_period_equip_item**](Destiny2Api.md#destiny2_period_equip_item) | **POST** /Destiny2/Actions/Items/EquipItem/ | 
[**destiny2_period_equip_items**](Destiny2Api.md#destiny2_period_equip_items) | **POST** /Destiny2/Actions/Items/EquipItems/ | 
[**destiny2_period_get_activity_history**](Destiny2Api.md#destiny2_period_get_activity_history) | **GET** /Destiny2/{membershipType}/Account/{destinyMembershipId}/Character/{characterId}/Stats/Activities/ | 
[**destiny2_period_get_character**](Destiny2Api.md#destiny2_period_get_character) | **GET** /Destiny2/{membershipType}/Profile/{destinyMembershipId}/Character/{characterId}/ | 
[**destiny2_period_get_clan_aggregate_stats**](Destiny2Api.md#destiny2_period_get_clan_aggregate_stats) | **GET** /Destiny2/Stats/AggregateClanStats/{groupId}/ | 
[**destiny2_period_get_clan_banner_source**](Destiny2Api.md#destiny2_period_get_clan_banner_source) | **GET** /Destiny2/Clan/ClanBannerDictionary/ | 
[**destiny2_period_get_clan_leaderboards**](Destiny2Api.md#destiny2_period_get_clan_leaderboards) | **GET** /Destiny2/Stats/Leaderboards/Clans/{groupId}/ | 
[**destiny2_period_get_clan_weekly_reward_state**](Destiny2Api.md#destiny2_period_get_clan_weekly_reward_state) | **GET** /Destiny2/Clan/{groupId}/WeeklyRewardState/ | 
[**destiny2_period_get_collectible_node_details**](Destiny2Api.md#destiny2_period_get_collectible_node_details) | **GET** /Destiny2/{membershipType}/Profile/{destinyMembershipId}/Character/{characterId}/Collectibles/{collectiblePresentationNodeHash}/ | 
[**destiny2_period_get_destiny_aggregate_activity_stats**](Destiny2Api.md#destiny2_period_get_destiny_aggregate_activity_stats) | **GET** /Destiny2/{membershipType}/Account/{destinyMembershipId}/Character/{characterId}/Stats/AggregateActivityStats/ | 
[**destiny2_period_get_destiny_entity_definition**](Destiny2Api.md#destiny2_period_get_destiny_entity_definition) | **GET** /Destiny2/Manifest/{entityType}/{hashIdentifier}/ | 
[**destiny2_period_get_destiny_manifest**](Destiny2Api.md#destiny2_period_get_destiny_manifest) | **GET** /Destiny2/Manifest/ | 
[**destiny2_period_get_historical_stats**](Destiny2Api.md#destiny2_period_get_historical_stats) | **GET** /Destiny2/{membershipType}/Account/{destinyMembershipId}/Character/{characterId}/Stats/ | 
[**destiny2_period_get_historical_stats_definition**](Destiny2Api.md#destiny2_period_get_historical_stats_definition) | **GET** /Destiny2/Stats/Definition/ | 
[**destiny2_period_get_historical_stats_for_account**](Destiny2Api.md#destiny2_period_get_historical_stats_for_account) | **GET** /Destiny2/{membershipType}/Account/{destinyMembershipId}/Stats/ | 
[**destiny2_period_get_item**](Destiny2Api.md#destiny2_period_get_item) | **GET** /Destiny2/{membershipType}/Profile/{destinyMembershipId}/Item/{itemInstanceId}/ | 
[**destiny2_period_get_leaderboards**](Destiny2Api.md#destiny2_period_get_leaderboards) | **GET** /Destiny2/{membershipType}/Account/{destinyMembershipId}/Stats/Leaderboards/ | 
[**destiny2_period_get_leaderboards_for_character**](Destiny2Api.md#destiny2_period_get_leaderboards_for_character) | **GET** /Destiny2/Stats/Leaderboards/{membershipType}/{destinyMembershipId}/{characterId}/ | 
[**destiny2_period_get_linked_profiles**](Destiny2Api.md#destiny2_period_get_linked_profiles) | **GET** /Destiny2/{membershipType}/Profile/{membershipId}/LinkedProfiles/ | 
[**destiny2_period_get_post_game_carnage_report**](Destiny2Api.md#destiny2_period_get_post_game_carnage_report) | **GET** /Destiny2/Stats/PostGameCarnageReport/{activityId}/ | 
[**destiny2_period_get_profile**](Destiny2Api.md#destiny2_period_get_profile) | **GET** /Destiny2/{membershipType}/Profile/{destinyMembershipId}/ | 
[**destiny2_period_get_public_milestone_content**](Destiny2Api.md#destiny2_period_get_public_milestone_content) | **GET** /Destiny2/Milestones/{milestoneHash}/Content/ | 
[**destiny2_period_get_public_milestones**](Destiny2Api.md#destiny2_period_get_public_milestones) | **GET** /Destiny2/Milestones/ | 
[**destiny2_period_get_public_vendors**](Destiny2Api.md#destiny2_period_get_public_vendors) | **GET** /Destiny2/Vendors/ | 
[**destiny2_period_get_unique_weapon_history**](Destiny2Api.md#destiny2_period_get_unique_weapon_history) | **GET** /Destiny2/{membershipType}/Account/{destinyMembershipId}/Character/{characterId}/Stats/UniqueWeapons/ | 
[**destiny2_period_get_vendor**](Destiny2Api.md#destiny2_period_get_vendor) | **GET** /Destiny2/{membershipType}/Profile/{destinyMembershipId}/Character/{characterId}/Vendors/{vendorHash}/ | 
[**destiny2_period_get_vendors**](Destiny2Api.md#destiny2_period_get_vendors) | **GET** /Destiny2/{membershipType}/Profile/{destinyMembershipId}/Character/{characterId}/Vendors/ | 
[**destiny2_period_insert_socket_plug**](Destiny2Api.md#destiny2_period_insert_socket_plug) | **POST** /Destiny2/Actions/Items/InsertSocketPlug/ | 
[**destiny2_period_insert_socket_plug_free**](Destiny2Api.md#destiny2_period_insert_socket_plug_free) | **POST** /Destiny2/Actions/Items/InsertSocketPlugFree/ | 
[**destiny2_period_pull_from_postmaster**](Destiny2Api.md#destiny2_period_pull_from_postmaster) | **POST** /Destiny2/Actions/Items/PullFromPostmaster/ | 
[**destiny2_period_report_offensive_post_game_carnage_report_player**](Destiny2Api.md#destiny2_period_report_offensive_post_game_carnage_report_player) | **POST** /Destiny2/Stats/PostGameCarnageReport/{activityId}/Report/ | 
[**destiny2_period_search_destiny_entities**](Destiny2Api.md#destiny2_period_search_destiny_entities) | **GET** /Destiny2/Armory/Search/{type}/{searchTerm}/ | 
[**destiny2_period_search_destiny_player_by_bungie_name**](Destiny2Api.md#destiny2_period_search_destiny_player_by_bungie_name) | **POST** /Destiny2/SearchDestinyPlayerByBungieName/{membershipType}/ | 
[**destiny2_period_set_item_lock_state**](Destiny2Api.md#destiny2_period_set_item_lock_state) | **POST** /Destiny2/Actions/Items/SetLockState/ | 
[**destiny2_period_set_quest_tracked_state**](Destiny2Api.md#destiny2_period_set_quest_tracked_state) | **POST** /Destiny2/Actions/Items/SetTrackedState/ | 
[**destiny2_period_transfer_item**](Destiny2Api.md#destiny2_period_transfer_item) | **POST** /Destiny2/Actions/Items/TransferItem/ | 



## destiny2_period_awa_get_action_token

> crate::models::Destiny2AwaGetActionToken200Response destiny2_period_awa_get_action_token(correlation_id)


Returns the action token if user approves the request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**correlation_id** | **String** | The identifier for the advanced write action request. | [required] |

### Return type

[**crate::models::Destiny2AwaGetActionToken200Response**](Destiny2_AwaGetActionToken_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## destiny2_period_awa_initialize_request

> crate::models::Destiny2AwaInitializeRequest200Response destiny2_period_awa_initialize_request()


Initialize a request to perform an advanced write action.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::Destiny2AwaInitializeRequest200Response**](Destiny2_AwaInitializeRequest_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## destiny2_period_awa_provide_authorization_result

> crate::models::GroupV2EditGroup200Response destiny2_period_awa_provide_authorization_result()


Provide the result of the user interaction. Called by the Bungie Destiny App to approve or reject a request.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GroupV2EditGroup200Response**](GroupV2_EditGroup_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## destiny2_period_equip_item

> crate::models::GroupV2EditGroup200Response destiny2_period_equip_item()


Equip an item. You must have a valid Destiny Account, and either be in a social space, in orbit, or offline.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GroupV2EditGroup200Response**](GroupV2_EditGroup_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## destiny2_period_equip_items

> crate::models::Destiny2EquipItems200Response destiny2_period_equip_items()


Equip a list of items by itemInstanceIds. You must have a valid Destiny Account, and either be in a social space, in orbit, or offline. Any items not found on your character will be ignored.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::Destiny2EquipItems200Response**](Destiny2_EquipItems_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## destiny2_period_get_activity_history

> crate::models::Destiny2GetActivityHistory200Response destiny2_period_get_activity_history(character_id, destiny_membership_id, membership_type, count, mode, page)


Gets activity history stats for indicated character.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**character_id** | **i64** | The id of the character to retrieve. | [required] |
**destiny_membership_id** | **i64** | The Destiny membershipId of the user to retrieve. | [required] |
**membership_type** | **i32** | A valid non-BungieNet membership type. | [required] |
**count** | Option<**i32**> | Number of rows to return |  |
**mode** | Option<**i32**> | A filter for the activity mode to be returned. None returns all activities. See the documentation for DestinyActivityModeType for valid values, and pass in string representation. |  |
**page** | Option<**i32**> | Page number to return, starting with 0. |  |

### Return type

[**crate::models::Destiny2GetActivityHistory200Response**](Destiny2_GetActivityHistory_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## destiny2_period_get_character

> crate::models::Destiny2GetCharacter200Response destiny2_period_get_character(character_id, destiny_membership_id, membership_type, components)


Returns character information for the supplied character.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**character_id** | **i64** | ID of the character. | [required] |
**destiny_membership_id** | **i64** | Destiny membership ID. | [required] |
**membership_type** | **i32** | A valid non-BungieNet membership type. | [required] |
**components** | Option<[**Vec<i32>**](i32.md)> | A comma separated list of components to return (as strings or numeric values). See the DestinyComponentType enum for valid components to request. You must request at least one component to receive results. |  |

### Return type

[**crate::models::Destiny2GetCharacter200Response**](Destiny2_GetCharacter_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


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


## destiny2_period_get_clan_banner_source

> crate::models::Destiny2GetClanBannerSource200Response destiny2_period_get_clan_banner_source()


Returns the dictionary of values for the Clan Banner

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::Destiny2GetClanBannerSource200Response**](Destiny2_GetClanBannerSource_200_response.md)

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


## destiny2_period_get_clan_weekly_reward_state

> crate::models::Destiny2GetClanWeeklyRewardState200Response destiny2_period_get_clan_weekly_reward_state(group_id)


Returns information on the weekly clan rewards and if the clan has earned them or not. Note that this will always report rewards as not redeemed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **i64** | A valid group id of clan. | [required] |

### Return type

[**crate::models::Destiny2GetClanWeeklyRewardState200Response**](Destiny2_GetClanWeeklyRewardState_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## destiny2_period_get_collectible_node_details

> crate::models::Destiny2GetCollectibleNodeDetails200Response destiny2_period_get_collectible_node_details(character_id, collectible_presentation_node_hash, destiny_membership_id, membership_type, components)


Given a Presentation Node that has Collectibles as direct descendants, this will return item details about those descendants in the context of the requesting character.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**character_id** | **i64** | The Destiny Character ID of the character for whom we're getting collectible detail info. | [required] |
**collectible_presentation_node_hash** | **i32** | The hash identifier of the Presentation Node for whom we should return collectible details. Details will only be returned for collectibles that are direct descendants of this node. | [required] |
**destiny_membership_id** | **i64** | Destiny membership ID of another user. You may be denied. | [required] |
**membership_type** | **i32** | A valid non-BungieNet membership type. | [required] |
**components** | Option<[**Vec<i32>**](i32.md)> | A comma separated list of components to return (as strings or numeric values). See the DestinyComponentType enum for valid components to request. You must request at least one component to receive results. |  |

### Return type

[**crate::models::Destiny2GetCollectibleNodeDetails200Response**](Destiny2_GetCollectibleNodeDetails_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## destiny2_period_get_destiny_aggregate_activity_stats

> crate::models::Destiny2GetDestinyAggregateActivityStats200Response destiny2_period_get_destiny_aggregate_activity_stats(character_id, destiny_membership_id, membership_type)


Gets all activities the character has participated in together with aggregate statistics for those activities.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**character_id** | **i64** | The specific character whose activities should be returned. | [required] |
**destiny_membership_id** | **i64** | The Destiny membershipId of the user to retrieve. | [required] |
**membership_type** | **i32** | A valid non-BungieNet membership type. | [required] |

### Return type

[**crate::models::Destiny2GetDestinyAggregateActivityStats200Response**](Destiny2_GetDestinyAggregateActivityStats_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## destiny2_period_get_destiny_entity_definition

> crate::models::Destiny2GetDestinyEntityDefinition200Response destiny2_period_get_destiny_entity_definition(entity_type, hash_identifier)


Returns the static definition of an entity of the given Type and hash identifier. Examine the API Documentation for the Type Names of entities that have their own definitions. Note that the return type will always *inherit from* DestinyDefinition, but the specific type returned will be the requested entity type if it can be found. Please don't use this as a chatty alternative to the Manifest database if you require large sets of data, but for simple and one-off accesses this should be handy.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**entity_type** | **String** | The type of entity for whom you would like results. These correspond to the entity's definition contract name. For instance, if you are looking for items, this property should be 'DestinyInventoryItemDefinition'. PREVIEW: This endpoint is still in beta, and may experience rough edges. The schema is tentatively in final form, but there may be bugs that prevent desirable operation. | [required] |
**hash_identifier** | **i32** | The hash identifier for the specific Entity you want returned. | [required] |

### Return type

[**crate::models::Destiny2GetDestinyEntityDefinition200Response**](Destiny2_GetDestinyEntityDefinition_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## destiny2_period_get_destiny_manifest

> crate::models::Destiny2GetDestinyManifest200Response destiny2_period_get_destiny_manifest()


Returns the current version of the manifest as a json object.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::Destiny2GetDestinyManifest200Response**](Destiny2_GetDestinyManifest_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## destiny2_period_get_historical_stats

> crate::models::Destiny2GetHistoricalStats200Response destiny2_period_get_historical_stats(character_id, destiny_membership_id, membership_type, dayend, daystart, groups, modes, period_type)


Gets historical stats for indicated character.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**character_id** | **i64** | The id of the character to retrieve. You can omit this character ID or set it to 0 to get aggregate stats across all characters. | [required] |
**destiny_membership_id** | **i64** | The Destiny membershipId of the user to retrieve. | [required] |
**membership_type** | **i32** | A valid non-BungieNet membership type. | [required] |
**dayend** | Option<**String**> | Last day to return when daily stats are requested. Use the format YYYY-MM-DD. Currently, we cannot allow more than 31 days of daily data to be requested in a single request. |  |
**daystart** | Option<**String**> | First day to return when daily stats are requested. Use the format YYYY-MM-DD. Currently, we cannot allow more than 31 days of daily data to be requested in a single request. |  |
**groups** | Option<[**Vec<i32>**](i32.md)> | Group of stats to include, otherwise only general stats are returned. Comma separated list is allowed. Values: General, Weapons, Medals |  |
**modes** | Option<[**Vec<i32>**](i32.md)> | Game modes to return. See the documentation for DestinyActivityModeType for valid values, and pass in string representation, comma delimited. |  |
**period_type** | Option<**i32**> | Indicates a specific period type to return. Optional. May be: Daily, AllTime, or Activity |  |

### Return type

[**crate::models::Destiny2GetHistoricalStats200Response**](Destiny2_GetHistoricalStats_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## destiny2_period_get_historical_stats_definition

> crate::models::Destiny2GetHistoricalStatsDefinition200Response destiny2_period_get_historical_stats_definition()


Gets historical stats definitions.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::Destiny2GetHistoricalStatsDefinition200Response**](Destiny2_GetHistoricalStatsDefinition_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## destiny2_period_get_historical_stats_for_account

> crate::models::Destiny2GetHistoricalStatsForAccount200Response destiny2_period_get_historical_stats_for_account(destiny_membership_id, membership_type, groups)


Gets aggregate historical stats organized around each character for a given account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**destiny_membership_id** | **i64** | The Destiny membershipId of the user to retrieve. | [required] |
**membership_type** | **i32** | A valid non-BungieNet membership type. | [required] |
**groups** | Option<[**Vec<i32>**](i32.md)> | Groups of stats to include, otherwise only general stats are returned. Comma separated list is allowed. Values: General, Weapons, Medals. |  |

### Return type

[**crate::models::Destiny2GetHistoricalStatsForAccount200Response**](Destiny2_GetHistoricalStatsForAccount_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## destiny2_period_get_item

> crate::models::Destiny2GetItem200Response destiny2_period_get_item(destiny_membership_id, item_instance_id, membership_type, components)


Retrieve the details of an instanced Destiny Item. An instanced Destiny item is one with an ItemInstanceId. Non-instanced items, such as materials, have no useful instance-specific details and thus are not queryable here.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**destiny_membership_id** | **i64** | The membership ID of the destiny profile. | [required] |
**item_instance_id** | **i64** | The Instance ID of the destiny item. | [required] |
**membership_type** | **i32** | A valid non-BungieNet membership type. | [required] |
**components** | Option<[**Vec<i32>**](i32.md)> | A comma separated list of components to return (as strings or numeric values). See the DestinyComponentType enum for valid components to request. You must request at least one component to receive results. |  |

### Return type

[**crate::models::Destiny2GetItem200Response**](Destiny2_GetItem_200_response.md)

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


## destiny2_period_get_linked_profiles

> crate::models::Destiny2GetLinkedProfiles200Response destiny2_period_get_linked_profiles(membership_id, membership_type, get_all_memberships)


Returns a summary information about all profiles linked to the requesting membership type/membership ID that have valid Destiny information. The passed-in Membership Type/Membership ID may be a Bungie.Net membership or a Destiny membership. It only returns the minimal amount of data to begin making more substantive requests, but will hopefully serve as a useful alternative to UserServices for people who just care about Destiny data. Note that it will only return linked accounts whose linkages you are allowed to view.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**membership_id** | **i64** | The ID of the membership whose linked Destiny accounts you want returned. Make sure your membership ID matches its Membership Type: don't pass us a PSN membership ID and the XBox membership type, it's not going to work! | [required] |
**membership_type** | **i32** | The type for the membership whose linked Destiny accounts you want returned. | [required] |
**get_all_memberships** | Option<**bool**> | (optional) if set to 'true', all memberships regardless of whether they're obscured by overrides will be returned. Normal privacy restrictions on account linking will still apply no matter what. |  |

### Return type

[**crate::models::Destiny2GetLinkedProfiles200Response**](Destiny2_GetLinkedProfiles_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## destiny2_period_get_post_game_carnage_report

> crate::models::Destiny2GetPostGameCarnageReport200Response destiny2_period_get_post_game_carnage_report(activity_id)


Gets the available post game carnage report for the activity ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**activity_id** | **i64** | The ID of the activity whose PGCR is requested. | [required] |

### Return type

[**crate::models::Destiny2GetPostGameCarnageReport200Response**](Destiny2_GetPostGameCarnageReport_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## destiny2_period_get_profile

> crate::models::Destiny2GetProfile200Response destiny2_period_get_profile(destiny_membership_id, membership_type, components)


Returns Destiny Profile information for the supplied membership.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**destiny_membership_id** | **i64** | Destiny membership ID. | [required] |
**membership_type** | **i32** | A valid non-BungieNet membership type. | [required] |
**components** | Option<[**Vec<i32>**](i32.md)> | A comma separated list of components to return (as strings or numeric values). See the DestinyComponentType enum for valid components to request. You must request at least one component to receive results. |  |

### Return type

[**crate::models::Destiny2GetProfile200Response**](Destiny2_GetProfile_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## destiny2_period_get_public_milestone_content

> crate::models::Destiny2GetPublicMilestoneContent200Response destiny2_period_get_public_milestone_content(milestone_hash)


Gets custom localized content for the milestone of the given hash, if it exists.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**milestone_hash** | **i32** | The identifier for the milestone to be returned. | [required] |

### Return type

[**crate::models::Destiny2GetPublicMilestoneContent200Response**](Destiny2_GetPublicMilestoneContent_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## destiny2_period_get_public_milestones

> crate::models::Destiny2GetPublicMilestones200Response destiny2_period_get_public_milestones()


Gets public information about currently available Milestones.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::Destiny2GetPublicMilestones200Response**](Destiny2_GetPublicMilestones_200_response.md)

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


## destiny2_period_get_unique_weapon_history

> crate::models::Destiny2GetUniqueWeaponHistory200Response destiny2_period_get_unique_weapon_history(character_id, destiny_membership_id, membership_type)


Gets details about unique weapon usage, including all exotic weapons.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**character_id** | **i64** | The id of the character to retrieve. | [required] |
**destiny_membership_id** | **i64** | The Destiny membershipId of the user to retrieve. | [required] |
**membership_type** | **i32** | A valid non-BungieNet membership type. | [required] |

### Return type

[**crate::models::Destiny2GetUniqueWeaponHistory200Response**](Destiny2_GetUniqueWeaponHistory_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## destiny2_period_get_vendor

> crate::models::Destiny2GetVendor200Response destiny2_period_get_vendor(character_id, destiny_membership_id, membership_type, vendor_hash, components)


Get the details of a specific Vendor.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**character_id** | **i64** | The Destiny Character ID of the character for whom we're getting vendor info. | [required] |
**destiny_membership_id** | **i64** | Destiny membership ID of another user. You may be denied. | [required] |
**membership_type** | **i32** | A valid non-BungieNet membership type. | [required] |
**vendor_hash** | **i32** | The Hash identifier of the Vendor to be returned. | [required] |
**components** | Option<[**Vec<i32>**](i32.md)> | A comma separated list of components to return (as strings or numeric values). See the DestinyComponentType enum for valid components to request. You must request at least one component to receive results. |  |

### Return type

[**crate::models::Destiny2GetVendor200Response**](Destiny2_GetVendor_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## destiny2_period_get_vendors

> crate::models::Destiny2GetVendors200Response destiny2_period_get_vendors(character_id, destiny_membership_id, membership_type, components, filter)


Get currently available vendors from the list of vendors that can possibly have rotating inventory. Note that this does not include things like preview vendors and vendors-as-kiosks, neither of whom have rotating/dynamic inventories. Use their definitions as-is for those.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**character_id** | **i64** | The Destiny Character ID of the character for whom we're getting vendor info. | [required] |
**destiny_membership_id** | **i64** | Destiny membership ID of another user. You may be denied. | [required] |
**membership_type** | **i32** | A valid non-BungieNet membership type. | [required] |
**components** | Option<[**Vec<i32>**](i32.md)> | A comma separated list of components to return (as strings or numeric values). See the DestinyComponentType enum for valid components to request. You must request at least one component to receive results. |  |
**filter** | Option<**i32**> | The filter of what vendors and items to return, if any. |  |

### Return type

[**crate::models::Destiny2GetVendors200Response**](Destiny2_GetVendors_200_response.md)

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


## destiny2_period_pull_from_postmaster

> crate::models::GroupV2EditGroup200Response destiny2_period_pull_from_postmaster()


Extract an item from the Postmaster, with whatever implications that may entail. You must have a valid Destiny account. You must also pass BOTH a reference AND an instance ID if it's an instanced item.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GroupV2EditGroup200Response**](GroupV2_EditGroup_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## destiny2_period_report_offensive_post_game_carnage_report_player

> crate::models::GroupV2EditGroup200Response destiny2_period_report_offensive_post_game_carnage_report_player(activity_id)


Report a player that you met in an activity that was engaging in ToS-violating activities. Both you and the offending player must have played in the activityId passed in. Please use this judiciously and only when you have strong suspicions of violation, pretty please.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**activity_id** | **i64** | The ID of the activity where you ran into the brigand that you're reporting. | [required] |

### Return type

[**crate::models::GroupV2EditGroup200Response**](GroupV2_EditGroup_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## destiny2_period_search_destiny_entities

> crate::models::Destiny2SearchDestinyEntities200Response destiny2_period_search_destiny_entities(search_term, r#type, page)


Gets a page list of Destiny items.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search_term** | **String** | The string to use when searching for Destiny entities. | [required] |
**r#type** | **String** | The type of entity for whom you would like results. These correspond to the entity's definition contract name. For instance, if you are looking for items, this property should be 'DestinyInventoryItemDefinition'. | [required] |
**page** | Option<**i32**> | Page number to return, starting with 0. |  |

### Return type

[**crate::models::Destiny2SearchDestinyEntities200Response**](Destiny2_SearchDestinyEntities_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## destiny2_period_search_destiny_player_by_bungie_name

> crate::models::Destiny2SearchDestinyPlayerByBungieName200Response destiny2_period_search_destiny_player_by_bungie_name(membership_type)


Returns a list of Destiny memberships given a global Bungie Display Name. This method will hide overridden memberships due to cross save.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**membership_type** | **i32** | A valid non-BungieNet membership type, or All. Indicates which memberships to return. You probably want this set to All. | [required] |

### Return type

[**crate::models::Destiny2SearchDestinyPlayerByBungieName200Response**](Destiny2_SearchDestinyPlayerByBungieName_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## destiny2_period_set_item_lock_state

> crate::models::GroupV2EditGroup200Response destiny2_period_set_item_lock_state()


Set the Lock State for an instanced item. You must have a valid Destiny Account.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GroupV2EditGroup200Response**](GroupV2_EditGroup_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## destiny2_period_set_quest_tracked_state

> crate::models::GroupV2EditGroup200Response destiny2_period_set_quest_tracked_state()


Set the Tracking State for an instanced item, if that item is a Quest or Bounty. You must have a valid Destiny Account. Yeah, it's an item.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GroupV2EditGroup200Response**](GroupV2_EditGroup_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## destiny2_period_transfer_item

> crate::models::GroupV2EditGroup200Response destiny2_period_transfer_item()


Transfer an item to/from your vault. You must have a valid Destiny account. You must also pass BOTH a reference AND an instance ID if it's an instanced item. itshappening.gif

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GroupV2EditGroup200Response**](GroupV2_EditGroup_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

