# DestinyPeriodDefinitionsPeriodSeasonsPeriodDestinyEventCardDefinition

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**display_properties** | Option<[**crate::models::DestinyPeriodDefinitionsPeriodCommonPeriodDestinyDisplayPropertiesDefinition**](Destiny.Definitions.Common.DestinyDisplayPropertiesDefinition.md)> |  | [optional]
**link_redirect_path** | Option<**String**> |  | [optional]
**color** | Option<[**crate::models::DestinyPeriodMiscPeriodDestinyColor**](Destiny.Misc.DestinyColor.md)> |  | [optional]
**images** | Option<[**crate::models::DestinyPeriodDefinitionsPeriodSeasonsPeriodDestinyEventCardImages**](Destiny.Definitions.Seasons.DestinyEventCardImages.md)> |  | [optional]
**triumphs_presentation_node_hash** | Option<**i32**> |  | [optional]
**seal_presentation_node_hash** | Option<**i32**> |  | [optional]
**ticket_currency_item_hash** | Option<**i32**> |  | [optional]
**ticket_vendor_hash** | Option<**i32**> |  | [optional]
**ticket_vendor_category_hash** | Option<**i32**> |  | [optional]
**end_time** | Option<**i64**> |  | [optional]
**hash** | Option<**i32**> | The unique identifier for this entity. Guaranteed to be unique for the type of entity, but not globally.  When entities refer to each other in Destiny content, it is this hash that they are referring to. | [optional]
**index** | Option<**i32**> | The index of the entity as it was found in the investment tables. | [optional]
**redacted** | Option<**bool**> | If this is true, then there is an entity with this identifier/type combination, but BNet is not yet allowed to show it. Sorry! | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


