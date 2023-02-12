

use serde::{Serialize, Deserialize};


/// These are definitions for in-game "Lore," meant to be narrative enhancements of the game experience.
/// DestinyInventoryItemDefinitions for interesting items point to these definitions, but nothing's stopping you from scraping all of these and doing something cool with them. If they end up having cool data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyLoreDefinition {

    /// If this is true, then there is an entity with this identifier/type combination, but BNet is not yet allowed to show it. Sorry!
    pub redacted: bool,
    /// The index of the entity as it was found in the investment tables.
    pub index: i32,
    /// The unique identifier for this entity. Guaranteed to be unique for the type of entity, but not globally.
/// When entities refer to each other in Destiny content, it is this hash that they are referring to.
    pub hash: u32,
    /// No documentation provided.
    pub subtitle: String,
    /// No documentation provided.
    pub display_properties: i32,
}
