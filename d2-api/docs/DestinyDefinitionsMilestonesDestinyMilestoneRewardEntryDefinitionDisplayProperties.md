# DestinyDefinitionsMilestonesDestinyMilestoneRewardEntryDefinitionDisplayProperties

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | Option<**String**> |  | [optional]
**name** | Option<**String**> |  | [optional]
**icon** | Option<**String**> | Note that \"icon\" is sometimes misleading, and should be interpreted in the context of the entity. For instance, in Destiny 1 the DestinyRecordBookDefinition's icon was a big picture of a book.  But usually, it will be a small square image that you can use as... well, an icon.  They are currently represented as 96px x 96px images. | [optional]
**icon_sequences** | Option<[**Vec<crate::models::DestinyPeriodDefinitionsPeriodCommonPeriodDestinyIconSequenceDefinition>**](Destiny.Definitions.Common.DestinyIconSequenceDefinition.md)> |  | [optional]
**high_res_icon** | Option<**String**> | If this item has a high-res icon (at least for now, many things won't), then the path to that icon will be here. | [optional]
**has_icon** | Option<**bool**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


