# DestinyPeriodDefinitionsPeriodSeasonsPeriodDestinySeasonDefinition

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**display_properties** | Option<[**crate::models::DestinyPeriodDefinitionsPeriodCommonPeriodDestinyDisplayPropertiesDefinition**](Destiny.Definitions.Common.DestinyDisplayPropertiesDefinition.md)> |  | [optional]
**background_image_path** | Option<**String**> |  | [optional]
**season_number** | Option<**i32**> |  | [optional]
**start_date** | Option<**String**> |  | [optional]
**end_date** | Option<**String**> |  | [optional]
**season_pass_hash** | Option<**i32**> |  | [optional]
**season_pass_progression_hash** | Option<**i32**> |  | [optional]
**artifact_item_hash** | Option<**i32**> |  | [optional]
**seal_presentation_node_hash** | Option<**i32**> |  | [optional]
**seasonal_challenges_presentation_node_hash** | Option<**i32**> |  | [optional]
**preview** | Option<[**crate::models::DestinyDefinitionsSeasonsDestinySeasonDefinitionPreview**](Destiny_Definitions_Seasons_DestinySeasonDefinition_preview.md)> |  | [optional]
**hash** | Option<**i32**> | The unique identifier for this entity. Guaranteed to be unique for the type of entity, but not globally.  When entities refer to each other in Destiny content, it is this hash that they are referring to. | [optional]
**index** | Option<**i32**> | The index of the entity as it was found in the investment tables. | [optional]
**redacted** | Option<**bool**> | If this is true, then there is an entity with this identifier/type combination, but BNet is not yet allowed to show it. Sorry! | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


