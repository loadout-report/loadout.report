use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};
use serde_with::{serde_as, DisplayFromStr};

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyItemPlug {

    /// If true, this plug has met all of its insertion requirements. Big if true.
    pub can_insert: bool,
    /// If a plug is not enabled, this will be populated with indexes into the plug item definition's plug.enabledRules property, so that you can show the reasons why it is not enabled.
/// This list will be empty if the plug is enabled.
    pub enable_fail_indexes: Vec<i32>,
    /// If true, this plug will provide its benefits while inserted.
    pub enabled: bool,
    /// If the plug cannot be inserted for some reason, this will have the indexes into the plug item definition's plug.insertionRules property, so you can show the reasons why it can't be inserted.
/// This list will be empty if the plug can be inserted.
    pub insert_fail_indexes: Vec<i32>,
    /// The hash identifier of the DestinyInventoryItemDefinition that represents this plug.
    pub plug_item_hash: crate::id::Id<u32, crate::generated::models::destiny::definitions::DestinyInventoryItemDefinition>,
    /// Sometimes, Plugs may have objectives: these are often used for flavor and display purposes, but they can be used for any arbitrary purpose (both fortunately and unfortunately). Recently (with Season 2) they were expanded in use to be used as the "gating" for whether the plug can be inserted at all. For instance, a Plug might be tracking the number of PVP kills you have made. It will use the parent item's data about that tracking status to determine what to show, and will generally show it using the DestinyObjectiveDefinition's progressDescription property. Refer to the plug's itemHash and objective property for more information if you would like to display even more data.
    pub plug_objectives: Vec<crate::generated::models::destiny::quests::DestinyObjectiveProgress>,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyItemPlugBase {

    /// If true, this plug has met all of its insertion requirements. Big if true.
    pub can_insert: bool,
    /// If a plug is not enabled, this will be populated with indexes into the plug item definition's plug.enabledRules property, so that you can show the reasons why it is not enabled.
/// This list will be empty if the plug is enabled.
    pub enable_fail_indexes: Vec<i32>,
    /// If true, this plug will provide its benefits while inserted.
    pub enabled: bool,
    /// If the plug cannot be inserted for some reason, this will have the indexes into the plug item definition's plug.insertionRules property, so you can show the reasons why it can't be inserted.
/// This list will be empty if the plug can be inserted.
    pub insert_fail_indexes: Vec<i32>,
    /// The hash identifier of the DestinyInventoryItemDefinition that represents this plug.
    pub plug_item_hash: crate::id::Id<u32, crate::generated::models::destiny::definitions::DestinyInventoryItemDefinition>,
}
