# DestinyPeriodDefinitionsPeriodDestinyItemIntrinsicSocketEntryDefinition

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**plug_item_hash** | Option<**i32**> | Indicates the plug that is intrinsically inserted into this socket. | [optional]
**socket_type_hash** | Option<**i32**> | Indicates the type of this intrinsic socket. | [optional]
**default_visible** | Option<**bool**> | If true, then this socket is visible in the item's \"default\" state. If you have an instance, you should always check the runtime state, as that can override this visibility setting: but if you're looking at the item on a conceptual level, this property can be useful for hiding data such as legacy sockets - which remain defined on items for infrastructure purposes, but can be confusing for users to see. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


