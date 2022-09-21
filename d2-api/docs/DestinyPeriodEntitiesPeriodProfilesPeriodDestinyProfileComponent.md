# DestinyPeriodEntitiesPeriodProfilesPeriodDestinyProfileComponent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**user_info** | Option<[**crate::models::DestinyEntitiesProfilesDestinyProfileComponentUserInfo**](Destiny_Entities_Profiles_DestinyProfileComponent_userInfo.md)> |  | [optional]
**date_last_played** | Option<**String**> | The last time the user played with any character on this Profile. | [optional]
**versions_owned** | Option<**i32**> | If you want to know what expansions they own, this will contain that data.   IMPORTANT: This field may not return the data you're interested in for Cross-Saved users. It returns the last ownership data we saw for this account - which is to say, what they've purchased on the platform on which they last played, which now could be a different platform.   If you don't care about per-platform ownership and only care about whatever platform it seems they are playing on most recently, then this should be \"good enough.\" Otherwise, this should be considered deprecated. We do not have a good alternative to provide at this time with platform specific ownership data for DLC. | [optional]
**character_ids** | Option<**Vec<i64>**> | A list of the character IDs, for further querying on your part. | [optional]
**season_hashes** | Option<**Vec<i32>**> | A list of seasons that this profile owns. Unlike versionsOwned, these stay with the profile across Platforms, and thus will be valid.   It turns out that Stadia Pro subscriptions will give access to seasons but only while playing on Stadia and with an active subscription. So some users (users who have Stadia Pro but choose to play on some other platform) won't see these as available: it will be whatever seasons are available for the platform on which they last played. | [optional]
**event_card_hashes_owned** | Option<**Vec<i32>**> | A list of hashes for event cards that a profile owns. Unlike most values in versionsOwned, these stay with the profile across all platforms. | [optional]
**current_season_hash** | Option<**i32**> | If populated, this is a reference to the season that is currently active. | [optional]
**current_season_reward_power_cap** | Option<**i32**> | If populated, this is the reward power cap for the current season. | [optional]
**active_event_card_hash** | Option<**i32**> | If populated, this is a reference to the event card that is currently active. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


