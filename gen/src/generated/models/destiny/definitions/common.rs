use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};
use serde_with::{serde_as, DisplayFromStr};

/// Many Destiny*Definition contracts - the "first order" entities of Destiny that have their own tables in the Manifest Database - also have displayable information. This is the base class for that display information.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyDisplayPropertiesDefinition {

    /// No documentation provided.
    pub description: String,
    /// No documentation provided.
    pub has_icon: bool,
    /// If this item has a high-res icon (at least for now, many things won't), then the path to that icon will be here.
    pub high_res_icon: String,
    /// Note that "icon" is sometimes misleading, and should be interpreted in the context of the entity. For instance, in Destiny 1 the DestinyRecordBookDefinition's icon was a big picture of a book.
/// But usually, it will be a small square image that you can use as... well, an icon.
/// They are currently represented as 96px x 96px images.
    pub icon: String,
    /// No documentation provided.
    pub icon_sequences: i32,
    /// No documentation provided.
    pub name: String,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyIconSequenceDefinition {

    /// No documentation provided.
    pub frames: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyPositionDefinition {

    /// No documentation provided.
    pub x: i32,
    /// No documentation provided.
    pub y: i32,
    /// No documentation provided.
    pub z: i32,
}
