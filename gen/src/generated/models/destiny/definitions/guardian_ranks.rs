use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};
use serde_with::{serde_as, DisplayFromStr};

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyGuardianRankConstantsDefinition {

    /// No documentation provided.
    pub rank_count: i32,
    /// The unique identifier for this entity. Guaranteed to be unique for the type of entity, but not globally.
/// When entities refer to each other in Destiny content, it is this hash that they are referring to.
    pub hash: u32,
    /// The index of the entity as it was found in the investment tables.
    pub index: i32,
    /// If this is true, then there is an entity with this identifier/type combination, but BNet is not yet allowed to show it. Sorry!
    pub redacted: bool,
    /// No documentation provided.
    pub display_properties: crate::generated::models::destiny::definitions::common::DestinyDisplayPropertiesDefinition,
    /// No documentation provided.
    pub root_node_hash: u32,
    /// No documentation provided.
    pub icon_backgrounds: crate::generated::models::destiny::definitions::guardian_ranks::DestinyGuardianRankIconBackgroundsDefinition,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyGuardianRankDefinition {

    /// No documentation provided.
    pub overlay_mask_image_path: String,
    /// If this is true, then there is an entity with this identifier/type combination, but BNet is not yet allowed to show it. Sorry!
    pub redacted: bool,
    /// No documentation provided.
    pub overlay_image_path: String,
    /// The unique identifier for this entity. Guaranteed to be unique for the type of entity, but not globally.
/// When entities refer to each other in Destiny content, it is this hash that they are referring to.
    pub hash: u32,
    /// No documentation provided.
    pub presentation_node_hash: u32,
    /// No documentation provided.
    pub rank_number: i32,
    /// The index of the entity as it was found in the investment tables.
    pub index: i32,
    /// No documentation provided.
    pub display_properties: crate::generated::models::destiny::definitions::common::DestinyDisplayPropertiesDefinition,
    /// No documentation provided.
    pub foreground_image_path: String,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyGuardianRankIconBackgroundsDefinition {

    /// No documentation provided.
    pub background_filled_gray_heavy_alpha_bordered_image_path: String,
    /// No documentation provided.
    pub background_plate_black_image_path: String,
    /// No documentation provided.
    pub background_empty_bordered_image_path: String,
    /// No documentation provided.
    pub background_filled_gray_medium_alpha_bordered_image_path: String,
    /// No documentation provided.
    pub background_plate_black_alpha_image_path: String,
    /// No documentation provided.
    pub background_filled_blue_bordered_image_path: String,
    /// No documentation provided.
    pub background_filled_blue_low_alpha_image_path: String,
    /// No documentation provided.
    pub background_filled_blue_gradient_bordered_image_path: String,
    /// No documentation provided.
    pub background_filled_blue_medium_alpha_image_path: String,
    /// No documentation provided.
    pub background_empty_blue_gradient_bordered_image_path: String,
    /// No documentation provided.
    pub background_plate_white_image_path: String,
    /// No documentation provided.
    pub background_filled_white_image_path: String,
    /// No documentation provided.
    pub background_filled_white_medium_alpha_image_path: String,
}
