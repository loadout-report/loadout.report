

use serde::{Serialize, Deserialize};


/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyPresentationNodeRecordChildEntry {

    /// No documentation provided.
    pub record_hash: u32,
    /// Use this value to sort the presentation node children in ascending order.
    pub node_display_priority: u32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyPresentationNodeCollectibleChildEntry {

    /// No documentation provided.
    pub collectible_hash: u32,
    /// Use this value to sort the presentation node children in ascending order.
    pub node_display_priority: u32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyPresentationNodeMetricChildEntry {

    /// Use this value to sort the presentation node children in ascending order.
    pub node_display_priority: u32,
    /// No documentation provided.
    pub metric_hash: u32,
}

/// A PresentationNode is an entity that represents a logical grouping of other entities visually/organizationally.
/// For now, Presentation Nodes may contain the following... but it may be used for more in the future:
/// - Collectibles - Records (Or, as the public will call them, "Triumphs." Don't ask me why we're overloading the term "Triumph", it still hurts me to think about it) - Metrics (aka Stat Trackers) - Other Presentation Nodes, allowing a tree of Presentation Nodes to be created
/// Part of me wants to break these into conceptual definitions per entity being collected, but the possibility of these different types being mixed in the same UI and the possibility that it could actually be more useful to return the "bare metal" presentation node concept has resulted in me deciding against that for the time being.
/// We'll see if I come to regret this as well.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyPresentationNodeDefinition {

    /// A quick reference to presentation nodes that have this node as a child. Presentation nodes can be parented under multiple parents.
    pub parent_node_hashes: i32,
    /// The original icon for this presentation node, before we futzed with it.
    pub original_icon: String,
    /// No documentation provided.
    pub max_category_record_score: i32,
    /// No documentation provided.
    pub presentation_node_type: i32,
    /// Some presentation nodes are meant to be explicitly shown on the "root" or "entry" screens for the feature to which they are related. You should use this icon when showing them on such a view, if you have a similar "entry point" view in your UI. If you don't have a UI, then I guess it doesn't matter either way does it?
    pub root_view_icon: String,
    /// The requirements for being able to interact with this presentation node and its children.
    pub requirements: i32,
    /// No documentation provided.
    pub node_type: i32,
    /// If this presentation node has an associated "Record" that you can accomplish for completing its children, this is the identifier of that Record.
    pub completion_record_hash: Option<u32>,
    /// The index of the entity as it was found in the investment tables.
    pub index: i32,
    /// If this presentation node shows a related objective (for instance, if it tracks the progress of its children), the objective being tracked is indicated here.
    pub objective_hash: Option<u32>,
    /// A hint for how to display this presentation node when it's shown in a list.
    pub display_style: i32,
    /// Indicates whether this presentation node's state is determined on a per-character or on an account-wide basis.
    pub scope: i32,
    /// No documentation provided.
    pub display_properties: i32,
    /// The unique identifier for this entity. Guaranteed to be unique for the type of entity, but not globally.
/// When entities refer to each other in Destiny content, it is this hash that they are referring to.
    pub hash: u32,
    /// If this is true, then there is an entity with this identifier/type combination, but BNet is not yet allowed to show it. Sorry!
    pub redacted: bool,
    /// No documentation provided.
    pub trait_ids: i32,
    /// A hint for how to display this presentation node when it's shown in its own detail screen.
    pub screen_style: i32,
    /// If this presentation node has children, but the game doesn't let you inspect the details of those children, that is indicated here.
    pub disable_child_subscreen_navigation: bool,
    /// The child entities contained by this presentation node.
    pub children: i32,
    /// No documentation provided.
    pub trait_hashes: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyScoredPresentationNodeBaseDefinition {

    /// The unique identifier for this entity. Guaranteed to be unique for the type of entity, but not globally.
/// When entities refer to each other in Destiny content, it is this hash that they are referring to.
    pub hash: u32,
    /// No documentation provided.
    pub presentation_node_type: i32,
    /// If this is true, then there is an entity with this identifier/type combination, but BNet is not yet allowed to show it. Sorry!
    pub redacted: bool,
    /// No documentation provided.
    pub trait_hashes: i32,
    /// No documentation provided.
    pub trait_ids: i32,
    /// The index of the entity as it was found in the investment tables.
    pub index: i32,
    /// No documentation provided.
    pub max_category_record_score: i32,
    /// A quick reference to presentation nodes that have this node as a child. Presentation nodes can be parented under multiple parents.
    pub parent_node_hashes: i32,
}

/// As/if presentation nodes begin to host more entities as children, these lists will be added to. One list property exists per type of entity that can be treated as a child of this presentation node, and each holds the identifier of the entity and any associated information needed to display the UI for that entity (if anything)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyPresentationNodeChildrenBlock {

    /// No documentation provided.
    pub metrics: i32,
    /// No documentation provided.
    pub presentation_nodes: i32,
    /// No documentation provided.
    pub collectibles: i32,
    /// No documentation provided.
    pub records: i32,
    /// No documentation provided.
    pub craftables: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyPresentationChildBlock {

    /// No documentation provided.
    pub display_style: i32,
    /// No documentation provided.
    pub presentation_node_type: i32,
    /// No documentation provided.
    pub parent_presentation_node_hashes: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyPresentationNodeChildEntry {

    /// No documentation provided.
    pub presentation_node_hash: u32,
    /// Use this value to sort the presentation node children in ascending order.
    pub node_display_priority: u32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyPresentationNodeChildEntryBase {

    /// Use this value to sort the presentation node children in ascending order.
    pub node_display_priority: u32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyPresentationNodeCraftableChildEntry {

    /// Use this value to sort the presentation node children in ascending order.
    pub node_display_priority: u32,
    /// No documentation provided.
    pub craftable_item_hash: u32,
}

/// This is the base class for all presentation system children. Presentation Nodes, Records, Collectibles, and Metrics.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyPresentationNodeBaseDefinition {

    /// A quick reference to presentation nodes that have this node as a child. Presentation nodes can be parented under multiple parents.
    pub parent_node_hashes: i32,
    /// The index of the entity as it was found in the investment tables.
    pub index: i32,
    /// If this is true, then there is an entity with this identifier/type combination, but BNet is not yet allowed to show it. Sorry!
    pub redacted: bool,
    /// No documentation provided.
    pub trait_hashes: i32,
    /// No documentation provided.
    pub presentation_node_type: i32,
    /// No documentation provided.
    pub trait_ids: i32,
    /// The unique identifier for this entity. Guaranteed to be unique for the type of entity, but not globally.
/// When entities refer to each other in Destiny content, it is this hash that they are referring to.
    pub hash: u32,
}

/// Presentation nodes can be restricted by various requirements. This defines the rules of those requirements, and the message(s) to be shown if these requirements aren't met.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyPresentationNodeRequirementsBlock {

    /// If this node is not accessible due to Entitlements (for instance, you don't own the required game expansion), this is the message to show.
    pub entitlement_unavailable_message: String,
}
