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
pub struct DestinyPeriodHistoricalStatsPeriodDestinyPostGameCarnageReportExtendedData {
    /// List of weapons and their perspective values.
    #[serde(rename = "weapons", skip_serializing_if = "Option::is_none")]
    pub weapons: Option<Vec<crate::models::DestinyPeriodHistoricalStatsPeriodDestinyHistoricalWeaponStats>>,
    /// Collection of stats for the player in this activity.
    #[serde(rename = "values", skip_serializing_if = "Option::is_none")]
    pub values: Option<::std::collections::HashMap<String, crate::models::DestinyPeriodHistoricalStatsPeriodDestinyHistoricalStatsValue>>,
}

impl DestinyPeriodHistoricalStatsPeriodDestinyPostGameCarnageReportExtendedData {
    pub fn new() -> DestinyPeriodHistoricalStatsPeriodDestinyPostGameCarnageReportExtendedData {
        DestinyPeriodHistoricalStatsPeriodDestinyPostGameCarnageReportExtendedData {
            weapons: None,
            values: None,
        }
    }
}


