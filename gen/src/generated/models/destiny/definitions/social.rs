use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};
use serde_with::{serde_as, DisplayFromStr};

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinySocialCommendationDefinition {

    /// The index of the entity as it was found in the investment tables.
    pub index: i32,
    /// If this is true, then there is an entity with this identifier/type combination, but BNet is not yet allowed to show it. Sorry!
    pub redacted: bool,
    /// No documentation provided.
    pub display_properties: crate::generated::models::destiny::definitions::common::DestinyDisplayPropertiesDefinition,
    /// No documentation provided.
    pub parent_commendation_node_hash: u32,
    /// The display properties for the the activities that this commendation is available in.
    pub display_activities: i32,
    /// No documentation provided.
    pub activity_giving_limit: i32,
    /// The unique identifier for this entity. Guaranteed to be unique for the type of entity, but not globally.
/// When entities refer to each other in Destiny content, it is this hash that they are referring to.
    pub hash: u32,
    /// No documentation provided.
    pub card_image_path: String,
    /// No documentation provided.
    pub color: crate::generated::models::destiny::misc::DestinyColor,
    /// No documentation provided.
    pub display_priority: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinySocialCommendationNodeDefinition {

    /// The unique identifier for this entity. Guaranteed to be unique for the type of entity, but not globally.
/// When entities refer to each other in Destiny content, it is this hash that they are referring to.
    pub hash: u32,
    /// A list of hashes that map to child commendations.
    pub child_commendation_hashes: i32,
    /// No documentation provided.
    pub parent_commendation_node_hash: u32,
    /// A list of hashes that map to child commendation nodes. Only the root commendations node is expected to have child nodes.
    pub child_commendation_node_hashes: i32,
    /// No documentation provided.
    pub display_properties: crate::generated::models::destiny::definitions::common::DestinyDisplayPropertiesDefinition,
    /// The index of the entity as it was found in the investment tables.
    pub index: i32,
    /// The color associated with this group of commendations.
    pub color: crate::generated::models::destiny::misc::DestinyColor,
    /// If this is true, then there is an entity with this identifier/type combination, but BNet is not yet allowed to show it. Sorry!
    pub redacted: bool,
}
