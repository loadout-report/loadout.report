/*
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * The version of the OpenAPI document: 2.16.0
 * Contact: support@bungie.com
 * Generated by: https://openapi-generator.tech
 */

/// DestinyPeriodDefinitionsPeriodDestinyItemPerkEntryDefinition : An intrinsic perk on an item, and the requirements for it to be activated.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DestinyPeriodDefinitionsPeriodDestinyItemPerkEntryDefinition {
    /// If this perk is not active, this is the string to show for why it's not providing its benefits.
    #[serde(rename = "requirementDisplayString", skip_serializing_if = "Option::is_none")]
    pub requirement_display_string: Option<String>,
    /// A hash identifier for the DestinySandboxPerkDefinition being provided on the item.
    #[serde(rename = "perkHash", skip_serializing_if = "Option::is_none")]
    pub perk_hash: Option<i32>,
    /// Indicates whether this perk should be shown, or if it should be shown disabled.
    #[serde(rename = "perkVisibility", skip_serializing_if = "Option::is_none")]
    pub perk_visibility: Option<i32>,
}

impl DestinyPeriodDefinitionsPeriodDestinyItemPerkEntryDefinition {
    /// An intrinsic perk on an item, and the requirements for it to be activated.
    pub fn new() -> DestinyPeriodDefinitionsPeriodDestinyItemPerkEntryDefinition {
        DestinyPeriodDefinitionsPeriodDestinyItemPerkEntryDefinition {
            requirement_display_string: None,
            perk_hash: None,
            perk_visibility: None,
        }
    }
}


