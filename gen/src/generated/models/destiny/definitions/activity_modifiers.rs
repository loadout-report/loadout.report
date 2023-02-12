

use serde::{Serialize, Deserialize};


/// Modifiers - in Destiny 1, these were referred to as "Skulls" - are changes that can be applied to an Activity.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyActivityModifierDefinition {

    /// If this is true, then there is an entity with this identifier/type combination, but BNet is not yet allowed to show it. Sorry!
    pub redacted: bool,
    /// The unique identifier for this entity. Guaranteed to be unique for the type of entity, but not globally.
/// When entities refer to each other in Destiny content, it is this hash that they are referring to.
    pub hash: u32,
    /// No documentation provided.
    pub display_properties: i32,
    /// No documentation provided.
    pub display_in_nav_mode: bool,
    /// The index of the entity as it was found in the investment tables.
    pub index: i32,
    /// No documentation provided.
    pub display_in_activity_selection: bool,
}
