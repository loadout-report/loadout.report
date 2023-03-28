use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};
use serde_with::{serde_as, DisplayFromStr};

/// Returns data about a character's status with a given Objective. Combine with DestinyObjectiveDefinition static data for display purposes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyObjectiveProgress {

    /// If the Objective has an Activity associated with it, this is the unique identifier of the Activity being referred to. Use to look up the DestinyActivityDefinition in static data. This will give localized data about *what* you should be playing for the objective to be achieved.
    pub activity_hash: Option<crate::id::Id<crate::generated::models::destiny::definitions::DestinyActivityDefinition>>,
    /// Whether or not the Objective is completed.
    pub complete: bool,
    /// As of Forsaken, objectives' completion value is determined dynamically at runtime.
/// This value represents the threshold of progress you need to surpass in order for this objective to be considered "complete".
/// If you were using objective data, switch from using the DestinyObjectiveDefinition's "completionValue" to this value.
    pub completion_value: i32,
    /// If the Objective has a Destination associated with it, this is the unique identifier of the Destination being referred to. Use to look up the DestinyDestinationDefinition in static data. This will give localized data about *where* in the universe the objective should be achieved.
    pub destination_hash: Option<crate::id::Id<crate::generated::models::destiny::definitions::DestinyDestinationDefinition>>,
    /// The unique identifier of the Objective being referred to. Use to look up the DestinyObjectiveDefinition in static data.
    pub objective_hash: crate::id::Id<crate::generated::models::destiny::definitions::DestinyObjectiveDefinition>,
    /// If progress has been made, and the progress can be measured numerically, this will be the value of that progress. You can compare it to the DestinyObjectiveDefinition.completionValue property for current vs. upper bounds, and use DestinyObjectiveDefinition.inProgressValueStyle or completedValueStyle to determine how this should be rendered. Note that progress, in Destiny 2, need not be a literal numeric progression. It could be one of a number of possible values, even a Timestamp. Always examine DestinyObjectiveDefinition.inProgressValueStyle or completedValueStyle before rendering progress.
    pub progress: Option<i32>,
    /// If this is true, the objective is visible in-game. Otherwise, it's not yet visible to the player. Up to you if you want to honor this property.
    pub visible: bool,
}

/// Data regarding the progress of a Quest for a specific character. Quests are composed of multiple steps, each with potentially multiple objectives: this QuestStatus will return Objective data for the *currently active* step in this quest.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyQuestStatus {

    /// Whether or not the whole quest has been completed, regardless of whether or not you have redeemed the rewards for the quest.
    pub completed: bool,
    /// The current Quest Step will be an instanced item in the player's inventory. If you care about that, this is the instance ID of that item.
    #[serde(with = "crate::unfuck_js::stringified_numbers")]
    pub item_instance_id: i64,
    /// The hash identifier for the Quest Item. (Note: Quests are defined as Items, and thus you would use this to look up the quest's DestinyInventoryItemDefinition). For information on all steps in the quest, you can then examine its DestinyInventoryItemDefinition.setData property for Quest Steps (which are *also* items). You can use the Item Definition to display human readable data about the overall quest.
    pub quest_hash: crate::id::Id<crate::generated::models::destiny::definitions::DestinyInventoryItemDefinition>,
    /// Whether or not you have redeemed rewards for this quest.
    pub redeemed: bool,
    /// Whether or not you have started this quest.
    pub started: bool,
    /// The hash identifier of the current Quest Step, which is also a DestinyInventoryItemDefinition. You can use this to get human readable data about the current step and what to do in that step.
    pub step_hash: crate::id::Id<crate::generated::models::destiny::definitions::DestinyInventoryItemDefinition>,
    /// A step can have multiple objectives. This will give you the progress for each objective in the current step, in the order in which they are rendered in-game.
    pub step_objectives: Vec<crate::generated::models::destiny::quests::DestinyObjectiveProgress>,
    /// Whether or not the quest is tracked
    pub tracked: bool,
    /// If the quest has a related Vendor that you should talk to in order to initiate the quest/earn rewards/continue the quest, this will be the hash identifier of that Vendor. Look it up its DestinyVendorDefinition.
    pub vendor_hash: Option<u32>,
}
