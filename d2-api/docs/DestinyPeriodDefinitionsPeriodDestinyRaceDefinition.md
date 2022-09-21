# DestinyPeriodDefinitionsPeriodDestinyRaceDefinition

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**display_properties** | Option<[**crate::models::DestinyPeriodDefinitionsPeriodCommonPeriodDestinyDisplayPropertiesDefinition**](Destiny.Definitions.Common.DestinyDisplayPropertiesDefinition.md)> |  | [optional]
**race_type** | Option<**i32**> | An enumeration defining the existing, known Races/Species for player characters. This value will be the enum value matching this definition. | [optional]
**gendered_race_names** | Option<**::std::collections::HashMap<String, String>**> | A localized string referring to the singular form of the Race's name when referred to in gendered form. Keyed by the DestinyGender. | [optional]
**gendered_race_names_by_gender_hash** | Option<**::std::collections::HashMap<String, String>**> |  | [optional]
**hash** | Option<**i32**> | The unique identifier for this entity. Guaranteed to be unique for the type of entity, but not globally.  When entities refer to each other in Destiny content, it is this hash that they are referring to. | [optional]
**index** | Option<**i32**> | The index of the entity as it was found in the investment tables. | [optional]
**redacted** | Option<**bool**> | If this is true, then there is an entity with this identifier/type combination, but BNet is not yet allowed to show it. Sorry! | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


