/*
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * The version of the OpenAPI document: 2.16.0
 * Contact: support@bungie.com
 * Generated by: https://openapi-generator.tech
 */

/// DestinyHistoricalStatsDestinyPostGameCarnageReportDataActivityDetails : Details about the activity.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DestinyHistoricalStatsDestinyPostGameCarnageReportDataActivityDetails {
    /// The unique hash identifier of the DestinyActivityDefinition that was played. If I had this to do over, it'd be named activityHash. Too late now.
    #[serde(rename = "referenceId", skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<i32>,
    /// The unique hash identifier of the DestinyActivityDefinition that was played.
    #[serde(rename = "directorActivityHash", skip_serializing_if = "Option::is_none")]
    pub director_activity_hash: Option<i32>,
    /// The unique identifier for this *specific* match that was played.  This value can be used to get additional data about this activity such as who else was playing via the GetPostGameCarnageReport endpoint.
    #[serde(rename = "instanceId", skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<i64>,
    /// Indicates the most specific game mode of the activity that we could find.
    #[serde(rename = "mode", skip_serializing_if = "Option::is_none")]
    pub mode: Option<i32>,
    /// The list of all Activity Modes to which this activity applies, including aggregates. This will let you see, for example, whether the activity was both Clash and part of the Trials of the Nine event.
    #[serde(rename = "modes", skip_serializing_if = "Option::is_none")]
    pub modes: Option<Vec<i32>>,
    /// Whether or not the match was a private match.
    #[serde(rename = "isPrivate", skip_serializing_if = "Option::is_none")]
    pub is_private: Option<bool>,
    /// The Membership Type indicating the platform on which this match was played.
    #[serde(rename = "membershipType", skip_serializing_if = "Option::is_none")]
    pub membership_type: Option<i32>,
}

impl DestinyHistoricalStatsDestinyPostGameCarnageReportDataActivityDetails {
    /// Details about the activity.
    pub fn new() -> DestinyHistoricalStatsDestinyPostGameCarnageReportDataActivityDetails {
        DestinyHistoricalStatsDestinyPostGameCarnageReportDataActivityDetails {
            reference_id: None,
            director_activity_hash: None,
            instance_id: None,
            mode: None,
            modes: None,
            is_private: None,
            membership_type: None,
        }
    }
}


