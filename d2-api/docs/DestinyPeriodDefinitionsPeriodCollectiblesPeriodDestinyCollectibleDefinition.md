# DestinyPeriodDefinitionsPeriodCollectiblesPeriodDestinyCollectibleDefinition

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**display_properties** | Option<[**crate::models::DestinyPeriodDefinitionsPeriodCommonPeriodDestinyDisplayPropertiesDefinition**](Destiny.Definitions.Common.DestinyDisplayPropertiesDefinition.md)> |  | [optional]
**scope** | Option<**i32**> | Indicates whether the state of this Collectible is determined on a per-character or on an account-wide basis. | [optional]
**source_string** | Option<**String**> | A human readable string for a hint about how to acquire the item. | [optional]
**source_hash** | Option<**i32**> | This is a hash identifier we are building on the BNet side in an attempt to let people group collectibles by similar sources.  I can't promise that it's going to be 100% accurate, but if the designers were consistent in assigning the same source strings to items with the same sources, it *ought to* be. No promises though.  This hash also doesn't relate to an actual definition, just to note: we've got nothing useful other than the source string for this data. | [optional]
**item_hash** | Option<**i32**> |  | [optional]
**acquisition_info** | Option<[**crate::models::DestinyPeriodDefinitionsPeriodCollectiblesPeriodDestinyCollectibleAcquisitionBlock**](Destiny.Definitions.Collectibles.DestinyCollectibleAcquisitionBlock.md)> |  | [optional]
**state_info** | Option<[**crate::models::DestinyPeriodDefinitionsPeriodCollectiblesPeriodDestinyCollectibleStateBlock**](Destiny.Definitions.Collectibles.DestinyCollectibleStateBlock.md)> |  | [optional]
**presentation_info** | Option<[**crate::models::DestinyPeriodDefinitionsPeriodPresentationPeriodDestinyPresentationChildBlock**](Destiny.Definitions.Presentation.DestinyPresentationChildBlock.md)> |  | [optional]
**presentation_node_type** | Option<**i32**> |  | [optional]
**trait_ids** | Option<**Vec<String>**> |  | [optional]
**trait_hashes** | Option<**Vec<i32>**> |  | [optional]
**parent_node_hashes** | Option<**Vec<i32>**> | A quick reference to presentation nodes that have this node as a child. Presentation nodes can be parented under multiple parents. | [optional]
**hash** | Option<**i32**> | The unique identifier for this entity. Guaranteed to be unique for the type of entity, but not globally.  When entities refer to each other in Destiny content, it is this hash that they are referring to. | [optional]
**index** | Option<**i32**> | The index of the entity as it was found in the investment tables. | [optional]
**redacted** | Option<**bool**> | If this is true, then there is an entity with this identifier/type combination, but BNet is not yet allowed to show it. Sorry! | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


