

use serde::{Serialize, Deserialize};


/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyBreakerTypeDefinition {

    /// The index of the entity as it was found in the investment tables.
    pub index: i32,
    /// If this is true, then there is an entity with this identifier/type combination, but BNet is not yet allowed to show it. Sorry!
    pub redacted: bool,
    /// The unique identifier for this entity. Guaranteed to be unique for the type of entity, but not globally.
/// When entities refer to each other in Destiny content, it is this hash that they are referring to.
    pub hash: u32,
    /// No documentation provided.
    pub display_properties: i32,
    /// We have an enumeration for Breaker types for quick reference. This is the current definition's breaker type enum value.
    pub enum_value: i32,
}
