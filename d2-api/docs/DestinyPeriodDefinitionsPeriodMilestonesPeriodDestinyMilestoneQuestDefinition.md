# DestinyPeriodDefinitionsPeriodMilestonesPeriodDestinyMilestoneQuestDefinition

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**quest_item_hash** | Option<**i32**> | The item representing this Milestone quest. Use this hash to look up the DestinyInventoryItemDefinition for the quest to find its steps and human readable data. | [optional]
**display_properties** | Option<[**crate::models::DestinyDefinitionsMilestonesDestinyMilestoneQuestDefinitionDisplayProperties**](Destiny_Definitions_Milestones_DestinyMilestoneQuestDefinition_displayProperties.md)> |  | [optional]
**override_image** | Option<**String**> | If populated, this image can be shown instead of the generic milestone's image when this quest is live, or it can be used to show a background image for the quest itself that differs from that of the Activity or the Milestone. | [optional]
**quest_rewards** | Option<[**crate::models::DestinyDefinitionsMilestonesDestinyMilestoneQuestDefinitionQuestRewards**](Destiny_Definitions_Milestones_DestinyMilestoneQuestDefinition_questRewards.md)> |  | [optional]
**activities** | Option<[**::std::collections::HashMap<String, crate::models::DestinyPeriodDefinitionsPeriodMilestonesPeriodDestinyMilestoneActivityDefinition>**](Destiny.Definitions.Milestones.DestinyMilestoneActivityDefinition.md)> | The full set of all possible \"conceptual activities\" that are related to this Milestone. Tiers or alternative modes of play within these conceptual activities will be defined as sub-entities. Keyed by the Conceptual Activity Hash. Use the key to look up DestinyActivityDefinition. | [optional]
**destination_hash** | Option<**i32**> | Sometimes, a Milestone's quest is related to an entire Destination rather than a specific activity. In that situation, this will be the hash of that Destination. Hotspots are currently the only Milestones that expose this data, but that does not preclude this data from being returned for other Milestones in the future. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


