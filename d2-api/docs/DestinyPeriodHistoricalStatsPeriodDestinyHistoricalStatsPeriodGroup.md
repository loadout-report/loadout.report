# DestinyPeriodHistoricalStatsPeriodDestinyHistoricalStatsPeriodGroup

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**period** | Option<**String**> | Period for the group. If the stat periodType is day, then this will have a specific day. If the type is monthly, then this value will be the first day of the applicable month. This value is not set when the periodType is 'all time'. | [optional]
**activity_details** | Option<[**crate::models::DestinyHistoricalStatsDestinyHistoricalStatsPeriodGroupActivityDetails**](Destiny_HistoricalStats_DestinyHistoricalStatsPeriodGroup_activityDetails.md)> |  | [optional]
**values** | Option<[**::std::collections::HashMap<String, crate::models::DestinyPeriodHistoricalStatsPeriodDestinyHistoricalStatsValue>**](Destiny.HistoricalStats.DestinyHistoricalStatsValue.md)> | Collection of stats for the period. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


