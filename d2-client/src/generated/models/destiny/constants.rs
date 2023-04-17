use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};
use serde_with::{serde_as, DisplayFromStr};

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyEnvironmentLocationMapping {

    /// A hint that the UI uses to figure out how this location is activated by the player.
    pub activation_source: String,
    /// If this is populated, this is the activity you have to be playing in order to see this location appear because of this mapping. (theoretically, a location can have multiple mappings, and some might require you to be in a specific activity when others don't)
    pub activity_hash: Option<crate::id::Id<u32, crate::generated::models::destiny::definitions::DestinyActivityDefinition>>,
    /// If this is populated, it is the item that you must possess for this location to be active because of this mapping. (theoretically, a location can have multiple mappings, and some might require an item while others don't)
    pub item_hash: Option<crate::id::Id<u32, crate::generated::models::destiny::definitions::DestinyInventoryItemDefinition>>,
    /// The location that is revealed on the director by this mapping.
    pub location_hash: crate::id::Id<u32, crate::generated::models::destiny::definitions::DestinyLocationDefinition>,
    /// If this is populated, this is an objective related to the location.
    pub objective_hash: Option<crate::id::Id<u32, crate::generated::models::destiny::definitions::DestinyObjectiveDefinition>>,
}
