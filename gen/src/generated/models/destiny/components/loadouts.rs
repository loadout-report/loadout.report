use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};
use serde_with::{serde_as, DisplayFromStr};

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyLoadoutComponent {

    /// No documentation provided.
    pub name_hash: u32,
    /// No documentation provided.
    pub color_hash: u32,
    /// No documentation provided.
    pub icon_hash: u32,
    /// No documentation provided.
    pub items: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyLoadoutItemComponent {

    /// No documentation provided.
    #[serde(with = "crate::unfuck_js::stringified_numbers")]
    pub item_instance_id: i64,
    /// No documentation provided.
    pub plug_item_hashes: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyLoadoutsComponent {

    /// No documentation provided.
    pub loadouts: i32,
}
