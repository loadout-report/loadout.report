/*
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * The version of the OpenAPI document: 2.16.0
 * Contact: support@bungie.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DestinyPeriodHistoricalStatsPeriodDestinyPostGameCarnageReportTeamEntry {
    /// Integer ID for the team.
    #[serde(rename = "teamId", skip_serializing_if = "Option::is_none")]
    pub team_id: Option<i32>,
    #[serde(rename = "standing", skip_serializing_if = "Option::is_none")]
    pub standing: Option<Box<crate::models::DestinyHistoricalStatsDestinyPostGameCarnageReportTeamEntryStanding>>,
    #[serde(rename = "score", skip_serializing_if = "Option::is_none")]
    pub score: Option<Box<crate::models::DestinyHistoricalStatsDestinyPostGameCarnageReportTeamEntryScore>>,
    /// Alpha or Bravo
    #[serde(rename = "teamName", skip_serializing_if = "Option::is_none")]
    pub team_name: Option<String>,
}

impl DestinyPeriodHistoricalStatsPeriodDestinyPostGameCarnageReportTeamEntry {
    pub fn new() -> DestinyPeriodHistoricalStatsPeriodDestinyPostGameCarnageReportTeamEntry {
        DestinyPeriodHistoricalStatsPeriodDestinyPostGameCarnageReportTeamEntry {
            team_id: None,
            standing: None,
            score: None,
            team_name: None,
        }
    }
}


