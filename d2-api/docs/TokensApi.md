# \TokensApi

All URIs are relative to *https://www.bungie.net/Platform*

Method | HTTP request | Description
------------- | ------------- | -------------
[**tokens_period_apply_missing_partner_offers_without_claim**](TokensApi.md#tokens_period_apply_missing_partner_offers_without_claim) | **POST** /Tokens/Partner/ApplyMissingOffers/{partnerApplicationId}/{targetBnetMembershipId}/ | 
[**tokens_period_claim_partner_offer**](TokensApi.md#tokens_period_claim_partner_offer) | **POST** /Tokens/Partner/ClaimOffer/ | 
[**tokens_period_force_drops_repair**](TokensApi.md#tokens_period_force_drops_repair) | **POST** /Tokens/Partner/ForceDropsRepair/ | 
[**tokens_period_get_bungie_rewards_for_platform_user**](TokensApi.md#tokens_period_get_bungie_rewards_for_platform_user) | **GET** /Tokens/Rewards/GetRewardsForPlatformUser/{membershipId}/{membershipType}/ | 
[**tokens_period_get_bungie_rewards_for_user**](TokensApi.md#tokens_period_get_bungie_rewards_for_user) | **GET** /Tokens/Rewards/GetRewardsForUser/{membershipId}/ | 
[**tokens_period_get_bungie_rewards_list**](TokensApi.md#tokens_period_get_bungie_rewards_list) | **GET** /Tokens/Rewards/BungieRewards/ | 
[**tokens_period_get_partner_offer_sku_history**](TokensApi.md#tokens_period_get_partner_offer_sku_history) | **GET** /Tokens/Partner/History/{partnerApplicationId}/{targetBnetMembershipId}/ | 
[**tokens_period_get_partner_reward_history**](TokensApi.md#tokens_period_get_partner_reward_history) | **GET** /Tokens/Partner/History/{targetBnetMembershipId}/Application/{partnerApplicationId}/ | 



## tokens_period_apply_missing_partner_offers_without_claim

> crate::models::GroupV2GetUserClanInviteSetting200Response tokens_period_apply_missing_partner_offers_without_claim(partner_application_id, target_bnet_membership_id)


Apply a partner offer to the targeted user. This endpoint does not claim a new offer, but any already claimed offers will be applied to the game if not already.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**partner_application_id** | **i32** | The partner application identifier. | [required] |
**target_bnet_membership_id** | **i64** | The bungie.net user to apply missing offers to. If not self, elevated permissions are required. | [required] |

### Return type

[**crate::models::GroupV2GetUserClanInviteSetting200Response**](GroupV2_GetUserClanInviteSetting_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tokens_period_claim_partner_offer

> crate::models::GroupV2GetUserClanInviteSetting200Response tokens_period_claim_partner_offer()


Claim a partner offer as the authenticated user.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GroupV2GetUserClanInviteSetting200Response**](GroupV2_GetUserClanInviteSetting_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tokens_period_force_drops_repair

> crate::models::GroupV2GetUserClanInviteSetting200Response tokens_period_force_drops_repair()


Twitch Drops self-repair function - scans twitch for drops not marked as fulfilled and resyncs them.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GroupV2GetUserClanInviteSetting200Response**](GroupV2_GetUserClanInviteSetting_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tokens_period_get_bungie_rewards_for_platform_user

> crate::models::TokensGetBungieRewardsForUser200Response tokens_period_get_bungie_rewards_for_platform_user(membership_id, membership_type)


Returns the bungie rewards for the targeted user when a platform membership Id and Type are used.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**membership_id** | **i64** | users platform membershipId for requested user rewards. If not self, elevated permissions are required. | [required] |
**membership_type** | **i32** | The target Destiny 2 membership type. | [required] |

### Return type

[**crate::models::TokensGetBungieRewardsForUser200Response**](Tokens_GetBungieRewardsForUser_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tokens_period_get_bungie_rewards_for_user

> crate::models::TokensGetBungieRewardsForUser200Response tokens_period_get_bungie_rewards_for_user(membership_id)


Returns the bungie rewards for the targeted user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**membership_id** | **i64** | bungie.net user membershipId for requested user rewards. If not self, elevated permissions are required. | [required] |

### Return type

[**crate::models::TokensGetBungieRewardsForUser200Response**](Tokens_GetBungieRewardsForUser_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tokens_period_get_bungie_rewards_list

> crate::models::TokensGetBungieRewardsForUser200Response tokens_period_get_bungie_rewards_list()


Returns a list of the current bungie rewards

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::TokensGetBungieRewardsForUser200Response**](Tokens_GetBungieRewardsForUser_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tokens_period_get_partner_offer_sku_history

> crate::models::TokensGetPartnerOfferSkuHistory200Response tokens_period_get_partner_offer_sku_history(partner_application_id, target_bnet_membership_id)


Returns the partner sku and offer history of the targeted user. Elevated permissions are required to see users that are not yourself.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**partner_application_id** | **i32** | The partner application identifier. | [required] |
**target_bnet_membership_id** | **i64** | The bungie.net user to apply missing offers to. If not self, elevated permissions are required. | [required] |

### Return type

[**crate::models::TokensGetPartnerOfferSkuHistory200Response**](Tokens_GetPartnerOfferSkuHistory_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tokens_period_get_partner_reward_history

> crate::models::TokensGetPartnerRewardHistory200Response tokens_period_get_partner_reward_history(partner_application_id, target_bnet_membership_id)


Returns the partner rewards history of the targeted user, both partner offers and Twitch drops.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**partner_application_id** | **i32** | The partner application identifier. | [required] |
**target_bnet_membership_id** | **i64** | The bungie.net user to return reward history for. | [required] |

### Return type

[**crate::models::TokensGetPartnerRewardHistory200Response**](Tokens_GetPartnerRewardHistory_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

