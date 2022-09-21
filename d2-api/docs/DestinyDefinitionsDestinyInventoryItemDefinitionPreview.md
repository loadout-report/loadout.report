# DestinyDefinitionsDestinyInventoryItemDefinitionPreview

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**screen_style** | Option<**String**> | A string that the game UI uses as a hint for which detail screen to show for the item. You, too, can leverage this for your own custom screen detail views. Note, however, that these are arbitrarily defined by designers: there's no guarantees of a fixed, known number of these - so fall back to something reasonable if you don't recognize it. | [optional]
**preview_vendor_hash** | Option<**i32**> | If the preview data is derived from a fake \"Preview\" Vendor, this will be the hash identifier for the DestinyVendorDefinition of that fake vendor. | [optional]
**artifact_hash** | Option<**i32**> | If this item should show you Artifact information when you preview it, this is the hash identifier of the DestinyArtifactDefinition for the artifact whose data should be shown. | [optional]
**preview_action_string** | Option<**String**> | If the preview has an associated action (like \"Open\"), this will be the localized string for that action. | [optional]
**derived_item_categories** | Option<[**Vec<crate::models::DestinyPeriodDefinitionsPeriodItemsPeriodDestinyDerivedItemCategoryDefinition>**](Destiny.Definitions.Items.DestinyDerivedItemCategoryDefinition.md)> | This is a list of the items being previewed, categorized in the same way as they are in the preview UI. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


