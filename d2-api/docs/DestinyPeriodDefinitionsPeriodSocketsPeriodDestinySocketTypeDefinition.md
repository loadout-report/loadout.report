# DestinyPeriodDefinitionsPeriodSocketsPeriodDestinySocketTypeDefinition

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**display_properties** | Option<[**crate::models::DestinyDefinitionsSocketsDestinySocketTypeDefinitionDisplayProperties**](Destiny_Definitions_Sockets_DestinySocketTypeDefinition_displayProperties.md)> |  | [optional]
**insert_action** | Option<[**crate::models::DestinyDefinitionsSocketsDestinySocketTypeDefinitionInsertAction**](Destiny_Definitions_Sockets_DestinySocketTypeDefinition_insertAction.md)> |  | [optional]
**plug_whitelist** | Option<[**Vec<crate::models::DestinyPeriodDefinitionsPeriodSocketsPeriodDestinyPlugWhitelistEntryDefinition>**](Destiny.Definitions.Sockets.DestinyPlugWhitelistEntryDefinition.md)> | A list of Plug \"Categories\" that are allowed to be plugged into sockets of this type.  These should be compared against a given plug item's DestinyInventoryItemDefinition.plug.plugCategoryHash, which indicates the plug item's category.  If the plug's category matches any whitelisted plug, or if the whitelist is empty, it is allowed to be inserted. | [optional]
**socket_category_hash** | Option<**i32**> |  | [optional]
**visibility** | Option<**i32**> | Sometimes a socket isn't visible. These are some of the conditions under which sockets of this type are not visible. Unfortunately, the truth of visibility is much, much more complex. Best to rely on the live data for whether the socket is visible and enabled. | [optional]
**always_randomize_sockets** | Option<**bool**> |  | [optional]
**is_preview_enabled** | Option<**bool**> |  | [optional]
**hide_duplicate_reusable_plugs** | Option<**bool**> |  | [optional]
**overrides_ui_appearance** | Option<**bool**> | This property indicates if the socket type determines whether Emblem icons and nameplates should be overridden by the inserted plug item's icon and nameplate. | [optional]
**avoid_duplicates_on_initialization** | Option<**bool**> |  | [optional]
**currency_scalars** | Option<[**Vec<crate::models::DestinyPeriodDefinitionsPeriodSocketsPeriodDestinySocketTypeScalarMaterialRequirementEntry>**](Destiny.Definitions.Sockets.DestinySocketTypeScalarMaterialRequirementEntry.md)> |  | [optional]
**hash** | Option<**i32**> | The unique identifier for this entity. Guaranteed to be unique for the type of entity, but not globally.  When entities refer to each other in Destiny content, it is this hash that they are referring to. | [optional]
**index** | Option<**i32**> | The index of the entity as it was found in the investment tables. | [optional]
**redacted** | Option<**bool**> | If this is true, then there is an entity with this identifier/type combination, but BNet is not yet allowed to show it. Sorry! | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


