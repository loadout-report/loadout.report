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

    /// The hash for the root presentation node definition of Collection categories.
    pub collection_categories_root_node_hash: u32,
    /// No documentation provided.
    pub collectibles: i32,
    /// The hash for the root presentation node definition of Collection Badges.
    pub collection_badges_root_node_hash: u32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyProfileCollectiblesComponent {

    /// The list of collectibles determined by the game as having been "recently" acquired.
/// The game client itself actually controls this data, so I personally question whether anyone will get much use out of this: because we can't edit this value through the API. But in case anyone finds it useful, here it is.
    pub newness_flagged_collectible_hashes: i32,
    /// The hash for the root presentation node definition of Collection categories.
    pub collection_categories_root_node_hash: u32,
    /// The hash for the root presentation node definition of Collection Badges.
    pub collection_badges_root_node_hash: u32,
    /// No documentation provided.
    pub collectibles: i32,
    /// The list of collectibles determined by the game as having been "recently" acquired.
    pub recent_collectible_hashes: i32,
}
