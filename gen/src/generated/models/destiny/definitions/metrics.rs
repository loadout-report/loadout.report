use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};
use serde_with::{serde_as, DisplayFromStr};

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyMetricDefinition {

    /// No documentation provided.
    pub display_properties: crate::generated::models::destiny::definitions::common::DestinyDisplayPropertiesDefinition,
    /// No documentation provided.
    pub lower_value_is_better: bool,
    /// If this is true, then there is an entity with this identifier/type combination, but BNet is not yet allowed to show it. Sorry!
    pub redacted: bool,
    /// No documentation provided.
    pub tracking_objective_hash: u32,
    /// No documentation provided.
    pub trait_ids: i32,
    /// The index of the entity as it was found in the investment tables.
    pub index: i32,
    /// A quick reference to presentation nodes that have this node as a child. Presentation nodes can be parented under multiple parents.
    pub parent_node_hashes: i32,
    /// The unique identifier for this entity. Guaranteed to be unique for the type of entity, but not globally.
/// When entities refer to each other in Destiny content, it is this hash that they are referring to.
    pub hash: u32,
    /// No documentation provided.
    pub trait_hashes: i32,
    /// No documentation provided.
    pub presentation_node_type: crate::generated::models::destiny::DestinyPresentationNodeType,
}
