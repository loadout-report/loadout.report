# DestinyPeriodDefinitionsPeriodDestinyItemSetBlockDefinition

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**item_list** | Option<[**Vec<crate::models::DestinyPeriodDefinitionsPeriodDestinyItemSetBlockEntryDefinition>**](Destiny.Definitions.DestinyItemSetBlockEntryDefinition.md)> | A collection of hashes of set items, for items such as Quest Metadata items that possess this data. | [optional]
**require_ordered_set_item_add** | Option<**bool**> | If true, items in the set can only be added in increasing order, and adding an item will remove any previous item. For Quests, this is by necessity true. Only one quest step is present at a time, and previous steps are removed as you advance in the quest. | [optional]
**set_is_featured** | Option<**bool**> | If true, the UI should treat this quest as \"featured\" | [optional]
**set_type** | Option<**String**> | A string identifier we can use to attempt to identify the category of the Quest. | [optional]
**quest_line_name** | Option<**String**> | The name of the quest line that this quest step is a part of. | [optional]
**quest_line_description** | Option<**String**> | The description of the quest line that this quest step is a part of. | [optional]
**quest_step_summary** | Option<**String**> | An additional summary of this step in the quest line. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


