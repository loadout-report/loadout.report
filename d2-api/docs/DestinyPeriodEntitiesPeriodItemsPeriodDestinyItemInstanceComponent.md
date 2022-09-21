# DestinyPeriodEntitiesPeriodItemsPeriodDestinyItemInstanceComponent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**damage_type** | Option<**i32**> | If the item has a damage type, this is the item's current damage type. | [optional]
**damage_type_hash** | Option<**i32**> | The current damage type's hash, so you can look up localized info and icons for it. | [optional]
**primary_stat** | Option<[**crate::models::DestinyEntitiesItemsDestinyItemInstanceComponentPrimaryStat**](Destiny_Entities_Items_DestinyItemInstanceComponent_primaryStat.md)> |  | [optional]
**item_level** | Option<**i32**> | The Item's \"Level\" has the most significant bearing on its stats, such as Light and Power. | [optional]
**quality** | Option<**i32**> | The \"Quality\" of the item has a lesser - but still impactful - bearing on stats like Light and Power. | [optional]
**is_equipped** | Option<**bool**> | Is the item currently equipped on the given character? | [optional]
**can_equip** | Option<**bool**> | If this is an equippable item, you can check it here. There are permanent as well as transitory reasons why an item might not be able to be equipped: check cannotEquipReason for details. | [optional]
**equip_required_level** | Option<**i32**> | If the item cannot be equipped until you reach a certain level, that level will be reflected here. | [optional]
**unlock_hashes_required_to_equip** | Option<**Vec<i32>**> | Sometimes, there are limitations to equipping that are represented by character-level flags called \"unlocks\".  This is a list of flags that they need in order to equip the item that the character has not met. Use these to look up the descriptions to show in your UI by looking up the relevant DestinyUnlockDefinitions for the hashes. | [optional]
**cannot_equip_reason** | Option<**i32**> | If you cannot equip the item, this is a flags enum that enumerates all of the reasons why you couldn't equip the item. You may need to refine your UI further by using unlockHashesRequiredToEquip and equipRequiredLevel. | [optional]
**breaker_type** | Option<**i32**> | If populated, this item has a breaker type corresponding to the given value. See DestinyBreakerTypeDefinition for more details. | [optional]
**breaker_type_hash** | Option<**i32**> | If populated, this is the hash identifier for the item's breaker type. See DestinyBreakerTypeDefinition for more details. | [optional]
**energy** | Option<[**crate::models::DestinyEntitiesItemsDestinyItemInstanceComponentEnergy**](Destiny_Entities_Items_DestinyItemInstanceComponent_energy.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


