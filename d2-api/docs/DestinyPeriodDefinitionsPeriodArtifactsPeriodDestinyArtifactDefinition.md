# DestinyPeriodDefinitionsPeriodArtifactsPeriodDestinyArtifactDefinition

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**display_properties** | Option<[**crate::models::DestinyDefinitionsArtifactsDestinyArtifactDefinitionDisplayProperties**](Destiny_Definitions_Artifacts_DestinyArtifactDefinition_displayProperties.md)> |  | [optional]
**translation_block** | Option<[**crate::models::DestinyDefinitionsArtifactsDestinyArtifactDefinitionTranslationBlock**](Destiny_Definitions_Artifacts_DestinyArtifactDefinition_translationBlock.md)> |  | [optional]
**tiers** | Option<[**Vec<crate::models::DestinyPeriodDefinitionsPeriodArtifactsPeriodDestinyArtifactTierDefinition>**](Destiny.Definitions.Artifacts.DestinyArtifactTierDefinition.md)> | Any Tier/Rank data related to this artifact, listed in display order.  Currently sourced from a Vendor, but this source is subject to change. | [optional]
**hash** | Option<**i32**> | The unique identifier for this entity. Guaranteed to be unique for the type of entity, but not globally.  When entities refer to each other in Destiny content, it is this hash that they are referring to. | [optional]
**index** | Option<**i32**> | The index of the entity as it was found in the investment tables. | [optional]
**redacted** | Option<**bool**> | If this is true, then there is an entity with this identifier/type combination, but BNet is not yet allowed to show it. Sorry! | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


