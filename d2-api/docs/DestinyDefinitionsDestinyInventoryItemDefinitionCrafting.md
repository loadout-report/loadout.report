# DestinyDefinitionsDestinyInventoryItemDefinitionCrafting

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**output_item_hash** | Option<**i32**> | A reference to the item definition that is created when crafting with this 'recipe' item. | [optional]
**required_socket_type_hashes** | Option<**Vec<i32>**> | A list of socket type hashes that describes which sockets are required for crafting with this recipe. | [optional]
**failed_requirement_strings** | Option<**Vec<String>**> |  | [optional]
**base_material_requirements** | Option<**i32**> | A reference to the base material requirements for crafting with this recipe. | [optional]
**bonus_plugs** | Option<[**Vec<crate::models::DestinyPeriodDefinitionsPeriodDestinyItemCraftingBlockBonusPlugDefinition>**](Destiny.Definitions.DestinyItemCraftingBlockBonusPlugDefinition.md)> | A list of 'bonus' socket plugs that may be available if certain requirements are met. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


