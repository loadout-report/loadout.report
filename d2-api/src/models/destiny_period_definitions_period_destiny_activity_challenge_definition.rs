/*
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * The version of the OpenAPI document: 2.16.0
 * Contact: support@bungie.com
 * Generated by: https://openapi-generator.tech
 */

/// DestinyPeriodDefinitionsPeriodDestinyActivityChallengeDefinition : Represents a reference to a Challenge, which for now is just an Objective.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DestinyPeriodDefinitionsPeriodDestinyActivityChallengeDefinition {
    /// The hash for the Objective that matches this challenge. Use it to look up the DestinyObjectiveDefinition.
    #[serde(rename = "objectiveHash", skip_serializing_if = "Option::is_none")]
    pub objective_hash: Option<i32>,
    /// The rewards as they're represented in the UI. Note that they generally link to \"dummy\" items that give a summary of rewards rather than direct, real items themselves.  If the quantity is 0, don't show the quantity.
    #[serde(rename = "dummyRewards", skip_serializing_if = "Option::is_none")]
    pub dummy_rewards: Option<Vec<crate::models::DestinyPeriodDestinyItemQuantity>>,
}

impl DestinyPeriodDefinitionsPeriodDestinyActivityChallengeDefinition {
    /// Represents a reference to a Challenge, which for now is just an Objective.
    pub fn new() -> DestinyPeriodDefinitionsPeriodDestinyActivityChallengeDefinition {
        DestinyPeriodDefinitionsPeriodDestinyActivityChallengeDefinition {
            objective_hash: None,
            dummy_rewards: None,
        }
    }
}


