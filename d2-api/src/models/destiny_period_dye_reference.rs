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
pub struct DestinyPeriodDyeReference {
    #[serde(rename = "channelHash", skip_serializing_if = "Option::is_none")]
    pub channel_hash: Option<i32>,
    #[serde(rename = "dyeHash", skip_serializing_if = "Option::is_none")]
    pub dye_hash: Option<i32>,
}

impl DestinyPeriodDyeReference {
    pub fn new() -> DestinyPeriodDyeReference {
        DestinyPeriodDyeReference {
            channel_hash: None,
            dye_hash: None,
        }
    }
}


