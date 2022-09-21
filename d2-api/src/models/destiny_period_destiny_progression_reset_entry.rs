/*
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * The version of the OpenAPI document: 2.16.0
 * Contact: support@bungie.com
 * Generated by: https://openapi-generator.tech
 */

/// DestinyPeriodDestinyProgressionResetEntry : Represents a season and the number of resets you had in that season.   We do not necessarily - even for progressions with resets - track it over all seasons. So be careful and check the season numbers being returned.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DestinyPeriodDestinyProgressionResetEntry {
    #[serde(rename = "season", skip_serializing_if = "Option::is_none")]
    pub season: Option<i32>,
    #[serde(rename = "resets", skip_serializing_if = "Option::is_none")]
    pub resets: Option<i32>,
}

impl DestinyPeriodDestinyProgressionResetEntry {
    /// Represents a season and the number of resets you had in that season.   We do not necessarily - even for progressions with resets - track it over all seasons. So be careful and check the season numbers being returned.
    pub fn new() -> DestinyPeriodDestinyProgressionResetEntry {
        DestinyPeriodDestinyProgressionResetEntry {
            season: None,
            resets: None,
        }
    }
}


