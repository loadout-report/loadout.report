# DestinyRequestsActionsDestinyInsertPlugsActionRequestPlug

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**socket_index** | Option<**i32**> | The index into the socket array, which identifies the specific socket being operated on. We also need to know the socketArrayType in order to uniquely identify the socket.  Don't point to or try to insert a plug into an infusion socket. It won't work. | [optional]
**socket_array_type** | Option<**i32**> | This property, combined with the socketIndex, tells us which socket we are referring to (since operations can be performed on both Intrinsic and \"default\" sockets, and they occupy different arrays in the Inventory Item Definition). I know, I know. Don't give me that look. | [optional]
**plug_item_hash** | Option<**i32**> | Plugs are never instanced (except in infusion). So with the hash alone, we should be able to: 1) Infer whether the player actually needs to have the item, or if it's a reusable plug 2) Perform any operation needed to use the Plug, including removing the plug item and running reward sheets. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


