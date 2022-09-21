/*
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * The version of the OpenAPI document: 2.16.0
 * Contact: support@bungie.com
 * Generated by: https://openapi-generator.tech
 */

/// DestinyPeriodDefinitionsPeriodDestinyItemCraftingBlockDefinition : If an item can have an action performed on it (like \"Dismantle\"), it will be defined here if you care.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DestinyPeriodDefinitionsPeriodDestinyItemCraftingBlockDefinition {
    /// A reference to the item definition that is created when crafting with this 'recipe' item.
    #[serde(rename = "outputItemHash", skip_serializing_if = "Option::is_none")]
    pub output_item_hash: Option<i32>,
    /// A list of socket type hashes that describes which sockets are required for crafting with this recipe.
    #[serde(rename = "requiredSocketTypeHashes", skip_serializing_if = "Option::is_none")]
    pub required_socket_type_hashes: Option<Vec<i32>>,
    #[serde(rename = "failedRequirementStrings", skip_serializing_if = "Option::is_none")]
    pub failed_requirement_strings: Option<Vec<String>>,
    /// A reference to the base material requirements for crafting with this recipe.
    #[serde(rename = "baseMaterialRequirements", skip_serializing_if = "Option::is_none")]
    pub base_material_requirements: Option<i32>,
    /// A list of 'bonus' socket plugs that may be available if certain requirements are met.
    #[serde(rename = "bonusPlugs", skip_serializing_if = "Option::is_none")]
    pub bonus_plugs: Option<Vec<crate::models::DestinyPeriodDefinitionsPeriodDestinyItemCraftingBlockBonusPlugDefinition>>,
}

impl DestinyPeriodDefinitionsPeriodDestinyItemCraftingBlockDefinition {
    /// If an item can have an action performed on it (like \"Dismantle\"), it will be defined here if you care.
    pub fn new() -> DestinyPeriodDefinitionsPeriodDestinyItemCraftingBlockDefinition {
        DestinyPeriodDefinitionsPeriodDestinyItemCraftingBlockDefinition {
            output_item_hash: None,
            required_socket_type_hashes: None,
            failed_requirement_strings: None,
            base_material_requirements: None,
            bonus_plugs: None,
        }
    }
}


