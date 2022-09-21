# DestinyPeriodEntitiesPeriodCharactersPeriodDestinyCharacterComponent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**membership_id** | Option<**i64**> | Every Destiny Profile has a membershipId. This is provided on the character as well for convenience. | [optional]
**membership_type** | Option<**i32**> | membershipType tells you the platform on which the character plays. Examine the BungieMembershipType enumeration for possible values. | [optional]
**character_id** | Option<**i64**> | The unique identifier for the character. | [optional]
**date_last_played** | Option<**String**> | The last date that the user played Destiny. | [optional]
**minutes_played_this_session** | Option<**i64**> | If the user is currently playing, this is how long they've been playing. | [optional]
**minutes_played_total** | Option<**i64**> | If this value is 525,600, then they played Destiny for a year. Or they're a very dedicated Rent fan. Note that this includes idle time, not just time spent actually in activities shooting things. | [optional]
**light** | Option<**i32**> | The user's calculated \"Light Level\". Light level is an indicator of your power that mostly matters in the end game, once you've reached the maximum character level: it's a level that's dependent on the average Attack/Defense power of your items. | [optional]
**stats** | Option<**::std::collections::HashMap<String, i32>**> | Your character's stats, such as Agility, Resilience, etc... *not* historical stats.  You'll have to call a different endpoint for those. | [optional]
**race_hash** | Option<**i32**> | Use this hash to look up the character's DestinyRaceDefinition. | [optional]
**gender_hash** | Option<**i32**> | Use this hash to look up the character's DestinyGenderDefinition. | [optional]
**class_hash** | Option<**i32**> | Use this hash to look up the character's DestinyClassDefinition. | [optional]
**race_type** | Option<**i32**> | Mostly for historical purposes at this point, this is an enumeration for the character's race.  It'll be preferable in the general case to look up the related definition: but for some people this was too convenient to remove. | [optional]
**class_type** | Option<**i32**> | Mostly for historical purposes at this point, this is an enumeration for the character's class.  It'll be preferable in the general case to look up the related definition: but for some people this was too convenient to remove. | [optional]
**gender_type** | Option<**i32**> | Mostly for historical purposes at this point, this is an enumeration for the character's Gender.  It'll be preferable in the general case to look up the related definition: but for some people this was too convenient to remove. And yeah, it's an enumeration and not a boolean. Fight me. | [optional]
**emblem_path** | Option<**String**> | A shortcut path to the user's currently equipped emblem image. If you're just showing summary info for a user, this is more convenient than examining their equipped emblem and looking up the definition. | [optional]
**emblem_background_path** | Option<**String**> | A shortcut path to the user's currently equipped emblem background image. If you're just showing summary info for a user, this is more convenient than examining their equipped emblem and looking up the definition. | [optional]
**emblem_hash** | Option<**i32**> | The hash of the currently equipped emblem for the user. Can be used to look up the DestinyInventoryItemDefinition. | [optional]
**emblem_color** | Option<[**crate::models::DestinyEntitiesCharactersDestinyCharacterComponentEmblemColor**](Destiny_Entities_Characters_DestinyCharacterComponent_emblemColor.md)> |  | [optional]
**level_progression** | Option<[**crate::models::DestinyEntitiesCharactersDestinyCharacterComponentLevelProgression**](Destiny_Entities_Characters_DestinyCharacterComponent_levelProgression.md)> |  | [optional]
**base_character_level** | Option<**i32**> | The \"base\" level of your character, not accounting for any light level. | [optional]
**percent_to_next_level** | Option<**f32**> | A number between 0 and 100, indicating the whole and fractional % remaining to get to the next character level. | [optional]
**title_record_hash** | Option<**i32**> | If this Character has a title assigned to it, this is the identifier of the DestinyRecordDefinition that has that title information. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


