# DestinyPeriodProgressionPeriodDestinyFactionProgression

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**faction_hash** | Option<**i32**> | The hash identifier of the Faction related to this progression. Use it to look up the DestinyFactionDefinition for more rendering info. | [optional]
**faction_vendor_index** | Option<**i32**> | The index of the Faction vendor that is currently available. Will be set to -1 if no vendors are available. | [optional]
**progression_hash** | Option<**i32**> | The hash identifier of the Progression in question. Use it to look up the DestinyProgressionDefinition in static data. | [optional]
**daily_progress** | Option<**i32**> | The amount of progress earned today for this progression. | [optional]
**daily_limit** | Option<**i32**> | If this progression has a daily limit, this is that limit. | [optional]
**weekly_progress** | Option<**i32**> | The amount of progress earned toward this progression in the current week. | [optional]
**weekly_limit** | Option<**i32**> | If this progression has a weekly limit, this is that limit. | [optional]
**current_progress** | Option<**i32**> | This is the total amount of progress obtained overall for this progression (for instance, the total amount of Character Level experience earned) | [optional]
**level** | Option<**i32**> | This is the level of the progression (for instance, the Character Level). | [optional]
**level_cap** | Option<**i32**> | This is the maximum possible level you can achieve for this progression (for example, the maximum character level obtainable) | [optional]
**step_index** | Option<**i32**> | Progressions define their levels in \"steps\". Since the last step may be repeatable, the user may be at a higher level than the actual Step achieved in the progression. Not necessarily useful, but potentially interesting for those cruising the API. Relate this to the \"steps\" property of the DestinyProgression to see which step the user is on, if you care about that. (Note that this is Content Version dependent since it refers to indexes.) | [optional]
**progress_to_next_level** | Option<**i32**> | The amount of progression (i.e. \"Experience\") needed to reach the next level of this Progression. Jeez, progression is such an overloaded word. | [optional]
**next_level_at** | Option<**i32**> | The total amount of progression (i.e. \"Experience\") needed in order to reach the next level. | [optional]
**current_reset_count** | Option<**i32**> | The number of resets of this progression you've executed this season, if applicable to this progression. | [optional]
**season_resets** | Option<[**Vec<crate::models::DestinyPeriodDestinyProgressionResetEntry>**](Destiny.DestinyProgressionResetEntry.md)> | Information about historical resets of this progression, if there is any data for it. | [optional]
**reward_item_states** | Option<**Vec<i32>**> | Information about historical rewards for this progression, if there is any data for it. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


