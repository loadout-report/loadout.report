use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};
use serde_with::{serde_as, DisplayFromStr};

/// These are pre-constructed collections of data that can be used to determine the Level Requirement for an item given a Progression to be tested (such as the Character's level).
/// For instance, say a character receives a new Auto Rifle, and that Auto Rifle's DestinyInventoryItemDefinition.quality.progressionLevelRequirementHash property is pointing at one of these DestinyProgressionLevelRequirementDefinitions. Let's pretend also that the progressionHash it is pointing at is the Character Level progression. In that situation, the character's level will be used to interpolate a value in the requirementCurve property. The value picked up from that interpolation will be the required level for the item.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyProgressionLevelRequirementDefinition {

    /// The unique identifier for this entity. Guaranteed to be unique for the type of entity, but not globally.
/// When entities refer to each other in Destiny content, it is this hash that they are referring to.
    pub hash: crate::id::Id<u32, Self>,
    /// The index of the entity as it was found in the investment tables.
    pub index: i32,
    /// The progression whose level should be used to determine the level requirement.
/// Look up the DestinyProgressionDefinition with this hash for more information about the progression in question.
    pub progression_hash: crate::id::Id<u32, crate::generated::models::destiny::definitions::DestinyProgressionDefinition>,
    /// If this is true, then there is an entity with this identifier/type combination, but BNet is not yet allowed to show it. Sorry!
    pub redacted: bool,
    /// A curve of level requirements, weighted by the related progressions' level.
/// Interpolate against this curve with the character's progression level to determine what the level requirement of the generated item that is using this data will be.
    pub requirement_curve: Vec<crate::generated::models::interpolation::InterpolationPointFloat>,
}