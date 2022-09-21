# DestinyPeriodDefinitionsPeriodDestinyActivityPlaylistItemDefinition

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**activity_hash** | Option<**i32**> | The hash identifier of the Activity that can be played. Use it to look up the DestinyActivityDefinition. | [optional]
**direct_activity_mode_hash** | Option<**i32**> | If this playlist entry had an activity mode directly defined on it, this will be the hash of that mode. | [optional]
**direct_activity_mode_type** | Option<**i32**> | If the playlist entry had an activity mode directly defined on it, this will be the enum value of that mode. | [optional]
**activity_mode_hashes** | Option<**Vec<i32>**> | The hash identifiers for Activity Modes relevant to this entry. | [optional]
**activity_mode_types** | Option<**Vec<i32>**> | The activity modes - if any - in enum form. Because we can't seem to escape the enums. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


