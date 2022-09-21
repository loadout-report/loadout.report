# DestinyPeriodDefinitionsPeriodDestinyVendorCategoryEntryDefinition

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**category_index** | Option<**i32**> | The index of the category in the original category definitions for the vendor. | [optional]
**sort_value** | Option<**i32**> | Used in sorting items in vendors... but there's a lot more to it. Just go with the order provided in the itemIndexes property on the DestinyVendorCategoryComponent instead, it should be more reliable than trying to recalculate it yourself. | [optional]
**category_hash** | Option<**i32**> | The hashed identifier for the category. | [optional]
**quantity_available** | Option<**i32**> | The amount of items that will be available when this category is shown. | [optional]
**show_unavailable_items** | Option<**bool**> | If items aren't up for sale in this category, should we still show them (greyed out)? | [optional]
**hide_if_no_currency** | Option<**bool**> | If you don't have the currency required to buy items from this category, should the items be hidden? | [optional]
**hide_from_regular_purchase** | Option<**bool**> | True if this category doesn't allow purchases. | [optional]
**buy_string_override** | Option<**String**> | The localized string for making purchases from this category, if it is different from the vendor's string for purchasing. | [optional]
**disabled_description** | Option<**String**> | If the category is disabled, this is the localized description to show. | [optional]
**display_title** | Option<**String**> | The localized title of the category. | [optional]
**overlay** | Option<[**crate::models::DestinyDefinitionsDestinyVendorCategoryEntryDefinitionOverlay**](Destiny_Definitions_DestinyVendorCategoryEntryDefinition_overlay.md)> |  | [optional]
**vendor_item_indexes** | Option<**Vec<i32>**> | A shortcut for the vendor item indexes sold under this category. Saves us from some expensive reorganization at runtime. | [optional]
**is_preview** | Option<**bool**> | Sometimes a category isn't actually used to sell items, but rather to preview them. This implies different UI (and manual placement of the category in the UI) in the game, and special treatment. | [optional]
**is_display_only** | Option<**bool**> | If true, this category only displays items: you can't purchase anything in them. | [optional]
**reset_interval_minutes_override** | Option<**i32**> |  | [optional]
**reset_offset_minutes_override** | Option<**i32**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


