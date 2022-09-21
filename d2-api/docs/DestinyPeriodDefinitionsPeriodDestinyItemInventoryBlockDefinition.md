# DestinyPeriodDefinitionsPeriodDestinyItemInventoryBlockDefinition

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**stack_unique_label** | Option<**String**> | If this string is populated, you can't have more than one stack with this label in a given inventory. Note that this is different from the equipping block's unique label, which is used for equipping uniqueness. | [optional]
**max_stack_size** | Option<**i32**> | The maximum quantity of this item that can exist in a stack. | [optional]
**bucket_type_hash** | Option<**i32**> | The hash identifier for the DestinyInventoryBucketDefinition to which this item belongs. I should have named this \"bucketHash\", but too many things refer to it now. Sigh. | [optional]
**recovery_bucket_type_hash** | Option<**i32**> | If the item is picked up by the lost loot queue, this is the hash identifier for the DestinyInventoryBucketDefinition into which it will be placed. Again, I should have named this recoveryBucketHash instead. | [optional]
**tier_type_hash** | Option<**i32**> | The hash identifier for the Tier Type of the item, use to look up its DestinyItemTierTypeDefinition if you need to show localized data for the item's tier. | [optional]
**is_instance_item** | Option<**bool**> | If TRUE, this item is instanced. Otherwise, it is a generic item that merely has a quantity in a stack (like Glimmer). | [optional]
**tier_type_name** | Option<**String**> | The localized name of the tier type, which is a useful shortcut so you don't have to look up the definition every time. However, it's mostly a holdover from days before we had a DestinyItemTierTypeDefinition to refer to. | [optional]
**tier_type** | Option<**i32**> | The enumeration matching the tier type of the item to known values, again for convenience sake. | [optional]
**expiration_tooltip** | Option<**String**> | The tooltip message to show, if any, when the item expires. | [optional]
**expired_in_activity_message** | Option<**String**> | If the item expires while playing in an activity, we show a different message. | [optional]
**expired_in_orbit_message** | Option<**String**> | If the item expires in orbit, we show a... more different message. (\"Consummate V's, consummate!\") | [optional]
**suppress_expiration_when_objectives_complete** | Option<**bool**> |  | [optional]
**recipe_item_hash** | Option<**i32**> | A reference to the associated crafting 'recipe' item definition, if this item can be crafted. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


