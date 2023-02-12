

use serde::{Serialize, Deserialize};


/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyTraitDefinition {

    /// An identifier for how this trait can be displayed. For example: a 'keyword' hint to show an explanation for certain related terms.
    pub display_hint: String,
    /// The unique identifier for this entity. Guaranteed to be unique for the type of entity, but not globally.
/// When entities refer to each other in Destiny content, it is this hash that they are referring to.
    pub hash: u32,
    /// No documentation provided.
    pub trait_category_hash: u32,
    /// No documentation provided.
    pub display_properties: i32,
    /// The index of the entity as it was found in the investment tables.
    pub index: i32,
    /// No documentation provided.
    pub trait_category_id: String,
    /// If this is true, then there is an entity with this identifier/type combination, but BNet is not yet allowed to show it. Sorry!
    pub redacted: bool,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyTraitCategoryDefinition {

    /// The unique identifier for this entity. Guaranteed to be unique for the type of entity, but not globally.
/// When entities refer to each other in Destiny content, it is this hash that they are referring to.
    pub hash: u32,
    /// No documentation provided.
    pub trait_ids: i32,
    /// No documentation provided.
    pub trait_category_id: String,
    /// If this is true, then there is an entity with this identifier/type combination, but BNet is not yet allowed to show it. Sorry!
    pub redacted: bool,
    /// The index of the entity as it was found in the investment tables.
    pub index: i32,
    /// No documentation provided.
    pub trait_hashes: i32,
}
