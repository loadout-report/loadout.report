

use serde::{Serialize, Deserialize};


/// Represents known info about a Destiny Artifact.
/// We cannot guarantee that artifact definitions will be immutable between seasons - in fact, we've been told that they will be replaced between seasons. But this definition is built both to minimize the amount of lookups for related data that have to occur, and is built in hope that, if this plan changes, we will be able to accommodate it more easily.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyArtifactDefinition {

    /// The unique identifier for this entity. Guaranteed to be unique for the type of entity, but not globally.
/// When entities refer to each other in Destiny content, it is this hash that they are referring to.
    pub hash: u32,
    /// Any basic display info we know about the Artifact. Currently sourced from a related inventory item, but the source of this data is subject to change.
    pub display_properties: i32,
    /// Any Geometry/3D info we know about the Artifact. Currently sourced from a related inventory item's gearset information, but the source of this data is subject to change.
    pub translation_block: i32,
    /// The index of the entity as it was found in the investment tables.
    pub index: i32,
    /// Any Tier/Rank data related to this artifact, listed in display order.  Currently sourced from a Vendor, but this source is subject to change.
    pub tiers: i32,
    /// If this is true, then there is an entity with this identifier/type combination, but BNet is not yet allowed to show it. Sorry!
    pub redacted: bool,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyArtifactTierItemDefinition {

    /// The identifier of the Plug Item unlocked by activating this item in the Artifact.
    pub item_hash: u32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyArtifactTierDefinition {

    /// The items that can be earned within this tier.
    pub items: i32,
    /// The human readable title of this tier, if any.
    pub display_title: String,
    /// A string representing the localized minimum requirement text for this Tier, if any.
    pub progress_requirement_message: String,
    /// An identifier, unique within the Artifact, for this specific tier.
    pub tier_hash: u32,
    /// The minimum number of "unlock points" that you must have used before you can unlock items from this tier.
    pub minimum_unlock_points_used_requirement: i32,
}
