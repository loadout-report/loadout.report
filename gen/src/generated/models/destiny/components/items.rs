use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};
use serde_with::{serde_as, DisplayFromStr};

/// Plugs are non-instanced items that can provide Stat and Perk benefits when socketed into an instanced item. Items have Sockets, and Plugs are inserted into Sockets.
/// This component finds all items that are considered "Plugs" in your inventory, and return information about the plug aside from any specific Socket into which it could be inserted.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyItemPlugComponent {

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
    pub plug_item_hash: crate::id::Id<crate::generated::models::destiny::definitions::DestinyInventoryItemDefinition>,
    /// Sometimes, Plugs may have objectives: these are often used for flavor and display purposes, but they can be used for any arbitrary purpose (both fortunately and unfortunately). Recently (with Season 2) they were expanded in use to be used as the "gating" for whether the plug can be inserted at all. For instance, a Plug might be tracking the number of PVP kills you have made. It will use the parent item's data about that tracking status to determine what to show, and will generally show it using the DestinyObjectiveDefinition's progressDescription property. Refer to the plug's itemHash and objective property for more information if you would like to display even more data.
    pub plug_objectives: Vec<crate::generated::models::destiny::quests::DestinyObjectiveProgress>,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyItemPlugObjectivesComponent {

    /// This set of data is keyed by the Item Hash (DestinyInventoryItemDefinition) of the plug whose objectives are being returned, with the value being the list of those objectives.
///  What if two plugs with the same hash are returned for an item, you ask?
///  Good question! They share the same item-scoped state, and as such would have identical objective state as a result. How's that for convenient.
///  Sometimes, Plugs may have objectives: generally, these are used for flavor and display purposes. For instance, a Plug might be tracking the number of PVP kills you have made. It will use the parent item's data about that tracking status to determine what to show, and will generally show it using the DestinyObjectiveDefinition's progressDescription property. Refer to the plug's itemHash and objective property for more information if you would like to display even more data.
    pub objectives_per_plug: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyItemReusablePlugsComponent {

    /// If the item supports reusable plugs, this is the list of plugs that are allowed to be used for the socket, and any relevant information about whether they are "enabled", whether they are allowed to be inserted, and any other information such as objectives.
///  A Reusable Plug is a plug that you can always insert into this socket as long as its insertion rules are passed, regardless of whether or not you have the plug in your inventory. An example of it failing an insertion rule would be if it has an Objective that needs to be completed before it can be inserted, and that objective hasn't been completed yet.
///  In practice, a socket will *either* have reusable plugs *or* it will allow for plugs in your inventory to be inserted. See DestinyInventoryItemDefinition.socket for more info.
///  KEY = The INDEX into the item's list of sockets. VALUE = The set of plugs for that socket.
///  If a socket doesn't have any reusable plugs defined at the item scope, there will be no entry for that socket.
    pub plugs: i32,
}
