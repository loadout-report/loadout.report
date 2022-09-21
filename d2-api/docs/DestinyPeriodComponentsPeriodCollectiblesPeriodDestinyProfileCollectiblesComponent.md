# DestinyPeriodComponentsPeriodCollectiblesPeriodDestinyProfileCollectiblesComponent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**recent_collectible_hashes** | Option<**Vec<i32>**> | The list of collectibles determined by the game as having been \"recently\" acquired. | [optional]
**newness_flagged_collectible_hashes** | Option<**Vec<i32>**> | The list of collectibles determined by the game as having been \"recently\" acquired.  The game client itself actually controls this data, so I personally question whether anyone will get much use out of this: because we can't edit this value through the API. But in case anyone finds it useful, here it is. | [optional]
**collectibles** | Option<[**::std::collections::HashMap<String, crate::models::DestinyPeriodComponentsPeriodCollectiblesPeriodDestinyCollectibleComponent>**](Destiny.Components.Collectibles.DestinyCollectibleComponent.md)> |  | [optional]
**collection_categories_root_node_hash** | Option<**i32**> | The hash for the root presentation node definition of Collection categories. | [optional]
**collection_badges_root_node_hash** | Option<**i32**> | The hash for the root presentation node definition of Collection Badges. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


