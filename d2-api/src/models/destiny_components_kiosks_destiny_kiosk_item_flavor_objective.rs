/*
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * The version of the OpenAPI document: 2.16.0
 * Contact: support@bungie.com
 * Generated by: https://openapi-generator.tech
 */

/// DestinyComponentsKiosksDestinyKioskItemFlavorObjective : I may regret naming it this way - but this represents when an item has an objective that doesn't serve a beneficial purpose, but rather is used for \"flavor\" or additional information. For instance, when Emblems track specific stats, those stats are represented as Objectives on the item.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DestinyComponentsKiosksDestinyKioskItemFlavorObjective {
    /// The unique identifier of the Objective being referred to. Use to look up the DestinyObjectiveDefinition in static data.
    #[serde(rename = "objectiveHash", skip_serializing_if = "Option::is_none")]
    pub objective_hash: Option<i32>,
    /// If the Objective has a Destination associated with it, this is the unique identifier of the Destination being referred to. Use to look up the DestinyDestinationDefinition in static data. This will give localized data about *where* in the universe the objective should be achieved.
    #[serde(rename = "destinationHash", skip_serializing_if = "Option::is_none")]
    pub destination_hash: Option<i32>,
    /// If the Objective has an Activity associated with it, this is the unique identifier of the Activity being referred to. Use to look up the DestinyActivityDefinition in static data. This will give localized data about *what* you should be playing for the objective to be achieved.
    #[serde(rename = "activityHash", skip_serializing_if = "Option::is_none")]
    pub activity_hash: Option<i32>,
    /// If progress has been made, and the progress can be measured numerically, this will be the value of that progress. You can compare it to the DestinyObjectiveDefinition.completionValue property for current vs. upper bounds, and use DestinyObjectiveDefinition.inProgressValueStyle or completedValueStyle to determine how this should be rendered. Note that progress, in Destiny 2, need not be a literal numeric progression. It could be one of a number of possible values, even a Timestamp. Always examine DestinyObjectiveDefinition.inProgressValueStyle or completedValueStyle before rendering progress.
    #[serde(rename = "progress", skip_serializing_if = "Option::is_none")]
    pub progress: Option<i32>,
    /// As of Forsaken, objectives' completion value is determined dynamically at runtime.  This value represents the threshold of progress you need to surpass in order for this objective to be considered \"complete\".  If you were using objective data, switch from using the DestinyObjectiveDefinition's \"completionValue\" to this value.
    #[serde(rename = "completionValue", skip_serializing_if = "Option::is_none")]
    pub completion_value: Option<i32>,
    /// Whether or not the Objective is completed.
    #[serde(rename = "complete", skip_serializing_if = "Option::is_none")]
    pub complete: Option<bool>,
    /// If this is true, the objective is visible in-game. Otherwise, it's not yet visible to the player. Up to you if you want to honor this property.
    #[serde(rename = "visible", skip_serializing_if = "Option::is_none")]
    pub visible: Option<bool>,
}

impl DestinyComponentsKiosksDestinyKioskItemFlavorObjective {
    /// I may regret naming it this way - but this represents when an item has an objective that doesn't serve a beneficial purpose, but rather is used for \"flavor\" or additional information. For instance, when Emblems track specific stats, those stats are represented as Objectives on the item.
    pub fn new() -> DestinyComponentsKiosksDestinyKioskItemFlavorObjective {
        DestinyComponentsKiosksDestinyKioskItemFlavorObjective {
            objective_hash: None,
            destination_hash: None,
            activity_hash: None,
            progress: None,
            completion_value: None,
            complete: None,
            visible: None,
        }
    }
}


