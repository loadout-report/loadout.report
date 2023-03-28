use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};
use serde_with::{serde_as, DisplayFromStr};

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyMetricComponent {

    /// No documentation provided.
    pub invisible: bool,
    /// No documentation provided.
    pub objective_progress: crate::generated::models::destiny::quests::DestinyObjectiveProgress,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyMetricsComponent {

    /// No documentation provided.
    pub metrics_root_node_hash: u32,
    /// No documentation provided.
    pub metrics: i32,
}
