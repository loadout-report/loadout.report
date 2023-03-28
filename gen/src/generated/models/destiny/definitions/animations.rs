use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};
use serde_with::{serde_as, DisplayFromStr};

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyAnimationReference {

    /// No documentation provided.
    pub path: String,
    /// No documentation provided.
    pub anim_identifier: String,
    /// No documentation provided.
    pub anim_name: String,
}
