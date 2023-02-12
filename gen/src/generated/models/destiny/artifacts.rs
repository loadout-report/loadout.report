

use serde::{Serialize, Deserialize};


/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyArtifactCharacterScoped {

    /// No documentation provided.
    pub reset_count: i32,
    /// No documentation provided.
    pub tiers: i32,
    /// No documentation provided.
    pub artifact_hash: u32,
    /// No documentation provided.
    pub points_used: i32,
}

/// Represents a Seasonal Artifact and all data related to it for the requested Account.
/// It can be combined with Character-scoped data for a full picture of what a character has available/has chosen, or just these settings can be used for overview information.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyArtifactProfileScoped {

    /// No documentation provided.
    pub artifact_hash: u32,
    /// No documentation provided.
    pub power_bonus: i32,
    /// No documentation provided.
    pub points_acquired: i32,
    /// No documentation provided.
    pub point_progression: i32,
    /// No documentation provided.
    pub power_bonus_progression: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyArtifactTierItem {

    /// No documentation provided.
    pub item_hash: u32,
    /// No documentation provided.
    pub is_active: bool,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyArtifactTier {

    /// No documentation provided.
    pub points_to_unlock: i32,
    /// No documentation provided.
    pub is_unlocked: bool,
    /// No documentation provided.
    pub items: i32,
    /// No documentation provided.
    pub tier_hash: u32,
}
