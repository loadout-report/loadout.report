# DestinyPeriodDefinitionsPeriodDestinyEntitySearchResultItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**hash** | Option<**i32**> | The hash identifier of the entity. You will use this to look up the DestinyDefinition relevant for the entity found. | [optional]
**entity_type** | Option<**String**> | The type of entity, returned as a string matching the DestinyDefinition's contract class name. You'll have to have your own mapping from class names to actually looking up those definitions in the manifest databases. | [optional]
**display_properties** | Option<[**crate::models::DestinyDefinitionsDestinyEntitySearchResultItemDisplayProperties**](Destiny_Definitions_DestinyEntitySearchResultItem_displayProperties.md)> |  | [optional]
**weight** | Option<**f64**> | The ranking value for sorting that we calculated using our relevance formula. This will hopefully get better with time and iteration. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


