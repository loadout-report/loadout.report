/*
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * The version of the OpenAPI document: 2.16.0
 * Contact: support@bungie.com
 * Generated by: https://openapi-generator.tech
 */

/// DestinyPeriodDefinitionsPeriodDestinyItemSackBlockDefinition : Some items are \"sacks\" - they can be \"opened\" to produce other items. This is information related to its sack status, mostly UI strings. Engrams are an example of items that are considered to be \"Sacks\".



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DestinyPeriodDefinitionsPeriodDestinyItemSackBlockDefinition {
    /// A description of what will happen when you open the sack. As far as I can tell, this is blank currently. Unknown whether it will eventually be populated with useful info.
    #[serde(rename = "detailAction", skip_serializing_if = "Option::is_none")]
    pub detail_action: Option<String>,
    /// The localized name of the action being performed when you open the sack.
    #[serde(rename = "openAction", skip_serializing_if = "Option::is_none")]
    pub open_action: Option<String>,
    #[serde(rename = "selectItemCount", skip_serializing_if = "Option::is_none")]
    pub select_item_count: Option<i32>,
    #[serde(rename = "vendorSackType", skip_serializing_if = "Option::is_none")]
    pub vendor_sack_type: Option<String>,
    #[serde(rename = "openOnAcquire", skip_serializing_if = "Option::is_none")]
    pub open_on_acquire: Option<bool>,
}

impl DestinyPeriodDefinitionsPeriodDestinyItemSackBlockDefinition {
    /// Some items are \"sacks\" - they can be \"opened\" to produce other items. This is information related to its sack status, mostly UI strings. Engrams are an example of items that are considered to be \"Sacks\".
    pub fn new() -> DestinyPeriodDefinitionsPeriodDestinyItemSackBlockDefinition {
        DestinyPeriodDefinitionsPeriodDestinyItemSackBlockDefinition {
            detail_action: None,
            open_action: None,
            select_item_count: None,
            vendor_sack_type: None,
            open_on_acquire: None,
        }
    }
}


