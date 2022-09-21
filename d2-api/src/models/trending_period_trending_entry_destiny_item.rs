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
pub struct TrendingPeriodTrendingEntryDestinyItem {
    #[serde(rename = "itemHash", skip_serializing_if = "Option::is_none")]
    pub item_hash: Option<i32>,
}

impl TrendingPeriodTrendingEntryDestinyItem {
    pub fn new() -> TrendingPeriodTrendingEntryDestinyItem {
        TrendingPeriodTrendingEntryDestinyItem {
            item_hash: None,
        }
    }
}


