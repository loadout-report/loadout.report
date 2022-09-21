# DestinyPeriodDefinitionsPeriodDestinyDisplayCategoryDefinition

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**index** | Option<**i32**> |  | [optional]
**identifier** | Option<**String**> | A string identifier for the display category. | [optional]
**display_category_hash** | Option<**i32**> |  | [optional]
**display_properties** | Option<[**crate::models::DestinyPeriodDefinitionsPeriodCommonPeriodDestinyDisplayPropertiesDefinition**](Destiny.Definitions.Common.DestinyDisplayPropertiesDefinition.md)> |  | [optional]
**display_in_banner** | Option<**bool**> | If true, this category should be displayed in the \"Banner\" section of the vendor's UI. | [optional]
**progression_hash** | Option<**i32**> | If it exists, this is the hash identifier of a DestinyProgressionDefinition that represents the progression to show on this display category.  Specific categories can now have thier own distinct progression, apparently. So that's cool. | [optional]
**sort_order** | Option<**i32**> | If this category sorts items in a nonstandard way, this will be the way we sort. | [optional]
**display_style_hash** | Option<**i32**> | An indicator of how the category will be displayed in the UI. It's up to you to do something cool or interesting in response to this, or just to treat it as a normal category. | [optional]
**display_style_identifier** | Option<**String**> | An indicator of how the category will be displayed in the UI. It's up to you to do something cool or interesting in response to this, or just to treat it as a normal category. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


