use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};
use serde_with::{serde_as, DisplayFromStr};

/// Represents the public-facing status of an activity: any data about what is currently active in the Activity, regardless of an individual character's progress in it.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyPublicActivityStatus {

    /// Active Challenges for the activity, if any - represented as hashes for DestinyObjectiveDefinitions.
    pub challenge_objective_hashes: Vec<crate::id::Id<u32, crate::generated::models::destiny::definitions::DestinyObjectiveDefinition>>,
    /// The active modifiers on this activity, if any - represented as hashes for DestinyActivityModifierDefinitions.
    pub modifier_hashes: Vec<crate::id::Id<u32, crate::generated::models::destiny::definitions::activity_modifiers::DestinyActivityModifierDefinition>>,
    /// If the activity itself provides any specific "mock" rewards, this will be the items and their quantity.
/// Why "mock", you ask? Because these are the rewards as they are represented in the tooltip of the Activity.
/// These are often pointers to fake items that look good in a tooltip, but represent an abstract concept of what you will get for a reward rather than the specific items you may obtain.
    pub reward_tooltip_items: Vec<crate::generated::models::destiny::DestinyItemQuantity>,
}
