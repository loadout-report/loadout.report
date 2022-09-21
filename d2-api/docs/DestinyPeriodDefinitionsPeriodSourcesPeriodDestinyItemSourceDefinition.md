# DestinyPeriodDefinitionsPeriodSourcesPeriodDestinyItemSourceDefinition

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**level** | Option<**i32**> | The level at which the item spawns. Essentially the Primary Key for this source data: there will be multiple of these source entries per item that has source data, grouped by the level at which the item spawns. | [optional]
**min_quality** | Option<**i32**> | The minimum Quality at which the item spawns for this level. Examine DestinyInventoryItemDefinition for more information about what Quality means. Just don't ask Phaedrus about it, he'll never stop talking and you'll have to write a book about it. | [optional]
**max_quality** | Option<**i32**> | The maximum quality at which the item spawns for this level. | [optional]
**min_level_required** | Option<**i32**> | The minimum Character Level required for equipping the item when the item spawns at the item level defined on this DestinyItemSourceDefinition, as far as we saw in our processing. | [optional]
**max_level_required** | Option<**i32**> | The maximum Character Level required for equipping the item when the item spawns at the item level defined on this DestinyItemSourceDefinition, as far as we saw in our processing. | [optional]
**computed_stats** | Option<[**::std::collections::HashMap<String, crate::models::DestinyPeriodDefinitionsPeriodDestinyInventoryItemStatDefinition>**](Destiny.Definitions.DestinyInventoryItemStatDefinition.md)> | The stats computed for this level/quality range. | [optional]
**source_hashes** | Option<**Vec<i32>**> | The DestinyRewardSourceDefinitions found that can spawn the item at this level. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


