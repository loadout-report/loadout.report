use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};
use serde_with::{serde_as, DisplayFromStr};

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyLoadoutColorDefinition {

    /// The index of the entity as it was found in the investment tables.
    pub index: i32,
    /// No documentation provided.
    pub color_image_path: String,
    /// The unique identifier for this entity. Guaranteed to be unique for the type of entity, but not globally.
/// When entities refer to each other in Destiny content, it is this hash that they are referring to.
    pub hash: u32,
    /// If this is true, then there is an entity with this identifier/type combination, but BNet is not yet allowed to show it. Sorry!
    pub redacted: bool,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyLoadoutConstantsDefinition {

    /// A list of the loadout color hashes in index order, for convenience.
    pub loadout_color_hashes: i32,
    /// The index of the entity as it was found in the investment tables.
    pub index: i32,
    /// If this is true, then there is an entity with this identifier/type combination, but BNet is not yet allowed to show it. Sorry!
    pub redacted: bool,
    /// The unique identifier for this entity. Guaranteed to be unique for the type of entity, but not globally.
/// When entities refer to each other in Destiny content, it is this hash that they are referring to.
    pub hash: u32,
    /// The maximum number of loadouts available to each character. The loadouts component API response can return fewer loadouts than this, as more loadouts are unlocked by reaching higher Guardian Ranks.
    pub loadout_count_per_character: i32,
    /// No documentation provided.
    pub display_properties: crate::generated::models::destiny::definitions::common::DestinyDisplayPropertiesDefinition,
    /// A list of the loadout icon hashes in index order, for convenience.
    pub loadout_icon_hashes: i32,
    /// This is a color-inverted version of the whiteIconImagePath.
    pub black_icon_image_path: String,
    /// A list of the socket category hashes to be filtered out of loadout item preview displays.
    pub loadout_preview_filter_out_socket_category_hashes: i32,
    /// A list of the loadout name hashes in index order, for convenience.
    pub loadout_name_hashes: i32,
    /// A list of the socket type hashes to be filtered out of loadout item preview displays.
    pub loadout_preview_filter_out_socket_type_hashes: i32,
    /// This is the same icon as the one in the display properties, offered here as well with a more descriptive name.
    pub white_icon_image_path: String,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyLoadoutIconDefinition {

    /// If this is true, then there is an entity with this identifier/type combination, but BNet is not yet allowed to show it. Sorry!
    pub redacted: bool,
    /// The unique identifier for this entity. Guaranteed to be unique for the type of entity, but not globally.
/// When entities refer to each other in Destiny content, it is this hash that they are referring to.
    pub hash: u32,
    /// No documentation provided.
    pub icon_image_path: String,
    /// The index of the entity as it was found in the investment tables.
    pub index: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyLoadoutNameDefinition {

    /// If this is true, then there is an entity with this identifier/type combination, but BNet is not yet allowed to show it. Sorry!
    pub redacted: bool,
    /// The unique identifier for this entity. Guaranteed to be unique for the type of entity, but not globally.
/// When entities refer to each other in Destiny content, it is this hash that they are referring to.
    pub hash: u32,
    /// No documentation provided.
    pub name: String,
    /// The index of the entity as it was found in the investment tables.
    pub index: i32,
}
