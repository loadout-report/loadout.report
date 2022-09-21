# DestinyPeriodDefinitionsPeriodDestinyDamageTypeDefinition

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**display_properties** | Option<[**crate::models::DestinyDefinitionsDestinyDamageTypeDefinitionDisplayProperties**](Destiny_Definitions_DestinyDamageTypeDefinition_displayProperties.md)> |  | [optional]
**transparent_icon_path** | Option<**String**> | A variant of the icon that is transparent and colorless. | [optional]
**show_icon** | Option<**bool**> | If TRUE, the game shows this damage type's icon. Otherwise, it doesn't. Whether you show it or not is up to you. | [optional]
**enum_value** | Option<**i32**> | We have an enumeration for damage types for quick reference. This is the current definition's damage type enum value. | [optional]
**hash** | Option<**i32**> | The unique identifier for this entity. Guaranteed to be unique for the type of entity, but not globally.  When entities refer to each other in Destiny content, it is this hash that they are referring to. | [optional]
**index** | Option<**i32**> | The index of the entity as it was found in the investment tables. | [optional]
**redacted** | Option<**bool**> | If this is true, then there is an entity with this identifier/type combination, but BNet is not yet allowed to show it. Sorry! | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


