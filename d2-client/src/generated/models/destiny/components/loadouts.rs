use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};
use serde_with::{serde_as, DisplayFromStr};

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyLoadoutComponent {

    /// No documentation provided.
    pub color_hash: crate::id::Id<u32, crate::generated::models::destiny::definitions::loadouts::DestinyLoadoutColorDefinition>,
    /// No documentation provided.
    pub icon_hash: crate::id::Id<u32, crate::generated::models::destiny::definitions::loadouts::DestinyLoadoutIconDefinition>,
    /// No documentation provided.
    pub items: Vec<crate::generated::models::destiny::components::loadouts::DestinyLoadoutItemComponent>,
    /// No documentation provided.
    pub name_hash: crate::id::Id<u32, crate::generated::models::destiny::definitions::loadouts::DestinyLoadoutNameDefinition>,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyLoadoutItemComponent {

    /// No documentation provided.
    #[serde(with = "crate::unfuck_js::stringified_numbers")]
    pub item_instance_id: i64,
    /// No documentation provided.
    pub plug_item_hashes: Vec<crate::id::Id<u32, crate::generated::models::destiny::definitions::DestinyInventoryItemDefinition>>,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyLoadoutsComponent {

    /// No documentation provided.
    pub loadouts: Vec<crate::generated::models::destiny::components::loadouts::DestinyLoadoutComponent>,
}
