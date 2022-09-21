# DestinyPeriodDefinitionsPeriodDestinyVendorInventoryFlyoutDefinition

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**locked_description** | Option<**String**> | If the flyout is locked, this is the reason why. | [optional]
**display_properties** | Option<[**crate::models::DestinyDefinitionsDestinyVendorInventoryFlyoutDefinitionDisplayProperties**](Destiny_Definitions_DestinyVendorInventoryFlyoutDefinition_displayProperties.md)> |  | [optional]
**buckets** | Option<[**Vec<crate::models::DestinyPeriodDefinitionsPeriodDestinyVendorInventoryFlyoutBucketDefinition>**](Destiny.Definitions.DestinyVendorInventoryFlyoutBucketDefinition.md)> | A list of inventory buckets and other metadata to show on the screen. | [optional]
**flyout_id** | Option<**i32**> | An identifier for the flyout, in case anything else needs to refer to them. | [optional]
**suppress_newness** | Option<**bool**> | If this is true, don't show any of the glistening \"this is a new item\" UI elements, like we show on the inventory items themselves in in-game UI. | [optional]
**equipment_slot_hash** | Option<**i32**> | If this flyout is meant to show you the contents of the player's equipment slot, this is the slot to show. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


