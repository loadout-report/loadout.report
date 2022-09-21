# DestinyPeriodDefinitionsPeriodDestinyEquipmentSlotDefinition

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**display_properties** | Option<[**crate::models::DestinyPeriodDefinitionsPeriodCommonPeriodDestinyDisplayPropertiesDefinition**](Destiny.Definitions.Common.DestinyDisplayPropertiesDefinition.md)> |  | [optional]
**equipment_category_hash** | Option<**i32**> | These technically point to \"Equipment Category Definitions\". But don't get excited. There's nothing of significant value in those definitions, so I didn't bother to expose them. You can use the hash here to group equipment slots by common functionality, which serves the same purpose as if we had the Equipment Category definitions exposed. | [optional]
**bucket_type_hash** | Option<**i32**> | The inventory bucket that owns this equipment slot. | [optional]
**apply_custom_art_dyes** | Option<**bool**> | If True, equipped items should have their custom art dyes applied when rendering the item. Otherwise, custom art dyes on an item should be ignored if the item is equipped in this slot. | [optional]
**art_dye_channels** | Option<[**Vec<crate::models::DestinyPeriodDefinitionsPeriodDestinyArtDyeReference>**](Destiny.Definitions.DestinyArtDyeReference.md)> | The Art Dye Channels that apply to this equipment slot. | [optional]
**hash** | Option<**i32**> | The unique identifier for this entity. Guaranteed to be unique for the type of entity, but not globally.  When entities refer to each other in Destiny content, it is this hash that they are referring to. | [optional]
**index** | Option<**i32**> | The index of the entity as it was found in the investment tables. | [optional]
**redacted** | Option<**bool**> | If this is true, then there is an entity with this identifier/type combination, but BNet is not yet allowed to show it. Sorry! | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


