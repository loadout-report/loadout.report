

use serde::{Serialize, Deserialize};


/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyEnvironmentLocationMapping {

    /// If this is populated, this is an objective related to the location.
    pub objective_hash: Option<u32>,
    /// If this is populated, this is the activity you have to be playing in order to see this location appear because of this mapping. (theoretically, a location can have multiple mappings, and some might require you to be in a specific activity when others don't)
    pub activity_hash: Option<u32>,
    /// If this is populated, it is the item that you must possess for this location to be active because of this mapping. (theoretically, a location can have multiple mappings, and some might require an item while others don't)
    pub item_hash: Option<u32>,
    /// A hint that the UI uses to figure out how this location is activated by the player.
    pub activation_source: String,
    /// The location that is revealed on the director by this mapping.
    pub location_hash: u32,
}
