

use serde::{Serialize, Deserialize};


/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyAnimationReference {

    /// No documentation provided.
    pub path: String,
    /// No documentation provided.
    pub anim_name: String,
    /// No documentation provided.
    pub anim_identifier: String,
}
