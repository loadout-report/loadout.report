# DestinyPeriodDefinitionsPeriodDestinyProgressionRewardItemQuantity

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**rewarded_at_progression_level** | Option<**i32**> |  | [optional]
**acquisition_behavior** | Option<**i32**> |  | [optional]
**ui_display_style** | Option<**String**> |  | [optional]
**claim_unlock_display_strings** | Option<**Vec<String>**> |  | [optional]
**item_hash** | Option<**i32**> | The hash identifier for the item in question. Use it to look up the item's DestinyInventoryItemDefinition. | [optional]
**item_instance_id** | Option<**i64**> | If this quantity is referring to a specific instance of an item, this will have the item's instance ID. Normally, this will be null. | [optional]
**quantity** | Option<**i32**> | The amount of the item needed/available depending on the context of where DestinyItemQuantity is being used. | [optional]
**has_conditional_visibility** | Option<**bool**> | Indicates that this item quantity may be conditionally shown or hidden, based on various sources of state. For example: server flags, account state, or character progress. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


