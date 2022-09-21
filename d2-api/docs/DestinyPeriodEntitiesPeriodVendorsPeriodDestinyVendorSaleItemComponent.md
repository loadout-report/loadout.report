# DestinyPeriodEntitiesPeriodVendorsPeriodDestinyVendorSaleItemComponent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**sale_status** | Option<**i32**> | A flag indicating whether the requesting character can buy the item, and if not the reasons why the character can't buy it. | [optional]
**required_unlocks** | Option<**Vec<i32>**> | If you can't buy the item due to a complex character state, these will be hashes for DestinyUnlockDefinitions that you can check to see messages regarding the failure (if the unlocks have human readable information: it is not guaranteed that Unlocks will have human readable strings, and your application will have to handle that)  Prefer using failureIndexes instead. These are provided for informational purposes, but have largely been supplanted by failureIndexes. | [optional]
**unlock_statuses** | Option<[**Vec<crate::models::DestinyPeriodDestinyUnlockStatus>**](Destiny.DestinyUnlockStatus.md)> | If any complex unlock states are checked in determining purchasability, these will be returned here along with the status of the unlock check.  Prefer using failureIndexes instead. These are provided for informational purposes, but have largely been supplanted by failureIndexes. | [optional]
**failure_indexes** | Option<**Vec<i32>**> | Indexes in to the \"failureStrings\" lookup table in DestinyVendorDefinition for the given Vendor. Gives some more reliable failure information for why you can't purchase an item.  It is preferred to use these over requiredUnlocks and unlockStatuses: the latter are provided mostly in case someone can do something interesting with it that I didn't anticipate. | [optional]
**augments** | Option<**i32**> | A flags enumeration value representing the current state of any \"state modifiers\" on the item being sold. These are meant to correspond with some sort of visual indicator as to the augmentation: for instance, if an item is on sale or if you already own the item in question.  Determining how you want to represent these in your own app (or if you even want to) is an exercise left for the reader. | [optional]
**item_value_visibility** | Option<**Vec<bool>**> | If available, a list that describes which item values (rewards) should be shown (true) or hidden (false). | [optional]
**vendor_item_index** | Option<**i32**> | The index into the DestinyVendorDefinition.itemList property. Note that this means Vendor data *is* Content Version dependent: make sure you have the latest content before you use Vendor data, or these indexes may mismatch.   Most systems avoid this problem, but Vendors is one area where we are unable to reasonably avoid content dependency at the moment. | [optional]
**item_hash** | Option<**i32**> | The hash of the item being sold, as a quick shortcut for looking up the DestinyInventoryItemDefinition of the sale item. | [optional]
**override_style_item_hash** | Option<**i32**> | If populated, this is the hash of the item whose icon (and other secondary styles, but *not* the human readable strings) should override whatever icons/styles are on the item being sold.  If you don't do this, certain items whose styles are being overridden by socketed items - such as the \"Recycle Shader\" item - would show whatever their default icon/style is, and it wouldn't be pretty or look accurate. | [optional]
**quantity** | Option<**i32**> | How much of the item you'll be getting. | [optional]
**costs** | Option<[**Vec<crate::models::DestinyPeriodDestinyItemQuantity>**](Destiny.DestinyItemQuantity.md)> | A summary of the current costs of the item. | [optional]
**override_next_refresh_date** | Option<**String**> | If this item has its own custom date where it may be removed from the Vendor's rotation, this is that date.  Note that there's not actually any guarantee that it will go away: it could be chosen again and end up still being in the Vendor's sale items! But this is the next date where that test will occur, and is also the date that the game shows for availability on things like Bounties being sold. So it's the best we can give. | [optional]
**api_purchasable** | Option<**bool**> | If true, this item can be purchased through the Bungie.net API. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


