# DestinyPeriodDefinitionsPeriodArtifactsPeriodDestinyArtifactTierDefinition

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**tier_hash** | Option<**i32**> | An identifier, unique within the Artifact, for this specific tier. | [optional]
**display_title** | Option<**String**> | The human readable title of this tier, if any. | [optional]
**progress_requirement_message** | Option<**String**> | A string representing the localized minimum requirement text for this Tier, if any. | [optional]
**items** | Option<[**Vec<crate::models::DestinyPeriodDefinitionsPeriodArtifactsPeriodDestinyArtifactTierItemDefinition>**](Destiny.Definitions.Artifacts.DestinyArtifactTierItemDefinition.md)> | The items that can be earned within this tier. | [optional]
**minimum_unlock_points_used_requirement** | Option<**i32**> | The minimum number of \"unlock points\" that you must have used before you can unlock items from this tier. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


