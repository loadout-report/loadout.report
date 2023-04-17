use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};
use serde_with::{serde_as, DisplayFromStr};

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyRecordCompletionBlock {

    /// No documentation provided.
    pub score_value: i32,
    /// The number of objectives that must be completed before the objective is considered "complete"
    pub partial_completion_objective_count_threshold: i32,
    /// No documentation provided.
    pub should_fire_toast: bool,
    /// No documentation provided.
    pub toast_style: crate::generated::models::destiny::DestinyRecordToastStyle,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyRecordDefinition {

    /// No documentation provided.
    pub completion_info: crate::generated::models::destiny::definitions::records::DestinyRecordCompletionBlock,
    /// No documentation provided.
    pub display_properties: crate::generated::models::destiny::definitions::common::DestinyDisplayPropertiesDefinition,
    /// No documentation provided.
    pub expiration_info: crate::generated::models::destiny::definitions::records::DestinyRecordExpirationBlock,
    /// No documentation provided.
    pub for_title_gilding: bool,
    /// The unique identifier for this entity. Guaranteed to be unique for the type of entity, but not globally.
/// When entities refer to each other in Destiny content, it is this hash that they are referring to.
    pub hash: crate::id::Id<u32, Self>,
    /// The index of the entity as it was found in the investment tables.
    pub index: i32,
    /// Some records have multiple 'interval' objectives, and the record may be claimed at each completed interval
    pub interval_info: crate::generated::models::destiny::definitions::records::DestinyRecordIntervalBlock,
    /// No documentation provided.
    pub lore_hash: Option<crate::id::Id<u32, crate::generated::models::destiny::definitions::lore::DestinyLoreDefinition>>,
    /// No documentation provided.
    pub objective_hashes: Vec<crate::id::Id<u32, crate::generated::models::destiny::definitions::DestinyObjectiveDefinition>>,
    /// A quick reference to presentation nodes that have this node as a child. Presentation nodes can be parented under multiple parents.
    pub parent_node_hashes: Vec<crate::id::Id<u32, crate::generated::models::destiny::definitions::presentation::DestinyPresentationNodeDefinition>>,
    /// No documentation provided.
    pub presentation_info: crate::generated::models::destiny::definitions::presentation::DestinyPresentationChildBlock,
    /// No documentation provided.
    pub presentation_node_type: crate::generated::models::destiny::DestinyPresentationNodeType,
    /// No documentation provided.
    pub record_value_style: crate::generated::models::destiny::DestinyRecordValueStyle,
    /// If this is true, then there is an entity with this identifier/type combination, but BNet is not yet allowed to show it. Sorry!
    pub redacted: bool,
    /// No documentation provided.
    pub requirements: crate::generated::models::destiny::definitions::presentation::DestinyPresentationNodeRequirementsBlock,
    /// If there is any publicly available information about rewards earned for achieving this record, this is the list of those items.
///  However, note that some records intentionally have "hidden" rewards. These will not be returned in this list.
    pub reward_items: Vec<crate::generated::models::destiny::DestinyItemQuantity>,
    /// Indicates whether this Record's state is determined on a per-character or on an account-wide basis.
    pub scope: crate::generated::models::destiny::DestinyScope,
    /// A hint to show a large icon for a reward
    pub should_show_large_icons: bool,
    /// No documentation provided.
    pub state_info: crate::generated::models::destiny::definitions::records::SchemaRecordStateBlock,
    /// No documentation provided.
    pub title_info: crate::generated::models::destiny::definitions::records::DestinyRecordTitleBlock,
    /// No documentation provided.
    pub trait_hashes: Vec<crate::id::Id<u32, crate::generated::models::destiny::definitions::traits::DestinyTraitDefinition>>,
    /// No documentation provided.
    pub trait_ids: Vec<String>,
}

/// If this record has an expiration after which it cannot be earned, this is some information about that expiration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyRecordExpirationBlock {

    /// No documentation provided.
    pub description: String,
    /// No documentation provided.
    pub has_expiration: bool,
    /// No documentation provided.
    pub icon: String,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyRecordIntervalBlock {

    /// No documentation provided.
    pub interval_objectives: Vec<crate::generated::models::destiny::definitions::records::DestinyRecordIntervalObjective>,
    /// No documentation provided.
    pub interval_rewards: Vec<crate::generated::models::destiny::definitions::records::DestinyRecordIntervalRewards>,
    /// No documentation provided.
    pub original_objective_array_insertion_index: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyRecordIntervalObjective {

    /// No documentation provided.
    pub interval_objective_hash: crate::id::Id<u32, crate::generated::models::destiny::definitions::DestinyObjectiveDefinition>,
    /// No documentation provided.
    pub interval_score_value: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyRecordIntervalRewards {

    /// No documentation provided.
    pub interval_reward_items: Vec<crate::generated::models::destiny::DestinyItemQuantity>,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyRecordTitleBlock {

    /// No documentation provided.
    pub gilding_tracking_record_hash: Option<crate::id::Id<u32, crate::generated::models::destiny::definitions::records::DestinyRecordDefinition>>,
    /// No documentation provided.
    pub has_title: bool,
    /// No documentation provided.
    pub titles_by_gender: HashMap<crate::generated::models::destiny::DestinyGender, String>,
    /// For those who prefer to use the definitions.
    pub titles_by_gender_hash: HashMap<u32, String>,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SchemaRecordStateBlock {

    /// No documentation provided.
    pub featured_priority: i32,
    /// No documentation provided.
    pub obscured_string: String,
}
