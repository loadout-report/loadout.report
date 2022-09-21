# DestinyPeriodDefinitionsPeriodChecklistsPeriodDestinyChecklistDefinition

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**display_properties** | Option<[**crate::models::DestinyPeriodDefinitionsPeriodCommonPeriodDestinyDisplayPropertiesDefinition**](Destiny.Definitions.Common.DestinyDisplayPropertiesDefinition.md)> |  | [optional]
**view_action_string** | Option<**String**> | A localized string prompting you to view the checklist. | [optional]
**scope** | Option<**i32**> | Indicates whether you will find this checklist on the Profile or Character components. | [optional]
**entries** | Option<[**Vec<crate::models::DestinyPeriodDefinitionsPeriodChecklistsPeriodDestinyChecklistEntryDefinition>**](Destiny.Definitions.Checklists.DestinyChecklistEntryDefinition.md)> | The individual checklist items. Gotta catch 'em all. | [optional]
**hash** | Option<**i32**> | The unique identifier for this entity. Guaranteed to be unique for the type of entity, but not globally.  When entities refer to each other in Destiny content, it is this hash that they are referring to. | [optional]
**index** | Option<**i32**> | The index of the entity as it was found in the investment tables. | [optional]
**redacted** | Option<**bool**> | If this is true, then there is an entity with this identifier/type combination, but BNet is not yet allowed to show it. Sorry! | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


