

use serde::{Serialize, Deserialize};


/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyCollectibleAcquisitionBlock {

    /// No documentation provided.
    pub acquire_timestamp_unlock_value_hash: Option<u32>,
    /// No documentation provided.
    pub acquire_material_requirement_hash: Option<u32>,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyCollectibleStateBlock {

    /// No documentation provided.
    pub requirements: i32,
    /// No documentation provided.
    pub obscured_override_item_hash: Option<u32>,
}

/// Defines a
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyCollectibleDefinition {

    /// The unique identifier for this entity. Guaranteed to be unique for the type of entity, but not globally.
/// When entities refer to each other in Destiny content, it is this hash that they are referring to.
    pub hash: u32,
    /// The index of the entity as it was found in the investment tables.
    pub index: i32,
    /// No documentation provided.
    pub presentation_info: i32,
    /// Indicates whether the state of this Collectible is determined on a per-character or on an account-wide basis.
    pub scope: i32,
    /// No documentation provided.
    pub display_properties: i32,
    /// No documentation provided.
    pub presentation_node_type: i32,
    /// No documentation provided.
    pub state_info: i32,
    /// No documentation provided.
    pub item_hash: u32,
    /// A quick reference to presentation nodes that have this node as a child. Presentation nodes can be parented under multiple parents.
    pub parent_node_hashes: i32,
    /// A human readable string for a hint about how to acquire the item.
    pub source_string: String,
    /// No documentation provided.
    pub trait_ids: i32,
    /// This is a hash identifier we are building on the BNet side in an attempt to let people group collectibles by similar sources.
/// I can't promise that it's going to be 100% accurate, but if the designers were consistent in assigning the same source strings to items with the same sources, it *ought to* be. No promises though.
/// This hash also doesn't relate to an actual definition, just to note: we've got nothing useful other than the source string for this data.
    pub source_hash: Option<u32>,
    /// No documentation provided.
    pub acquisition_info: i32,
    /// No documentation provided.
    pub trait_hashes: i32,
    /// If this is true, then there is an entity with this identifier/type combination, but BNet is not yet allowed to show it. Sorry!
    pub redacted: bool,
}
