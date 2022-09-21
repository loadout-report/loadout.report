/*
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * The version of the OpenAPI document: 2.16.0
 * Contact: support@bungie.com
 * Generated by: https://openapi-generator.tech
 */

/// DestinyEntitiesItemsDestinyItemInstanceComponentPrimaryStat : The item stat that we consider to be \"primary\" for the item. For instance, this would be \"Attack\" for Weapons or \"Defense\" for armor.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DestinyEntitiesItemsDestinyItemInstanceComponentPrimaryStat {
    /// The hash identifier for the Stat. Use it to look up the DestinyStatDefinition for static data about the stat.
    #[serde(rename = "statHash", skip_serializing_if = "Option::is_none")]
    pub stat_hash: Option<i32>,
    /// The current value of the Stat.
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<i32>,
}

impl DestinyEntitiesItemsDestinyItemInstanceComponentPrimaryStat {
    /// The item stat that we consider to be \"primary\" for the item. For instance, this would be \"Attack\" for Weapons or \"Defense\" for armor.
    pub fn new() -> DestinyEntitiesItemsDestinyItemInstanceComponentPrimaryStat {
        DestinyEntitiesItemsDestinyItemInstanceComponentPrimaryStat {
            stat_hash: None,
            value: None,
        }
    }
}


