

use serde::{Serialize, Deserialize};


/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EntityActionResult {

    /// No documentation provided.
    #[serde(with = "crate::unfuck_js::stringified_numbers")]
    pub entity_id: i64,
    /// No documentation provided.
    pub result: i32,
}
