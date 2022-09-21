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
pub struct DestinyPeriodSocketsPeriodDestinyItemPlugBase {
    /// The hash identifier of the DestinyInventoryItemDefinition that represents this plug.
    #[serde(rename = "plugItemHash", skip_serializing_if = "Option::is_none")]
    pub plug_item_hash: Option<i32>,
    /// If true, this plug has met all of its insertion requirements. Big if true.
    #[serde(rename = "canInsert", skip_serializing_if = "Option::is_none")]
    pub can_insert: Option<bool>,
    /// If true, this plug will provide its benefits while inserted.
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// If the plug cannot be inserted for some reason, this will have the indexes into the plug item definition's plug.insertionRules property, so you can show the reasons why it can't be inserted.  This list will be empty if the plug can be inserted.
    #[serde(rename = "insertFailIndexes", skip_serializing_if = "Option::is_none")]
    pub insert_fail_indexes: Option<Vec<i32>>,
    /// If a plug is not enabled, this will be populated with indexes into the plug item definition's plug.enabledRules property, so that you can show the reasons why it is not enabled.  This list will be empty if the plug is enabled.
    #[serde(rename = "enableFailIndexes", skip_serializing_if = "Option::is_none")]
    pub enable_fail_indexes: Option<Vec<i32>>,
}

impl DestinyPeriodSocketsPeriodDestinyItemPlugBase {
    pub fn new() -> DestinyPeriodSocketsPeriodDestinyItemPlugBase {
        DestinyPeriodSocketsPeriodDestinyItemPlugBase {
            plug_item_hash: None,
            can_insert: None,
            enabled: None,
            insert_fail_indexes: None,
            enable_fail_indexes: None,
        }
    }
}


