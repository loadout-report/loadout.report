use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};
use serde_with::{serde_as, DisplayFromStr};

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyCraftableComponent {

    /// If the requirements are not met for crafting this item, these will index into the list of failure strings.
    pub failed_requirement_indexes: Vec<i32>,
    /// Plug item state for the crafting sockets.
    pub sockets: Vec<crate::generated::models::destiny::components::craftables::DestinyCraftableSocketComponent>,
    /// No documentation provided.
    pub visible: bool,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyCraftableSocketComponent {

    /// No documentation provided.
    pub plug_set_hash: crate::id::Id<crate::generated::models::destiny::definitions::sockets::DestinyPlugSetDefinition>,
    /// Unlock state for plugs in the socket plug set definition
    pub plugs: Vec<crate::generated::models::destiny::components::craftables::DestinyCraftableSocketPlugComponent>,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyCraftableSocketPlugComponent {

    /// Index into the unlock requirements to display failure descriptions
    pub failed_requirement_indexes: Vec<i32>,
    /// No documentation provided.
    pub plug_item_hash: crate::id::Id<crate::generated::models::destiny::definitions::DestinyInventoryItemDefinition>,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyCraftablesComponent {

    /// A map of craftable item hashes to craftable item state components.
    pub craftables: HashMap<u32, crate::generated::models::destiny::components::craftables::DestinyCraftableComponent>,
    /// The hash for the root presentation node definition of craftable item categories.
    pub crafting_root_node_hash: crate::id::Id<crate::generated::models::destiny::definitions::presentation::DestinyPresentationNodeDefinition>,
}
