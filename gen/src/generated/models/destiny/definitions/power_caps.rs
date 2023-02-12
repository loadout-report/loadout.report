

use serde::{Serialize, Deserialize};


/// Defines a 'power cap' (limit) for gear items, based on the rarity tier and season of release.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyPowerCapDefinition {

    /// The raw value for a power cap.
    pub power_cap: i32,
    /// If this is true, then there is an entity with this identifier/type combination, but BNet is not yet allowed to show it. Sorry!
    pub redacted: bool,
    /// The index of the entity as it was found in the investment tables.
    pub index: i32,
    /// The unique identifier for this entity. Guaranteed to be unique for the type of entity, but not globally.
/// When entities refer to each other in Destiny content, it is this hash that they are referring to.
    pub hash: u32,
}
