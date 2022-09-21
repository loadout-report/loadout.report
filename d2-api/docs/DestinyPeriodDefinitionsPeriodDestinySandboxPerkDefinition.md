# DestinyPeriodDefinitionsPeriodDestinySandboxPerkDefinition

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**display_properties** | Option<[**crate::models::DestinyDefinitionsDestinySandboxPerkDefinitionDisplayProperties**](Destiny_Definitions_DestinySandboxPerkDefinition_displayProperties.md)> |  | [optional]
**perk_identifier** | Option<**String**> | The string identifier for the perk. | [optional]
**is_displayable** | Option<**bool**> | If true, you can actually show the perk in the UI. Otherwise, it doesn't have useful player-facing information. | [optional]
**damage_type** | Option<**i32**> | If this perk grants a damage type to a weapon, the damage type will be defined here.  Unless you have a compelling reason to use this enum value, use the damageTypeHash instead to look up the actual DestinyDamageTypeDefinition. | [optional]
**damage_type_hash** | Option<**i32**> | The hash identifier for looking up the DestinyDamageTypeDefinition, if this perk has a damage type.  This is preferred over using the damageType enumeration value, which has been left purely because it is occasionally convenient. | [optional]
**perk_groups** | Option<[**crate::models::DestinyDefinitionsDestinySandboxPerkDefinitionPerkGroups**](Destiny_Definitions_DestinySandboxPerkDefinition_perkGroups.md)> |  | [optional]
**hash** | Option<**i32**> | The unique identifier for this entity. Guaranteed to be unique for the type of entity, but not globally.  When entities refer to each other in Destiny content, it is this hash that they are referring to. | [optional]
**index** | Option<**i32**> | The index of the entity as it was found in the investment tables. | [optional]
**redacted** | Option<**bool**> | If this is true, then there is an entity with this identifier/type combination, but BNet is not yet allowed to show it. Sorry! | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


