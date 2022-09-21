# DestinyDefinitionsDestinyInventoryItemDefinitionQuality

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**item_levels** | Option<**Vec<i32>**> | The \"base\" defined level of an item. This is a list because, in theory, each Expansion could define its own base level for an item.  In practice, not only was that never done in Destiny 1, but now this isn't even populated at all. When it's not populated, the level at which it spawns has to be inferred by Reward information, of which BNet receives an imperfect view and will only be reliable on instanced data as a result. | [optional]
**quality_level** | Option<**i32**> | qualityLevel is used in combination with the item's level to calculate stats like Attack and Defense. It plays a role in that calculation, but not nearly as large as itemLevel does. | [optional]
**infusion_category_name** | Option<**String**> | The string identifier for this item's \"infusability\", if any.   Items that match the same infusionCategoryName are allowed to infuse with each other.  DEPRECATED: Items can now have multiple infusion categories. Please use infusionCategoryHashes instead. | [optional]
**infusion_category_hash** | Option<**i32**> | The hash identifier for the infusion. It does not map to a Definition entity.  DEPRECATED: Items can now have multiple infusion categories. Please use infusionCategoryHashes instead. | [optional]
**infusion_category_hashes** | Option<**Vec<i32>**> | If any one of these hashes matches any value in another item's infusionCategoryHashes, the two can infuse with each other. | [optional]
**progression_level_requirement_hash** | Option<**i32**> | An item can refer to pre-set level requirements. They are defined in DestinyProgressionLevelRequirementDefinition, and you can use this hash to find the appropriate definition. | [optional]
**current_version** | Option<**i32**> | The latest version available for this item. | [optional]
**versions** | Option<[**Vec<crate::models::DestinyPeriodDefinitionsPeriodDestinyItemVersionDefinition>**](Destiny.Definitions.DestinyItemVersionDefinition.md)> | The list of versions available for this item. | [optional]
**display_version_watermark_icons** | Option<**Vec<String>**> | Icon overlays to denote the item version and power cap status. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


