# DestinyPeriodDefinitionsPeriodRecordsPeriodDestinyRecordDefinition

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**display_properties** | Option<[**crate::models::DestinyPeriodDefinitionsPeriodCommonPeriodDestinyDisplayPropertiesDefinition**](Destiny.Definitions.Common.DestinyDisplayPropertiesDefinition.md)> |  | [optional]
**scope** | Option<**i32**> | Indicates whether this Record's state is determined on a per-character or on an account-wide basis. | [optional]
**presentation_info** | Option<[**crate::models::DestinyPeriodDefinitionsPeriodPresentationPeriodDestinyPresentationChildBlock**](Destiny.Definitions.Presentation.DestinyPresentationChildBlock.md)> |  | [optional]
**lore_hash** | Option<**i32**> |  | [optional]
**objective_hashes** | Option<**Vec<i32>**> |  | [optional]
**record_value_style** | Option<**i32**> |  | [optional]
**for_title_gilding** | Option<**bool**> |  | [optional]
**should_show_large_icons** | Option<**bool**> | A hint to show a large icon for a reward | [optional]
**title_info** | Option<[**crate::models::DestinyPeriodDefinitionsPeriodRecordsPeriodDestinyRecordTitleBlock**](Destiny.Definitions.Records.DestinyRecordTitleBlock.md)> |  | [optional]
**completion_info** | Option<[**crate::models::DestinyPeriodDefinitionsPeriodRecordsPeriodDestinyRecordCompletionBlock**](Destiny.Definitions.Records.DestinyRecordCompletionBlock.md)> |  | [optional]
**state_info** | Option<[**crate::models::DestinyPeriodDefinitionsPeriodRecordsPeriodSchemaRecordStateBlock**](Destiny.Definitions.Records.SchemaRecordStateBlock.md)> |  | [optional]
**requirements** | Option<[**crate::models::DestinyPeriodDefinitionsPeriodPresentationPeriodDestinyPresentationNodeRequirementsBlock**](Destiny.Definitions.Presentation.DestinyPresentationNodeRequirementsBlock.md)> |  | [optional]
**expiration_info** | Option<[**crate::models::DestinyPeriodDefinitionsPeriodRecordsPeriodDestinyRecordExpirationBlock**](Destiny.Definitions.Records.DestinyRecordExpirationBlock.md)> |  | [optional]
**interval_info** | Option<[**crate::models::DestinyDefinitionsRecordsDestinyRecordDefinitionIntervalInfo**](Destiny_Definitions_Records_DestinyRecordDefinition_intervalInfo.md)> |  | [optional]
**reward_items** | Option<[**Vec<crate::models::DestinyPeriodDestinyItemQuantity>**](Destiny.DestinyItemQuantity.md)> | If there is any publicly available information about rewards earned for achieving this record, this is the list of those items.   However, note that some records intentionally have \"hidden\" rewards. These will not be returned in this list. | [optional]
**presentation_node_type** | Option<**i32**> |  | [optional]
**trait_ids** | Option<**Vec<String>**> |  | [optional]
**trait_hashes** | Option<**Vec<i32>**> |  | [optional]
**parent_node_hashes** | Option<**Vec<i32>**> | A quick reference to presentation nodes that have this node as a child. Presentation nodes can be parented under multiple parents. | [optional]
**hash** | Option<**i32**> | The unique identifier for this entity. Guaranteed to be unique for the type of entity, but not globally.  When entities refer to each other in Destiny content, it is this hash that they are referring to. | [optional]
**index** | Option<**i32**> | The index of the entity as it was found in the investment tables. | [optional]
**redacted** | Option<**bool**> | If this is true, then there is an entity with this identifier/type combination, but BNet is not yet allowed to show it. Sorry! | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


