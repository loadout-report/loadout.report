use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};
use serde_with::{serde_as, DisplayFromStr};

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinySocialCommendationsComponent {

    /// No documentation provided.
    pub score_detail_values: i32,
    /// No documentation provided.
    pub total_score: i32,
    /// No documentation provided.
    pub commendation_scores_by_hash: i32,
    /// No documentation provided.
    pub commendation_node_scores_by_hash: i32,
}
