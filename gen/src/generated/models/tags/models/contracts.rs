
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};
use serde_with::{serde_as, DisplayFromStr};
/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TagResponse {

    /// No documentation provided.
    pub tag_text: String,
    /// No documentation provided.
    pub ignore_status: i32,
}
