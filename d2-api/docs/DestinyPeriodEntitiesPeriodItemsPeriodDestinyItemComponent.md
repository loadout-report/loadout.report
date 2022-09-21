# DestinyPeriodEntitiesPeriodItemsPeriodDestinyItemComponent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**item_hash** | Option<**i32**> | The identifier for the item's definition, which is where most of the useful static information for the item can be found. | [optional]
**item_instance_id** | Option<**i64**> | If the item is instanced, it will have an instance ID. Lack of an instance ID implies that the item has no distinct local qualities aside from stack size. | [optional]
**quantity** | Option<**i32**> | The quantity of the item in this stack. Note that Instanced items cannot stack. If an instanced item, this value will always be 1 (as the stack has exactly one item in it) | [optional]
**bind_status** | Option<**i32**> | If the item is bound to a location, it will be specified in this enum. | [optional]
**location** | Option<**i32**> | An easy reference for where the item is located. Redundant if you got the item from an Inventory, but useful when making detail calls on specific items. | [optional]
**bucket_hash** | Option<**i32**> | The hash identifier for the specific inventory bucket in which the item is located. | [optional]
**transfer_status** | Option<**i32**> | If there is a known error state that would cause this item to not be transferable, this Flags enum will indicate all of those error states. Otherwise, it will be 0 (CanTransfer). | [optional]
**lockable** | Option<**bool**> | If the item can be locked, this will indicate that state. | [optional]
**state** | Option<**i32**> | A flags enumeration indicating the transient/custom states of the item that affect how it is rendered: whether it's tracked or locked for example, or whether it has a masterwork plug inserted. | [optional]
**override_style_item_hash** | Option<**i32**> | If populated, this is the hash of the item whose icon (and other secondary styles, but *not* the human readable strings) should override whatever icons/styles are on the item being sold.  If you don't do this, certain items whose styles are being overridden by socketed items - such as the \"Recycle Shader\" item - would show whatever their default icon/style is, and it wouldn't be pretty or look accurate. | [optional]
**expiration_date** | Option<**String**> | If the item can expire, this is the date at which it will/did expire. | [optional]
**is_wrapper** | Option<**bool**> | If this is true, the object is actually a \"wrapper\" of the object it's representing. This means that it's not the actual item itself, but rather an item that must be \"opened\" in game before you have and can use the item.   Wrappers are an evolution of \"bundles\", which give an easy way to let you preview the contents of what you purchased while still letting you get a refund before you \"open\" it. | [optional]
**tooltip_notification_indexes** | Option<**Vec<i32>**> | If this is populated, it is a list of indexes into DestinyInventoryItemDefinition.tooltipNotifications for any special tooltip messages that need to be shown for this item. | [optional]
**metric_hash** | Option<**i32**> | The identifier for the currently-selected metric definition, to be displayed on the emblem nameplate. | [optional]
**metric_objective** | Option<[**crate::models::DestinyEntitiesItemsDestinyItemComponentMetricObjective**](Destiny_Entities_Items_DestinyItemComponent_metricObjective.md)> |  | [optional]
**version_number** | Option<**i32**> | The version of this item, used to index into the versions list in the item definition quality block. | [optional]
**item_value_visibility** | Option<**Vec<bool>**> | If available, a list that describes which item values (rewards) should be shown (true) or hidden (false). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


