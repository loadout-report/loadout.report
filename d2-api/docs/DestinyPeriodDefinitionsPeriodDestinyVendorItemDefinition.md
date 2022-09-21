# DestinyPeriodDefinitionsPeriodDestinyVendorItemDefinition

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**vendor_item_index** | Option<**i32**> | The index into the DestinyVendorDefinition.saleList. This is what we use to refer to items being sold throughout live and definition data. | [optional]
**item_hash** | Option<**i32**> | The hash identifier of the item being sold (DestinyInventoryItemDefinition).  Note that a vendor can sell the same item in multiple ways, so don't assume that itemHash is a unique identifier for this entity. | [optional]
**quantity** | Option<**i32**> | The amount you will recieve of the item described in itemHash if you make the purchase. | [optional]
**failure_indexes** | Option<**Vec<i32>**> | An list of indexes into the DestinyVendorDefinition.failureStrings array, indicating the possible failure strings that can be relevant for this item. | [optional]
**currencies** | Option<[**Vec<crate::models::DestinyPeriodDefinitionsPeriodDestinyVendorItemQuantity>**](Destiny.Definitions.DestinyVendorItemQuantity.md)> | This is a pre-compiled aggregation of item value and priceOverrideList, so that we have one place to check for what the purchaser must pay for the item. Use this instead of trying to piece together the price separately.  The somewhat crappy part about this is that, now that item quantity overrides have dynamic modifiers, this will not necessarily be statically true. If you were using this instead of live data, switch to using live data. | [optional]
**refund_policy** | Option<**i32**> | If this item can be refunded, this is the policy for what will be refundd, how, and in what time period. | [optional]
**refund_time_limit** | Option<**i32**> | The amount of time before refundability of the newly purchased item will expire. | [optional]
**creation_levels** | Option<[**Vec<crate::models::DestinyPeriodDefinitionsPeriodDestinyItemCreationEntryLevelDefinition>**](Destiny.Definitions.DestinyItemCreationEntryLevelDefinition.md)> | The Default level at which the item will spawn. Almost always driven by an adjusto these days. Ideally should be singular. It's a long story how this ended up as a list, but there is always either going to be 0:1 of these entities. | [optional]
**display_category_index** | Option<**i32**> | This is an index specifically into the display category, as opposed to the server-side Categories (which do not need to match or pair with each other in any way: server side categories are really just structures for common validation. Display Category will let us more easily categorize items visually) | [optional]
**category_index** | Option<**i32**> | The index into the DestinyVendorDefinition.categories array, so you can find the category associated with this item. | [optional]
**original_category_index** | Option<**i32**> | Same as above, but for the original category indexes. | [optional]
**minimum_level** | Option<**i32**> | The minimum character level at which this item is available for sale. | [optional]
**maximum_level** | Option<**i32**> | The maximum character level at which this item is available for sale. | [optional]
**action** | Option<[**crate::models::DestinyDefinitionsDestinyVendorItemDefinitionAction**](Destiny_Definitions_DestinyVendorItemDefinition_action.md)> |  | [optional]
**display_category** | Option<**String**> | The string identifier for the category selling this item. | [optional]
**inventory_bucket_hash** | Option<**i32**> | The inventory bucket into which this item will be placed upon purchase. | [optional]
**visibility_scope** | Option<**i32**> | The most restrictive scope that determines whether the item is available in the Vendor's inventory. See DestinyGatingScope's documentation for more information.  This can be determined by Unlock gating, or by whether or not the item has purchase level requirements (minimumLevel and maximumLevel properties). | [optional]
**purchasable_scope** | Option<**i32**> | Similar to visibilityScope, it represents the most restrictive scope that determines whether the item can be purchased. It will at least be as restrictive as visibilityScope, but could be more restrictive if the item has additional purchase requirements beyond whether it is merely visible or not.  See DestinyGatingScope's documentation for more information. | [optional]
**exclusivity** | Option<**i32**> | If this item can only be purchased by a given platform, this indicates the platform to which it is restricted. | [optional]
**is_offer** | Option<**bool**> | If this sale can only be performed as the result of an offer check, this is true. | [optional]
**is_crm** | Option<**bool**> | If this sale can only be performed as the result of receiving a CRM offer, this is true. | [optional]
**sort_value** | Option<**i32**> | *if* the category this item is in supports non-default sorting, this value should represent the sorting value to use, pre-processed and ready to go. | [optional]
**expiration_tooltip** | Option<**String**> | If this item can expire, this is the tooltip message to show with its expiration info. | [optional]
**redirect_to_sale_indexes** | Option<**Vec<i32>**> | If this is populated, the purchase of this item should redirect to purchasing these other items instead. | [optional]
**socket_overrides** | Option<[**Vec<crate::models::DestinyPeriodDefinitionsPeriodDestinyVendorItemSocketOverride>**](Destiny.Definitions.DestinyVendorItemSocketOverride.md)> |  | [optional]
**unpurchasable** | Option<**bool**> | If true, this item is some sort of dummy sale item that cannot actually be purchased. It may be a display only item, or some fluff left by a content designer for testing purposes, or something that got disabled because it was a terrible idea. You get the picture. We won't know *why* it can't be purchased, only that it can't be. Sorry.  This is also only whether it's unpurchasable as a static property according to game content. There are other reasons why an item may or may not be purchasable at runtime, so even if this isn't set to True you should trust the runtime value for this sale item over the static definition if this is unset. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


