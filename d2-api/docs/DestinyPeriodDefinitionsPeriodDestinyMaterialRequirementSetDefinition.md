# DestinyPeriodDefinitionsPeriodDestinyMaterialRequirementSetDefinition

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**materials** | Option<[**Vec<crate::models::DestinyPeriodDefinitionsPeriodDestinyMaterialRequirement>**](Destiny.Definitions.DestinyMaterialRequirement.md)> | The list of all materials that are required. | [optional]
**hash** | Option<**i32**> | The unique identifier for this entity. Guaranteed to be unique for the type of entity, but not globally.  When entities refer to each other in Destiny content, it is this hash that they are referring to. | [optional]
**index** | Option<**i32**> | The index of the entity as it was found in the investment tables. | [optional]
**redacted** | Option<**bool**> | If this is true, then there is an entity with this identifier/type combination, but BNet is not yet allowed to show it. Sorry! | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


