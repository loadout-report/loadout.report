/*
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * The version of the OpenAPI document: 2.16.0
 * Contact: support@bungie.com
 * Generated by: https://openapi-generator.tech
 */

/// DestinyDefinitionsDestinyInventoryItemDefinitionObjectives : If this item has Objectives (extra tasks that can be accomplished related to the item... most frequently when the item is a Quest Step and the Objectives need to be completed to move on to the next Quest Step), this block will be non-null and the objectives defined herein.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DestinyDefinitionsDestinyInventoryItemDefinitionObjectives {
    /// The hashes to Objectives (DestinyObjectiveDefinition) that are part of this Quest Step, in the order that they should be rendered.
    #[serde(rename = "objectiveHashes", skip_serializing_if = "Option::is_none")]
    pub objective_hashes: Option<Vec<i32>>,
    /// For every entry in objectiveHashes, there is a corresponding entry in this array at the same index. If the objective is meant to be associated with a specific DestinyActivityDefinition, there will be a valid hash at that index. Otherwise, it will be invalid (0).  Rendered somewhat obsolete by perObjectiveDisplayProperties, which currently has much the same information but may end up with more info in the future.
    #[serde(rename = "displayActivityHashes", skip_serializing_if = "Option::is_none")]
    pub display_activity_hashes: Option<Vec<i32>>,
    /// If True, all objectives must be completed for the step to be completed. If False, any one objective can be completed for the step to be completed.
    #[serde(rename = "requireFullObjectiveCompletion", skip_serializing_if = "Option::is_none")]
    pub require_full_objective_completion: Option<bool>,
    /// The hash for the DestinyInventoryItemDefinition representing the Quest to which this Quest Step belongs.
    #[serde(rename = "questlineItemHash", skip_serializing_if = "Option::is_none")]
    pub questline_item_hash: Option<i32>,
    /// The localized string for narrative text related to this quest step, if any.
    #[serde(rename = "narrative", skip_serializing_if = "Option::is_none")]
    pub narrative: Option<String>,
    /// The localized string describing an action to be performed associated with the objectives, if any.
    #[serde(rename = "objectiveVerbName", skip_serializing_if = "Option::is_none")]
    pub objective_verb_name: Option<String>,
    /// The identifier for the type of quest being performed, if any. Not associated with any fixed definition, yet.
    #[serde(rename = "questTypeIdentifier", skip_serializing_if = "Option::is_none")]
    pub quest_type_identifier: Option<String>,
    /// A hashed value for the questTypeIdentifier, because apparently I like to be redundant.
    #[serde(rename = "questTypeHash", skip_serializing_if = "Option::is_none")]
    pub quest_type_hash: Option<i32>,
    /// One entry per Objective on the item, it will have related display information.
    #[serde(rename = "perObjectiveDisplayProperties", skip_serializing_if = "Option::is_none")]
    pub per_objective_display_properties: Option<Vec<crate::models::DestinyPeriodDefinitionsPeriodDestinyObjectiveDisplayProperties>>,
    #[serde(rename = "displayAsStatTracker", skip_serializing_if = "Option::is_none")]
    pub display_as_stat_tracker: Option<bool>,
}

impl DestinyDefinitionsDestinyInventoryItemDefinitionObjectives {
    /// If this item has Objectives (extra tasks that can be accomplished related to the item... most frequently when the item is a Quest Step and the Objectives need to be completed to move on to the next Quest Step), this block will be non-null and the objectives defined herein.
    pub fn new() -> DestinyDefinitionsDestinyInventoryItemDefinitionObjectives {
        DestinyDefinitionsDestinyInventoryItemDefinitionObjectives {
            objective_hashes: None,
            display_activity_hashes: None,
            require_full_objective_completion: None,
            questline_item_hash: None,
            narrative: None,
            objective_verb_name: None,
            quest_type_identifier: None,
            quest_type_hash: None,
            per_objective_display_properties: None,
            display_as_stat_tracker: None,
        }
    }
}


