# DestinyPeriodDefinitionsPeriodMilestonesPeriodDestinyMilestoneChallengeActivityDefinition

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**activity_hash** | Option<**i32**> | The activity for which this challenge is active. | [optional]
**challenges** | Option<[**Vec<crate::models::DestinyPeriodDefinitionsPeriodMilestonesPeriodDestinyMilestoneChallengeDefinition>**](Destiny.Definitions.Milestones.DestinyMilestoneChallengeDefinition.md)> |  | [optional]
**activity_graph_nodes** | Option<[**Vec<crate::models::DestinyPeriodDefinitionsPeriodMilestonesPeriodDestinyMilestoneChallengeActivityGraphNodeEntry>**](Destiny.Definitions.Milestones.DestinyMilestoneChallengeActivityGraphNodeEntry.md)> | If the activity and its challenge is visible on any of these nodes, it will be returned. | [optional]
**phases** | Option<[**Vec<crate::models::DestinyPeriodDefinitionsPeriodMilestonesPeriodDestinyMilestoneChallengeActivityPhase>**](Destiny.Definitions.Milestones.DestinyMilestoneChallengeActivityPhase.md)> | Phases related to this activity, if there are any.  These will be listed in the order in which they will appear in the actual activity. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


