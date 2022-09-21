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
pub struct DestinyPeriodDefinitionsPeriodDestinyPlugItemCraftingRequirements {
    #[serde(rename = "unlockRequirements", skip_serializing_if = "Option::is_none")]
    pub unlock_requirements: Option<Vec<crate::models::DestinyPeriodDefinitionsPeriodDestinyPlugItemCraftingUnlockRequirement>>,
    /// If the plug has a known level requirement, it'll be available here.
    #[serde(rename = "requiredLevel", skip_serializing_if = "Option::is_none")]
    pub required_level: Option<i32>,
    #[serde(rename = "materialRequirementHashes", skip_serializing_if = "Option::is_none")]
    pub material_requirement_hashes: Option<Vec<i32>>,
}

impl DestinyPeriodDefinitionsPeriodDestinyPlugItemCraftingRequirements {
    pub fn new() -> DestinyPeriodDefinitionsPeriodDestinyPlugItemCraftingRequirements {
        DestinyPeriodDefinitionsPeriodDestinyPlugItemCraftingRequirements {
            unlock_requirements: None,
            required_level: None,
            material_requirement_hashes: None,
        }
    }
}


