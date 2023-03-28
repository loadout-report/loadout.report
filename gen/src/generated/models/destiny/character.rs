use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};
use serde_with::{serde_as, DisplayFromStr};

/// Raw data about the customization options chosen for a character's face and appearance.
/// You can look up the relevant class/race/gender combo in DestinyCharacterCustomizationOptionDefinition for the character, and then look up these values within the CustomizationOptions found to pull some data about their choices. Warning: not all of that data is meaningful. Some data has useful icons. Others have nothing, and are only meant for 3D rendering purposes (which we sadly do not expose yet)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyCharacterCustomization {

    /// No documentation provided.
    pub decal_color: u32,
    /// No documentation provided.
    pub wear_helmet: bool,
    /// No documentation provided.
    pub feature_index: i32,
    /// No documentation provided.
    pub personality: u32,
    /// No documentation provided.
    pub feature_colors: i32,
    /// No documentation provided.
    pub face: u32,
    /// No documentation provided.
    pub hair_index: i32,
    /// No documentation provided.
    pub decal_index: i32,
    /// No documentation provided.
    pub skin_color: u32,
    /// No documentation provided.
    pub eye_color: u32,
    /// No documentation provided.
    pub lip_color: u32,
    /// No documentation provided.
    pub hair_colors: i32,
}

/// A minimal view of a character's equipped items, for the purpose of rendering a summary screen or showing the character in 3D.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyCharacterPeerView {

    /// No documentation provided.
    pub equipment: i32,
}

/// Bare minimum summary information for an item, for the sake of 3D rendering the item.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyItemPeerView {

    /// The list of dyes that have been applied to this item.
    pub dyes: i32,
    /// The hash identifier of the item in question. Use it to look up the DestinyInventoryItemDefinition of the item for static rendering data.
    pub item_hash: u32,
}
