/*
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * The version of the OpenAPI document: 2.16.0
 * Contact: support@bungie.com
 * Generated by: https://openapi-generator.tech
 */

/// DestinyEntitiesItemsDestinyItemTalentGridComponentGridProgression : If the item has a progression, it will be detailed here. A progression means that the item can gain experience. Thresholds of experience are what determines whether and when a talent node can be activated.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DestinyEntitiesItemsDestinyItemTalentGridComponentGridProgression {
    /// The hash identifier of the Progression in question. Use it to look up the DestinyProgressionDefinition in static data.
    #[serde(rename = "progressionHash", skip_serializing_if = "Option::is_none")]
    pub progression_hash: Option<i32>,
    /// The amount of progress earned today for this progression.
    #[serde(rename = "dailyProgress", skip_serializing_if = "Option::is_none")]
    pub daily_progress: Option<i32>,
    /// If this progression has a daily limit, this is that limit.
    #[serde(rename = "dailyLimit", skip_serializing_if = "Option::is_none")]
    pub daily_limit: Option<i32>,
    /// The amount of progress earned toward this progression in the current week.
    #[serde(rename = "weeklyProgress", skip_serializing_if = "Option::is_none")]
    pub weekly_progress: Option<i32>,
    /// If this progression has a weekly limit, this is that limit.
    #[serde(rename = "weeklyLimit", skip_serializing_if = "Option::is_none")]
    pub weekly_limit: Option<i32>,
    /// This is the total amount of progress obtained overall for this progression (for instance, the total amount of Character Level experience earned)
    #[serde(rename = "currentProgress", skip_serializing_if = "Option::is_none")]
    pub current_progress: Option<i32>,
    /// This is the level of the progression (for instance, the Character Level).
    #[serde(rename = "level", skip_serializing_if = "Option::is_none")]
    pub level: Option<i32>,
    /// This is the maximum possible level you can achieve for this progression (for example, the maximum character level obtainable)
    #[serde(rename = "levelCap", skip_serializing_if = "Option::is_none")]
    pub level_cap: Option<i32>,
    /// Progressions define their levels in \"steps\". Since the last step may be repeatable, the user may be at a higher level than the actual Step achieved in the progression. Not necessarily useful, but potentially interesting for those cruising the API. Relate this to the \"steps\" property of the DestinyProgression to see which step the user is on, if you care about that. (Note that this is Content Version dependent since it refers to indexes.)
    #[serde(rename = "stepIndex", skip_serializing_if = "Option::is_none")]
    pub step_index: Option<i32>,
    /// The amount of progression (i.e. \"Experience\") needed to reach the next level of this Progression. Jeez, progression is such an overloaded word.
    #[serde(rename = "progressToNextLevel", skip_serializing_if = "Option::is_none")]
    pub progress_to_next_level: Option<i32>,
    /// The total amount of progression (i.e. \"Experience\") needed in order to reach the next level.
    #[serde(rename = "nextLevelAt", skip_serializing_if = "Option::is_none")]
    pub next_level_at: Option<i32>,
    /// The number of resets of this progression you've executed this season, if applicable to this progression.
    #[serde(rename = "currentResetCount", skip_serializing_if = "Option::is_none")]
    pub current_reset_count: Option<i32>,
    /// Information about historical resets of this progression, if there is any data for it.
    #[serde(rename = "seasonResets", skip_serializing_if = "Option::is_none")]
    pub season_resets: Option<Vec<crate::models::DestinyPeriodDestinyProgressionResetEntry>>,
    /// Information about historical rewards for this progression, if there is any data for it.
    #[serde(rename = "rewardItemStates", skip_serializing_if = "Option::is_none")]
    pub reward_item_states: Option<Vec<i32>>,
}

impl DestinyEntitiesItemsDestinyItemTalentGridComponentGridProgression {
    /// If the item has a progression, it will be detailed here. A progression means that the item can gain experience. Thresholds of experience are what determines whether and when a talent node can be activated.
    pub fn new() -> DestinyEntitiesItemsDestinyItemTalentGridComponentGridProgression {
        DestinyEntitiesItemsDestinyItemTalentGridComponentGridProgression {
            progression_hash: None,
            daily_progress: None,
            daily_limit: None,
            weekly_progress: None,
            weekly_limit: None,
            current_progress: None,
            level: None,
            level_cap: None,
            step_index: None,
            progress_to_next_level: None,
            next_level_at: None,
            current_reset_count: None,
            season_resets: None,
            reward_item_states: None,
        }
    }
}


