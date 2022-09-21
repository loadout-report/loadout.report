/*
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * The version of the OpenAPI document: 2.16.0
 * Contact: support@bungie.com
 * Generated by: https://openapi-generator.tech
 */

/// DestinyPeriodEntitiesPeriodCharactersPeriodDestinyCharacterComponent : This component contains base properties of the character. You'll probably want to always request this component, but hey you do you.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DestinyPeriodEntitiesPeriodCharactersPeriodDestinyCharacterComponent {
    /// Every Destiny Profile has a membershipId. This is provided on the character as well for convenience.
    #[serde(rename = "membershipId", skip_serializing_if = "Option::is_none")]
    pub membership_id: Option<i64>,
    /// membershipType tells you the platform on which the character plays. Examine the BungieMembershipType enumeration for possible values.
    #[serde(rename = "membershipType", skip_serializing_if = "Option::is_none")]
    pub membership_type: Option<i32>,
    /// The unique identifier for the character.
    #[serde(rename = "characterId", skip_serializing_if = "Option::is_none")]
    pub character_id: Option<i64>,
    /// The last date that the user played Destiny.
    #[serde(rename = "dateLastPlayed", skip_serializing_if = "Option::is_none")]
    pub date_last_played: Option<String>,
    /// If the user is currently playing, this is how long they've been playing.
    #[serde(rename = "minutesPlayedThisSession", skip_serializing_if = "Option::is_none")]
    pub minutes_played_this_session: Option<i64>,
    /// If this value is 525,600, then they played Destiny for a year. Or they're a very dedicated Rent fan. Note that this includes idle time, not just time spent actually in activities shooting things.
    #[serde(rename = "minutesPlayedTotal", skip_serializing_if = "Option::is_none")]
    pub minutes_played_total: Option<i64>,
    /// The user's calculated \"Light Level\". Light level is an indicator of your power that mostly matters in the end game, once you've reached the maximum character level: it's a level that's dependent on the average Attack/Defense power of your items.
    #[serde(rename = "light", skip_serializing_if = "Option::is_none")]
    pub light: Option<i32>,
    /// Your character's stats, such as Agility, Resilience, etc... *not* historical stats.  You'll have to call a different endpoint for those.
    #[serde(rename = "stats", skip_serializing_if = "Option::is_none")]
    pub stats: Option<::std::collections::HashMap<String, i32>>,
    /// Use this hash to look up the character's DestinyRaceDefinition.
    #[serde(rename = "raceHash", skip_serializing_if = "Option::is_none")]
    pub race_hash: Option<i32>,
    /// Use this hash to look up the character's DestinyGenderDefinition.
    #[serde(rename = "genderHash", skip_serializing_if = "Option::is_none")]
    pub gender_hash: Option<i32>,
    /// Use this hash to look up the character's DestinyClassDefinition.
    #[serde(rename = "classHash", skip_serializing_if = "Option::is_none")]
    pub class_hash: Option<i32>,
    /// Mostly for historical purposes at this point, this is an enumeration for the character's race.  It'll be preferable in the general case to look up the related definition: but for some people this was too convenient to remove.
    #[serde(rename = "raceType", skip_serializing_if = "Option::is_none")]
    pub race_type: Option<i32>,
    /// Mostly for historical purposes at this point, this is an enumeration for the character's class.  It'll be preferable in the general case to look up the related definition: but for some people this was too convenient to remove.
    #[serde(rename = "classType", skip_serializing_if = "Option::is_none")]
    pub class_type: Option<i32>,
    /// Mostly for historical purposes at this point, this is an enumeration for the character's Gender.  It'll be preferable in the general case to look up the related definition: but for some people this was too convenient to remove. And yeah, it's an enumeration and not a boolean. Fight me.
    #[serde(rename = "genderType", skip_serializing_if = "Option::is_none")]
    pub gender_type: Option<i32>,
    /// A shortcut path to the user's currently equipped emblem image. If you're just showing summary info for a user, this is more convenient than examining their equipped emblem and looking up the definition.
    #[serde(rename = "emblemPath", skip_serializing_if = "Option::is_none")]
    pub emblem_path: Option<String>,
    /// A shortcut path to the user's currently equipped emblem background image. If you're just showing summary info for a user, this is more convenient than examining their equipped emblem and looking up the definition.
    #[serde(rename = "emblemBackgroundPath", skip_serializing_if = "Option::is_none")]
    pub emblem_background_path: Option<String>,
    /// The hash of the currently equipped emblem for the user. Can be used to look up the DestinyInventoryItemDefinition.
    #[serde(rename = "emblemHash", skip_serializing_if = "Option::is_none")]
    pub emblem_hash: Option<i32>,
    #[serde(rename = "emblemColor", skip_serializing_if = "Option::is_none")]
    pub emblem_color: Option<Box<crate::models::DestinyEntitiesCharactersDestinyCharacterComponentEmblemColor>>,
    #[serde(rename = "levelProgression", skip_serializing_if = "Option::is_none")]
    pub level_progression: Option<Box<crate::models::DestinyEntitiesCharactersDestinyCharacterComponentLevelProgression>>,
    /// The \"base\" level of your character, not accounting for any light level.
    #[serde(rename = "baseCharacterLevel", skip_serializing_if = "Option::is_none")]
    pub base_character_level: Option<i32>,
    /// A number between 0 and 100, indicating the whole and fractional % remaining to get to the next character level.
    #[serde(rename = "percentToNextLevel", skip_serializing_if = "Option::is_none")]
    pub percent_to_next_level: Option<f32>,
    /// If this Character has a title assigned to it, this is the identifier of the DestinyRecordDefinition that has that title information.
    #[serde(rename = "titleRecordHash", skip_serializing_if = "Option::is_none")]
    pub title_record_hash: Option<i32>,
}

impl DestinyPeriodEntitiesPeriodCharactersPeriodDestinyCharacterComponent {
    /// This component contains base properties of the character. You'll probably want to always request this component, but hey you do you.
    pub fn new() -> DestinyPeriodEntitiesPeriodCharactersPeriodDestinyCharacterComponent {
        DestinyPeriodEntitiesPeriodCharactersPeriodDestinyCharacterComponent {
            membership_id: None,
            membership_type: None,
            character_id: None,
            date_last_played: None,
            minutes_played_this_session: None,
            minutes_played_total: None,
            light: None,
            stats: None,
            race_hash: None,
            gender_hash: None,
            class_hash: None,
            race_type: None,
            class_type: None,
            gender_type: None,
            emblem_path: None,
            emblem_background_path: None,
            emblem_hash: None,
            emblem_color: None,
            level_progression: None,
            base_character_level: None,
            percent_to_next_level: None,
            title_record_hash: None,
        }
    }
}


