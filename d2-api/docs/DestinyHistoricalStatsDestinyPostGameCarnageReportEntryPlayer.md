# DestinyHistoricalStatsDestinyPostGameCarnageReportEntryPlayer

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**destiny_user_info** | Option<[**crate::models::DestinyHistoricalStatsDestinyPlayerDestinyUserInfo**](Destiny_HistoricalStats_DestinyPlayer_destinyUserInfo.md)> |  | [optional]
**character_class** | Option<**String**> | Class of the character if applicable and available. | [optional]
**class_hash** | Option<**i32**> |  | [optional]
**race_hash** | Option<**i32**> |  | [optional]
**gender_hash** | Option<**i32**> |  | [optional]
**character_level** | Option<**i32**> | Level of the character if available. Zero if it is not available. | [optional]
**light_level** | Option<**i32**> | Light Level of the character if available. Zero if it is not available. | [optional]
**bungie_net_user_info** | Option<[**crate::models::DestinyHistoricalStatsDestinyPlayerBungieNetUserInfo**](Destiny_HistoricalStats_DestinyPlayer_bungieNetUserInfo.md)> |  | [optional]
**clan_name** | Option<**String**> | Current clan name for the player. This value may be null or an empty string if the user does not have a clan. | [optional]
**clan_tag** | Option<**String**> | Current clan tag for the player. This value may be null or an empty string if the user does not have a clan. | [optional]
**emblem_hash** | Option<**i32**> | If we know the emblem's hash, this can be used to look up the player's emblem at the time of a match when receiving PGCR data, or otherwise their currently equipped emblem (if we are able to obtain it). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


