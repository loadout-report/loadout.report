use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};
use serde_with::{serde_as, DisplayFromStr};
pub mod definitions;
/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyActivityHistoryResults {

    /// List of activities, the most recent activity first.
    pub activities: Vec<crate::generated::models::destiny::historical_stats::DestinyHistoricalStatsPeriodGroup>,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyAggregateActivityResults {

    /// List of all activities the player has participated in.
    pub activities: Vec<crate::generated::models::destiny::historical_stats::DestinyAggregateActivityStats>,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyAggregateActivityStats {

    /// Hash ID that can be looked up in the DestinyActivityTable.
    pub activity_hash: crate::id::Id<crate::generated::models::destiny::definitions::DestinyActivityDefinition>,
    /// Collection of stats for the player in this activity.
    pub values: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyClanAggregateStat {

    /// The id of the mode of stats (allPvp, allPvE, etc)
    pub mode: crate::generated::models::destiny::historical_stats::definitions::DestinyActivityModeType,
    /// The id of the stat
    pub stat_id: String,
    /// Value of the stat for this player
    pub value: crate::generated::models::destiny::historical_stats::DestinyHistoricalStatsValue,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyHistoricalStatsAccountResult {

    /// No documentation provided.
    pub characters: Vec<crate::generated::models::destiny::historical_stats::DestinyHistoricalStatsPerCharacter>,
    /// No documentation provided.
    pub merged_all_characters: crate::generated::models::destiny::historical_stats::DestinyHistoricalStatsWithMerged,
    /// No documentation provided.
    pub merged_deleted_characters: crate::generated::models::destiny::historical_stats::DestinyHistoricalStatsWithMerged,
}

/// Summary information about the activity that was played.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyHistoricalStatsActivity {

    /// The unique hash identifier of the DestinyActivityDefinition that was played.
    pub director_activity_hash: crate::id::Id<crate::generated::models::destiny::definitions::DestinyActivityDefinition>,
    /// The unique identifier for this *specific* match that was played.
/// This value can be used to get additional data about this activity such as who else was playing via the GetPostGameCarnageReport endpoint.
    #[serde(with = "crate::unfuck_js::stringified_numbers")]
    pub instance_id: i64,
    /// Whether or not the match was a private match.
    pub is_private: bool,
    /// The Membership Type indicating the platform on which this match was played.
    pub membership_type: crate::generated::models::BungieMembershipType,
    /// Indicates the most specific game mode of the activity that we could find.
    pub mode: crate::generated::models::destiny::historical_stats::definitions::DestinyActivityModeType,
    /// The list of all Activity Modes to which this activity applies, including aggregates. This will let you see, for example, whether the activity was both Clash and part of the Trials of the Nine event.
    pub modes: Vec<crate::generated::models::destiny::historical_stats::definitions::DestinyActivityModeType>,
    /// The unique hash identifier of the DestinyActivityDefinition that was played. If I had this to do over, it'd be named activityHash. Too late now.
    pub reference_id: crate::id::Id<crate::generated::models::destiny::definitions::DestinyActivityDefinition>,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyHistoricalStatsByPeriod {

    /// No documentation provided.
    pub all_time: i32,
    /// No documentation provided.
    pub all_time_tier_1: i32,
    /// No documentation provided.
    pub all_time_tier_2: i32,
    /// No documentation provided.
    pub all_time_tier_3: i32,
    /// No documentation provided.
    pub daily: Vec<crate::generated::models::destiny::historical_stats::DestinyHistoricalStatsPeriodGroup>,
    /// No documentation provided.
    pub monthly: Vec<crate::generated::models::destiny::historical_stats::DestinyHistoricalStatsPeriodGroup>,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyHistoricalStatsPerCharacter {

    /// No documentation provided.
    #[serde(with = "crate::unfuck_js::stringified_numbers")]
    pub character_id: i64,
    /// No documentation provided.
    pub deleted: bool,
    /// No documentation provided.
    pub merged: crate::generated::models::destiny::historical_stats::DestinyHistoricalStatsByPeriod,
    /// No documentation provided.
    pub results: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyHistoricalStatsPeriodGroup {

    /// If the period group is for a specific activity, this property will be set.
    pub activity_details: crate::generated::models::destiny::historical_stats::DestinyHistoricalStatsActivity,
    /// Period for the group. If the stat periodType is day, then this will have a specific day. If the type is monthly, then this value will be the first day of the applicable month. This value is not set when the periodType is 'all time'.
    pub period: chrono::DateTime<chrono::Utc>,
    /// Collection of stats for the period.
    pub values: i32,
}

pub type DestinyHistoricalStatsResults = HashMap<String, String>;

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyHistoricalStatsValue {

    /// When a stat represents the best, most, longest, fastest or some other personal best, the actual activity ID where that personal best was established is available on this property.
    #[serde(with = "crate::unfuck_js::nullable_stringified_numbers")]
    pub activity_id: Option<i64>,
    /// Basic stat value.
    pub basic: crate::generated::models::destiny::historical_stats::DestinyHistoricalStatsValuePair,
    /// Per game average for the statistic, if applicable
    pub pga: crate::generated::models::destiny::historical_stats::DestinyHistoricalStatsValuePair,
    /// Unique ID for this stat
    pub stat_id: String,
    /// Weighted value of the stat if a weight greater than 1 has been assigned.
    pub weighted: crate::generated::models::destiny::historical_stats::DestinyHistoricalStatsValuePair,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyHistoricalStatsValuePair {

    /// Localized formated version of the value.
    pub display_value: String,
    /// Raw value of the statistic
    pub value: f64,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyHistoricalStatsWithMerged {

    /// No documentation provided.
    pub merged: crate::generated::models::destiny::historical_stats::DestinyHistoricalStatsByPeriod,
    /// No documentation provided.
    pub results: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyHistoricalWeaponStats {

    /// The hash ID of the item definition that describes the weapon.
    pub reference_id: crate::id::Id<crate::generated::models::destiny::definitions::DestinyInventoryItemDefinition>,
    /// Collection of stats for the period.
    pub values: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyHistoricalWeaponStatsData {

    /// List of weapons and their perspective values.
    pub weapons: Vec<crate::generated::models::destiny::historical_stats::DestinyHistoricalWeaponStats>,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyLeaderboard {

    /// No documentation provided.
    pub entries: Vec<crate::generated::models::destiny::historical_stats::DestinyLeaderboardEntry>,
    /// No documentation provided.
    pub stat_id: String,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyLeaderboardEntry {

    /// ID of the player's best character for the reported stat.
    #[serde(with = "crate::unfuck_js::stringified_numbers")]
    pub character_id: i64,
    /// Identity details of the player
    pub player: crate::generated::models::destiny::historical_stats::DestinyPlayer,
    /// Where this player ranks on the leaderboard. A value of 1 is the top rank.
    pub rank: i32,
    /// Value of the stat for this player
    pub value: crate::generated::models::destiny::historical_stats::DestinyHistoricalStatsValue,
}

pub type DestinyLeaderboardResults = HashMap<String, String>;

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyPlayer {

    /// Details about the player as they are known on BungieNet. This will be undefined if the player has marked their credential private, or does not have a BungieNet account.
    pub bungie_net_user_info: crate::generated::models::user::UserInfoCard,
    /// Class of the character if applicable and available.
    pub character_class: String,
    /// Level of the character if available. Zero if it is not available.
    pub character_level: i32,
    /// Current clan name for the player. This value may be null or an empty string if the user does not have a clan.
    pub clan_name: String,
    /// Current clan tag for the player. This value may be null or an empty string if the user does not have a clan.
    pub clan_tag: String,
    /// No documentation provided.
    pub class_hash: crate::id::Id<crate::generated::models::destiny::definitions::DestinyClassDefinition>,
    /// Details about the player as they are known in game (platform display name, Destiny emblem)
    pub destiny_user_info: crate::generated::models::user::UserInfoCard,
    /// If we know the emblem's hash, this can be used to look up the player's emblem at the time of a match when receiving PGCR data, or otherwise their currently equipped emblem (if we are able to obtain it).
    pub emblem_hash: crate::id::Id<crate::generated::models::destiny::definitions::DestinyInventoryItemDefinition>,
    /// No documentation provided.
    pub gender_hash: crate::id::Id<crate::generated::models::destiny::definitions::DestinyGenderDefinition>,
    /// Light Level of the character if available. Zero if it is not available.
    pub light_level: i32,
    /// No documentation provided.
    pub race_hash: crate::id::Id<crate::generated::models::destiny::definitions::DestinyRaceDefinition>,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyPostGameCarnageReportData {

    /// Details about the activity.
    pub activity_details: crate::generated::models::destiny::historical_stats::DestinyHistoricalStatsActivity,
    /// True if the activity was started from the beginning, if that information is available and the activity was played post Witch Queen release.
    pub activity_was_started_from_beginning: Option<bool>,
    /// Collection of players and their data for this activity.
    pub entries: Vec<crate::generated::models::destiny::historical_stats::DestinyPostGameCarnageReportEntry>,
    /// Date and time for the activity.
    pub period: chrono::DateTime<chrono::Utc>,
    /// If this activity has "phases", this is the phase at which the activity was started. This value is only valid for activities before the Beyond Light expansion shipped. Subsequent activities will not have a valid value here.
    pub starting_phase_index: Option<i32>,
    /// Collection of stats for the player in this activity.
    pub teams: Vec<crate::generated::models::destiny::historical_stats::DestinyPostGameCarnageReportTeamEntry>,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyPostGameCarnageReportEntry {

    /// ID of the player's character used in the activity.
    #[serde(with = "crate::unfuck_js::stringified_numbers")]
    pub character_id: i64,
    /// Extended data extracted from the activity blob.
    pub extended: crate::generated::models::destiny::historical_stats::DestinyPostGameCarnageReportExtendedData,
    /// Identity details of the player
    pub player: crate::generated::models::destiny::historical_stats::DestinyPlayer,
    /// Score of the player if available
    pub score: crate::generated::models::destiny::historical_stats::DestinyHistoricalStatsValue,
    /// Standing of the player
    pub standing: i32,
    /// Collection of stats for the player in this activity.
    pub values: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyPostGameCarnageReportExtendedData {

    /// Collection of stats for the player in this activity.
    pub values: i32,
    /// List of weapons and their perspective values.
    pub weapons: Vec<crate::generated::models::destiny::historical_stats::DestinyHistoricalWeaponStats>,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyPostGameCarnageReportTeamEntry {

    /// Score earned by the team
    pub score: crate::generated::models::destiny::historical_stats::DestinyHistoricalStatsValue,
    /// Team's standing relative to other teams.
    pub standing: crate::generated::models::destiny::historical_stats::DestinyHistoricalStatsValue,
    /// Integer ID for the team.
    pub team_id: i32,
    /// Alpha or Bravo
    pub team_name: String,
}
