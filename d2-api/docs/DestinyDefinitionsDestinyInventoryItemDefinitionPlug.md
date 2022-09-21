# DestinyDefinitionsDestinyInventoryItemDefinitionPlug

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**insertion_rules** | Option<[**Vec<crate::models::DestinyPeriodDefinitionsPeriodItemsPeriodDestinyPlugRuleDefinition>**](Destiny.Definitions.Items.DestinyPlugRuleDefinition.md)> | The rules around when this plug can be inserted into a socket, aside from the socket's individual restrictions.  The live data DestinyItemPlugComponent.insertFailIndexes will be an index into this array, so you can pull out the failure strings appropriate for the user. | [optional]
**plug_category_identifier** | Option<**String**> | The string identifier for the plug's category. Use the socket's DestinySocketTypeDefinition.plugWhitelist to determine whether this plug can be inserted into the socket. | [optional]
**plug_category_hash** | Option<**i32**> | The hash for the plugCategoryIdentifier. You can use this instead if you wish: I put both in the definition for debugging purposes. | [optional]
**on_action_recreate_self** | Option<**bool**> | If you successfully socket the item, this will determine whether or not you get \"refunded\" on the plug. | [optional]
**insertion_material_requirement_hash** | Option<**i32**> | If inserting this plug requires materials, this is the hash identifier for looking up the DestinyMaterialRequirementSetDefinition for those requirements. | [optional]
**preview_item_override_hash** | Option<**i32**> | In the game, if you're inspecting a plug item directly, this will be the item shown with the plug attached. Look up the DestinyInventoryItemDefinition for this hash for the item. | [optional]
**enabled_material_requirement_hash** | Option<**i32**> | It's not enough for the plug to be inserted. It has to be enabled as well. For it to be enabled, it may require materials. This is the hash identifier for the DestinyMaterialRequirementSetDefinition for those requirements, if there is one. | [optional]
**enabled_rules** | Option<[**Vec<crate::models::DestinyPeriodDefinitionsPeriodItemsPeriodDestinyPlugRuleDefinition>**](Destiny.Definitions.Items.DestinyPlugRuleDefinition.md)> | The rules around whether the plug, once inserted, is enabled and providing its benefits.  The live data DestinyItemPlugComponent.enableFailIndexes will be an index into this array, so you can pull out the failure strings appropriate for the user. | [optional]
**ui_plug_label** | Option<**String**> | Plugs can have arbitrary, UI-defined identifiers that the UI designers use to determine the style applied to plugs. Unfortunately, we have neither a definitive list of these labels nor advance warning of when new labels might be applied or how that relates to how they get rendered. If you want to, you can refer to known labels to change your own styles: but know that new ones can be created arbitrarily, and we have no way of associating the labels with any specific UI style guidance... you'll have to piece that together on your end. Or do what we do, and just show plugs more generically, without specialized styles. | [optional]
**plug_style** | Option<**i32**> |  | [optional]
**plug_availability** | Option<**i32**> | Indicates the rules about when this plug can be used. See the PlugAvailabilityMode enumeration for more information! | [optional]
**alternate_ui_plug_label** | Option<**String**> | If the plug meets certain state requirements, it may have an alternative label applied to it. This is the alternative label that will be applied in such a situation. | [optional]
**alternate_plug_style** | Option<**i32**> | The alternate plug of the plug: only applies when the item is in states that only the server can know about and control, unfortunately. See AlternateUiPlugLabel for the related label info. | [optional]
**is_dummy_plug** | Option<**bool**> | If TRUE, this plug is used for UI display purposes only, and doesn't have any interesting effects of its own. | [optional]
**parent_item_override** | Option<[**crate::models::DestinyDefinitionsItemsDestinyItemPlugDefinitionParentItemOverride**](Destiny_Definitions_Items_DestinyItemPlugDefinition_parentItemOverride.md)> |  | [optional]
**energy_capacity** | Option<[**crate::models::DestinyDefinitionsItemsDestinyItemPlugDefinitionEnergyCapacity**](Destiny_Definitions_Items_DestinyItemPlugDefinition_energyCapacity.md)> |  | [optional]
**energy_cost** | Option<[**crate::models::DestinyDefinitionsItemsDestinyItemPlugDefinitionEnergyCost**](Destiny_Definitions_Items_DestinyItemPlugDefinition_energyCost.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


