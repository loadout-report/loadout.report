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
pub struct DestinyPeriodComponentsPeriodCraftablesPeriodDestinyCraftableSocketComponent {
    #[serde(rename = "plugSetHash", skip_serializing_if = "Option::is_none")]
    pub plug_set_hash: Option<i32>,
    /// Unlock state for plugs in the socket plug set definition
    #[serde(rename = "plugs", skip_serializing_if = "Option::is_none")]
    pub plugs: Option<Vec<crate::models::DestinyPeriodComponentsPeriodCraftablesPeriodDestinyCraftableSocketPlugComponent>>,
}

impl DestinyPeriodComponentsPeriodCraftablesPeriodDestinyCraftableSocketComponent {
    pub fn new() -> DestinyPeriodComponentsPeriodCraftablesPeriodDestinyCraftableSocketComponent {
        DestinyPeriodComponentsPeriodCraftablesPeriodDestinyCraftableSocketComponent {
            plug_set_hash: None,
            plugs: None,
        }
    }
}


