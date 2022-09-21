# DestinyPeriodComponentsPeriodKiosksPeriodDestinyKioskItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**index** | Option<**i32**> | The index of the item in the related DestinyVendorDefintion's itemList property, representing the sale. | [optional]
**can_acquire** | Option<**bool**> | If true, the user can not only see the item, but they can acquire it. It is possible that a user can see a kiosk item and not be able to acquire it. | [optional]
**failure_indexes** | Option<**Vec<i32>**> | Indexes into failureStrings for the Vendor, indicating the reasons why it failed if any. | [optional]
**flavor_objective** | Option<[**crate::models::DestinyComponentsKiosksDestinyKioskItemFlavorObjective**](Destiny_Components_Kiosks_DestinyKioskItem_flavorObjective.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


