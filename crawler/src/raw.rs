use optfield::optfield;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[optfield(pub OptPostGameCarnageReport, attrs)]
#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PostGameCarnageReport {
    /// Date and time for the activity.
    pub period: chrono::NaiveDateTime,
    /// If this activity has "phases", this is the phase at which the activity was started.
    /// This value is only valid for activities before the Beyond Light expansion shipped.
    /// Subsequent activities will not have a valid value here.
    pub starting_phase_index: Option<i32>,
    /// if the activity was started from the beginning,
    /// if that information is available and the activity was played post Witch Queen release.
    pub activity_was_started_from_beginning: Option<bool>,
    /// Details about the activity.
    pub activity_details: HistoricalStatsActivity,
    /// Collection of players and their data for this activity.
    pub entries: Vec<PostGameCarnageReportEntry>,
    /// Collection of stats for the player in this activity.
    pub teams: Vec<PostGameCarnageReportTeamEntry>,
}

#[optfield(pub OptHistoricalStatsActivity, attrs)]
#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HistoricalStatsActivity {
    /// The unique hash identifier of the DestinyActivityDefinition that was played.
    /// If I had this to do over, it'd be named activityHash. Too late now.
    pub reference_id: u32,
    /// The unique hash identifier of the DestinyActivityDefinition that was played.
    pub director_activity_hash: u32,
    /// The unique identifier for this *specific* match that was played.
    /// This value can be used to get additional data about this activity
    /// such as who else was playing via the GetPostGameCarnageReport endpoint.
    pub instance_id: i64,
    /// Indicates the most specific game mode of the activity that we could find.
    pub mode: i32,
    /// The list of all Activity Modes to which this activity applies, including aggregates.
    /// This will let you see, for example,
    /// whether the activity was both Clash and part of the Trials of the Nine event.
    pub modes: Vec<i32>,
    /// Whether or not the match was a private match.
    pub is_private: bool,
    /// The Membership Type indicating the platform on which this match was played.
    pub membership_type: i32,
}

#[optfield(pub OptPostGameCarnageReportEntry, attrs)]
#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PostGameCarnageReportEntry {
    /// Standing of the player
    pub standing: i32,
    /// Score of the player if available
    pub score: HistoricalStatsValue,
    /// Identity details of the player
    pub player: Player,
    /// ID of the player's character used in the activity.
    pub character_id: i64,
    /// Collection of stats for the player in this activity.
    pub values: HashMap<String, HistoricalStatsValue>,
    /// Extended data extracted from the activity blob.
    pub extended: PostGameCarnageReportExtendedData,
}

#[optfield(pub OptHistoricalStatsValue, attrs)]
#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HistoricalStatsValue {
    /// Unique ID for this stat
    pub stat_id: String,
    /// Basic stat value.
    pub basic: HistoricalStatsValuePair,
    /// Per game average for the statistic, if applicable
    pub pga: HistoricalStatsValuePair,
    /// Weighted value of the stat if a weight greater than 1 has been assigned.
    pub weighted: HistoricalStatsValuePair,
    /// When a stat represents the best, most, longest, fastest or some other personal best,
    /// the actual activity ID where that personal best was established is available on this property.
    pub activity_id: Option<i64>,
}

#[optfield(pub OptHistoricalStatsValuePair, attrs)]
#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HistoricalStatsValuePair {
    /// Raw value of the statistic
    pub value: f32,
    /// Localized formatted version of the value.
    pub display_value: String,
}

#[optfield(pub OptPlayer, attrs)]
#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    /// Details about the player as they are known in game (platform display name, Destiny emblem)
    pub destiny_user_info: UserInfoCard,
    /// Class of the character if applicable and available.
    pub character_class: String,
    pub class_hash: u32,
    pub race_hash: u32,
    pub gender_hash: u32,
    /// Level of the character if available. Zero if it is not available.
    pub character_level: i32,
    /// Light Level of the character if available. Zero if it is not available.
    pub light_level: i32,
    /// Details about the player as they are known on BungieNet.
    /// This will be undefined if the player has marked their credential private,
    /// or does not have a BungieNet account.
    pub bungie_net_user_info: UserInfoCard,
    pub clan_name: Option<String>,
    pub clan_tag: Option<String>,
    pub emblem_hash: InventoryItemDefinition,
}

#[optfield(pub OptPostGameCarnageReportExtendedData, attrs)]
#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PostGameCarnageReportExtendedData {}

#[optfield(pub OptUserInfoCard, attrs)]
#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserInfoCard {}

#[optfield(pub OptInventoryItemDefinition, attrs)]
#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InventoryItemDefinition {}

#[optfield(pub OptPostGameCarnageReportTeamEntry, attrs)]
#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PostGameCarnageReportTeamEntry {}
