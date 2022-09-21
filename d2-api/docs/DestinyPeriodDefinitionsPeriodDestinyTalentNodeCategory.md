# DestinyPeriodDefinitionsPeriodDestinyTalentNodeCategory

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**identifier** | Option<**String**> | Mostly just for debug purposes, but if you find it useful you can have it. This is BNet's manually created identifier for this category. | [optional]
**is_lore_driven** | Option<**bool**> | If true, we found the localized content in a related DestinyLoreDefinition instead of local BNet localization files. This is mostly for ease of my own future investigations. | [optional]
**display_properties** | Option<[**crate::models::DestinyDefinitionsDestinyTalentNodeCategoryDisplayProperties**](Destiny_Definitions_DestinyTalentNodeCategory_displayProperties.md)> |  | [optional]
**node_hashes** | Option<**Vec<i32>**> | The set of all hash identifiers for Talent Nodes (DestinyTalentNodeDefinition) in this Talent Grid that are part of this Category. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


