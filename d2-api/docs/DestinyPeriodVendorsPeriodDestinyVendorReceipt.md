# DestinyPeriodVendorsPeriodDestinyVendorReceipt

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**currency_paid** | Option<[**Vec<crate::models::DestinyPeriodDestinyItemQuantity>**](Destiny.DestinyItemQuantity.md)> | The amount paid for the item, in terms of items that were consumed in the purchase and their quantity. | [optional]
**item_received** | Option<[**crate::models::DestinyVendorsDestinyVendorReceiptItemReceived**](Destiny_Vendors_DestinyVendorReceipt_itemReceived.md)> |  | [optional]
**license_unlock_hash** | Option<**i32**> | The unlock flag used to determine whether you still have the purchased item. | [optional]
**purchased_by_character_id** | Option<**i64**> | The ID of the character who made the purchase. | [optional]
**refund_policy** | Option<**i32**> | Whether you can get a refund, and what happens in order for the refund to be received. See the DestinyVendorItemRefundPolicy enum for details. | [optional]
**sequence_number** | Option<**i32**> | The identifier of this receipt. | [optional]
**time_to_expiration** | Option<**i64**> | The seconds since epoch at which this receipt is rendered invalid. | [optional]
**expires_on** | Option<**String**> | The date at which this receipt is rendered invalid. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


