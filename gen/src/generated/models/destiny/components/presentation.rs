use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};
use serde_with::{serde_as, DisplayFromStr};

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyPresentationNodeComponent {

    /// The value at which the presentation node is considered to be completed.
    pub completion_value: i32,
    /// How much of the presentation node is considered to be completed so far by the given character/profile.
    pub progress_value: i32,
    /// An optional property: presentation nodes MAY have objectives, which can be used to infer more human readable data about the progress. However, progressValue and completionValue ought to be considered the canonical values for progress on Progression Nodes.
    pub objective: crate::generated::models::destiny::quests::DestinyObjectiveProgress,
    /// No documentation provided.
    pub state: crate::generated::models::destiny::DestinyPresentationNodeState,
    /// If available, this is the current score for the record category that this node represents.
    pub record_category_score: Option<i32>,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyPresentationNodesComponent {

    /// No documentation provided.
    pub nodes: i32,
}
