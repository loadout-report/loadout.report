use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};
use serde_with::{serde_as, DisplayFromStr};

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyCharacterRecordsComponent {

    /// No documentation provided.
    pub featured_record_hashes: Vec<crate::id::Id<u32, crate::generated::models::destiny::definitions::records::DestinyRecordDefinition>>,
    /// The hash for the root presentation node definition of Triumph categories.
    pub record_categories_root_node_hash: crate::id::Id<u32, crate::generated::models::destiny::definitions::presentation::DestinyPresentationNodeDefinition>,
    /// The hash for the root presentation node definition of Triumph Seals.
    pub record_seals_root_node_hash: crate::id::Id<u32, crate::generated::models::destiny::definitions::presentation::DestinyPresentationNodeDefinition>,
    /// No documentation provided.
    pub records: HashMap<u32, crate::generated::models::destiny::components::records::DestinyRecordComponent>,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyProfileRecordsComponent {

    /// Your 'active' Triumphs score.
    pub active_score: i32,
    /// Your 'legacy' Triumphs score.
    pub legacy_score: i32,
    /// Your 'lifetime' Triumphs score.
    pub lifetime_score: i32,
    /// The hash for the root presentation node definition of Triumph categories.
    pub record_categories_root_node_hash: crate::id::Id<u32, crate::generated::models::destiny::definitions::presentation::DestinyPresentationNodeDefinition>,
    /// The hash for the root presentation node definition of Triumph Seals.
    pub record_seals_root_node_hash: crate::id::Id<u32, crate::generated::models::destiny::definitions::presentation::DestinyPresentationNodeDefinition>,
    /// No documentation provided.
    pub records: HashMap<u32, crate::generated::models::destiny::components::records::DestinyRecordComponent>,
    /// Your 'active' Triumphs score, maintained for backwards compatibility.
    pub score: i32,
    /// If this profile is tracking a record, this is the hash identifier of the record it is tracking.
    pub tracked_record_hash: Option<crate::id::Id<u32, crate::generated::models::destiny::definitions::records::DestinyRecordDefinition>>,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyRecordComponent {

    /// If available, this is the number of times this record has been completed. For example, the number of times a seal title has been gilded.
    pub completed_count: Option<i32>,
    /// No documentation provided.
    pub interval_objectives: Vec<crate::generated::models::destiny::quests::DestinyObjectiveProgress>,
    /// No documentation provided.
    pub intervals_redeemed_count: i32,
    /// No documentation provided.
    pub objectives: Vec<crate::generated::models::destiny::quests::DestinyObjectiveProgress>,
    /// If available, a list that describes which reward rewards should be shown (true) or hidden (false). This property is for regular record rewards, and not for interval objective rewards.
    pub reward_visibilty: Vec<bool>,
    /// No documentation provided.
    pub state: crate::generated::models::destiny::DestinyRecordState,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyRecordsComponent {

    /// The hash for the root presentation node definition of Triumph categories.
    pub record_categories_root_node_hash: crate::id::Id<u32, crate::generated::models::destiny::definitions::presentation::DestinyPresentationNodeDefinition>,
    /// The hash for the root presentation node definition of Triumph Seals.
    pub record_seals_root_node_hash: crate::id::Id<u32, crate::generated::models::destiny::definitions::presentation::DestinyPresentationNodeDefinition>,
    /// No documentation provided.
    pub records: HashMap<u32, crate::generated::models::destiny::components::records::DestinyRecordComponent>,
}
