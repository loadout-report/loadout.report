# DestinyPeriodComponentsPeriodRecordsPeriodDestinyProfileRecordsComponent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**score** | Option<**i32**> | Your 'active' Triumphs score, maintained for backwards compatibility. | [optional]
**active_score** | Option<**i32**> | Your 'active' Triumphs score. | [optional]
**legacy_score** | Option<**i32**> | Your 'legacy' Triumphs score. | [optional]
**lifetime_score** | Option<**i32**> | Your 'lifetime' Triumphs score. | [optional]
**tracked_record_hash** | Option<**i32**> | If this profile is tracking a record, this is the hash identifier of the record it is tracking. | [optional]
**records** | Option<[**::std::collections::HashMap<String, crate::models::DestinyPeriodComponentsPeriodRecordsPeriodDestinyRecordComponent>**](Destiny.Components.Records.DestinyRecordComponent.md)> |  | [optional]
**record_categories_root_node_hash** | Option<**i32**> | The hash for the root presentation node definition of Triumph categories. | [optional]
**record_seals_root_node_hash** | Option<**i32**> | The hash for the root presentation node definition of Triumph Seals. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


