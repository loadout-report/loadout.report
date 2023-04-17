use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};
use serde_with::{serde_as, DisplayFromStr};

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyCollectibleComponent {

    /// No documentation provided.
    pub state: crate::generated::models::destiny::DestinyCollectibleState,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyCollectiblesComponent {

    /// No documentation provided.
    pub collectibles: HashMap<u32, crate::generated::models::destiny::components::collectibles::DestinyCollectibleComponent>,
    /// The hash for the root presentation node definition of Collection Badges.
    pub collection_badges_root_node_hash: crate::id::Id<u32, crate::generated::models::destiny::definitions::presentation::DestinyPresentationNodeDefinition>,
    /// The hash for the root presentation node definition of Collection categories.
    pub collection_categories_root_node_hash: crate::id::Id<u32, crate::generated::models::destiny::definitions::presentation::DestinyPresentationNodeDefinition>,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyProfileCollectiblesComponent {

    /// No documentation provided.
    pub collectibles: HashMap<u32, crate::generated::models::destiny::components::collectibles::DestinyCollectibleComponent>,
    /// The hash for the root presentation node definition of Collection Badges.
    pub collection_badges_root_node_hash: crate::id::Id<u32, crate::generated::models::destiny::definitions::presentation::DestinyPresentationNodeDefinition>,
    /// The hash for the root presentation node definition of Collection categories.
    pub collection_categories_root_node_hash: crate::id::Id<u32, crate::generated::models::destiny::definitions::presentation::DestinyPresentationNodeDefinition>,
    /// The list of collectibles determined by the game as having been "recently" acquired.
/// The game client itself actually controls this data, so I personally question whether anyone will get much use out of this: because we can't edit this value through the API. But in case anyone finds it useful, here it is.
    pub newness_flagged_collectible_hashes: Vec<crate::id::Id<u32, crate::generated::models::destiny::definitions::collectibles::DestinyCollectibleDefinition>>,
    /// The list of collectibles determined by the game as having been "recently" acquired.
    pub recent_collectible_hashes: Vec<crate::id::Id<u32, crate::generated::models::destiny::definitions::collectibles::DestinyCollectibleDefinition>>,
}
