# DestinyPeriodMilestonesPeriodDestinyMilestoneActivityVariant

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**activity_hash** | Option<**i32**> | The hash for the specific variant of the activity related to this milestone. You can pull more detailed static info from the DestinyActivityDefinition, such as difficulty level. | [optional]
**completion_status** | Option<[**crate::models::DestinyMilestonesDestinyMilestoneActivityVariantCompletionStatus**](Destiny_Milestones_DestinyMilestoneActivityVariant_completionStatus.md)> |  | [optional]
**activity_mode_hash** | Option<**i32**> | The hash identifier of the most specific Activity Mode under which this activity is played. This is useful for situations where the activity in question is - for instance - a PVP map, but it's not clear what mode the PVP map is being played under. If it's a playlist, this will be less specific: but hopefully useful in some way. | [optional]
**activity_mode_type** | Option<**i32**> | The enumeration equivalent of the most specific Activity Mode under which this activity is played. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


