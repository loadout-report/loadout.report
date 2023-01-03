use serde::{Deserialize, Serialize};

/// Many Destiny*Definition contracts - the "first order" entities of Destiny that have their own
/// tables in the Manifest Database - also have displayable information.
/// This is the base class for that display information.
#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct DisplayPropertiesDefinition {
    pub description: String,
    pub name: String,
    /// Note that "icon" is sometimes misleading,
    /// and should be interpreted in the context of the entity.
    /// For instance, in Destiny 1 the DestinyRecordBookDefinition's icon
    /// was a big picture of a book.
    ///
    /// But usually, it will be a small square image that you can use as... well, an icon.
    ///
    /// They are currently represented as 96px x 96px images.
    pub icon: String,
    pub icon_sequences: Vec<IconSequenceDefinition>,
    /// If this item has a high-res icon (at least for now, many things won't),
    /// then the path to that icon will be here.
    pub high_res_icon: Option<String>,
    pub has_icon: bool,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct IconSequenceDefinition {
    pub frames: Vec<String>,
}
