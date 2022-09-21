/*
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * The version of the OpenAPI document: 2.16.0
 * Contact: support@bungie.com
 * Generated by: https://openapi-generator.tech
 */

/// DestinyDefinitionsDestinyNodeStepDefinitionActivationRequirement : If the step has requirements for activation (they almost always do, if nothing else than for the Talent Grid's Progression to have reached a certain level), they will be defined here.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DestinyDefinitionsDestinyNodeStepDefinitionActivationRequirement {
    /// The Progression level on the Talent Grid required to activate this node.  See DestinyTalentGridDefinition.progressionHash for the related Progression, and read DestinyProgressionDefinition's documentation to learn more about Progressions.
    #[serde(rename = "gridLevel", skip_serializing_if = "Option::is_none")]
    pub grid_level: Option<i32>,
    /// The list of hash identifiers for material requirement sets: materials that are required for the node to be activated. See DestinyMaterialRequirementSetDefinition for more information about material requirements.  In this case, only a single DestinyMaterialRequirementSetDefinition will be chosen from this list, and we won't know which one will be chosen until an instance of the item is created.
    #[serde(rename = "materialRequirementHashes", skip_serializing_if = "Option::is_none")]
    pub material_requirement_hashes: Option<Vec<i32>>,
}

impl DestinyDefinitionsDestinyNodeStepDefinitionActivationRequirement {
    /// If the step has requirements for activation (they almost always do, if nothing else than for the Talent Grid's Progression to have reached a certain level), they will be defined here.
    pub fn new() -> DestinyDefinitionsDestinyNodeStepDefinitionActivationRequirement {
        DestinyDefinitionsDestinyNodeStepDefinitionActivationRequirement {
            grid_level: None,
            material_requirement_hashes: None,
        }
    }
}


