# DestinyPeriodHistoricalStatsPeriodDestinyPostGameCarnageReportData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**period** | Option<**String**> | Date and time for the activity. | [optional]
**starting_phase_index** | Option<**i32**> | If this activity has \"phases\", this is the phase at which the activity was started. This value is only valid for activities before the Beyond Light expansion shipped. Subsequent activities will not have a valid value here. | [optional]
**activity_was_started_from_beginning** | Option<**bool**> | True if the activity was started from the beginning, if that information is available and the activity was played post Witch Queen release. | [optional]
**activity_details** | Option<[**crate::models::DestinyHistoricalStatsDestinyPostGameCarnageReportDataActivityDetails**](Destiny_HistoricalStats_DestinyPostGameCarnageReportData_activityDetails.md)> |  | [optional]
**entries** | Option<[**Vec<crate::models::DestinyPeriodHistoricalStatsPeriodDestinyPostGameCarnageReportEntry>**](Destiny.HistoricalStats.DestinyPostGameCarnageReportEntry.md)> | Collection of players and their data for this activity. | [optional]
**teams** | Option<[**Vec<crate::models::DestinyPeriodHistoricalStatsPeriodDestinyPostGameCarnageReportTeamEntry>**](Destiny.HistoricalStats.DestinyPostGameCarnageReportTeamEntry.md)> | Collection of stats for the player in this activity. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


