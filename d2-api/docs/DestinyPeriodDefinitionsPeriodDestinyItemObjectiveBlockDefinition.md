# DestinyPeriodDefinitionsPeriodDestinyItemObjectiveBlockDefinition

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**objective_hashes** | Option<**Vec<i32>**> | The hashes to Objectives (DestinyObjectiveDefinition) that are part of this Quest Step, in the order that they should be rendered. | [optional]
**display_activity_hashes** | Option<**Vec<i32>**> | For every entry in objectiveHashes, there is a corresponding entry in this array at the same index. If the objective is meant to be associated with a specific DestinyActivityDefinition, there will be a valid hash at that index. Otherwise, it will be invalid (0).  Rendered somewhat obsolete by perObjectiveDisplayProperties, which currently has much the same information but may end up with more info in the future. | [optional]
**require_full_objective_completion** | Option<**bool**> | If True, all objectives must be completed for the step to be completed. If False, any one objective can be completed for the step to be completed. | [optional]
**questline_item_hash** | Option<**i32**> | The hash for the DestinyInventoryItemDefinition representing the Quest to which this Quest Step belongs. | [optional]
**narrative** | Option<**String**> | The localized string for narrative text related to this quest step, if any. | [optional]
**objective_verb_name** | Option<**String**> | The localized string describing an action to be performed associated with the objectives, if any. | [optional]
**quest_type_identifier** | Option<**String**> | The identifier for the type of quest being performed, if any. Not associated with any fixed definition, yet. | [optional]
**quest_type_hash** | Option<**i32**> | A hashed value for the questTypeIdentifier, because apparently I like to be redundant. | [optional]
**per_objective_display_properties** | Option<[**Vec<crate::models::DestinyPeriodDefinitionsPeriodDestinyObjectiveDisplayProperties>**](Destiny.Definitions.DestinyObjectiveDisplayProperties.md)> | One entry per Objective on the item, it will have related display information. | [optional]
**display_as_stat_tracker** | Option<**bool**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


