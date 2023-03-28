use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};
use serde_with::{serde_as, DisplayFromStr};

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyActionRequest {

    /// No documentation provided.
    pub membership_type: crate::generated::models::BungieMembershipType,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyCharacterActionRequest {

    /// No documentation provided.
    #[serde(with = "crate::unfuck_js::stringified_numbers")]
    pub character_id: i64,
    /// No documentation provided.
    pub membership_type: crate::generated::models::BungieMembershipType,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyInsertPlugsActionRequest {

    /// No documentation provided.
    pub membership_type: crate::generated::models::BungieMembershipType,
    /// No documentation provided.
    #[serde(with = "crate::unfuck_js::stringified_numbers")]
    pub character_id: i64,
    /// Action token provided by the AwaGetActionToken API call.
    pub action_token: String,
    /// The instance ID of the item having a plug inserted. Only instanced items can have sockets.
    #[serde(with = "crate::unfuck_js::stringified_numbers")]
    pub item_instance_id: i64,
    /// The plugs being inserted.
    pub plug: crate::generated::models::destiny::requests::actions::DestinyInsertPlugsRequestEntry,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyInsertPlugsFreeActionRequest {

    /// No documentation provided.
    #[serde(with = "crate::unfuck_js::stringified_numbers")]
    pub character_id: i64,
    /// The plugs being inserted.
    pub plug: crate::generated::models::destiny::requests::actions::DestinyInsertPlugsRequestEntry,
    /// The instance ID of the item for this action request.
    #[serde(with = "crate::unfuck_js::stringified_numbers")]
    pub item_id: i64,
    /// No documentation provided.
    pub membership_type: crate::generated::models::BungieMembershipType,
}

/// Represents all of the data related to a single plug to be inserted.
/// Note that, while you *can* point to a socket that represents infusion, you will receive an error if you attempt to do so. Come on guys, let's play nice.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyInsertPlugsRequestEntry {

    /// The index into the socket array, which identifies the specific socket being operated on. We also need to know the socketArrayType in order to uniquely identify the socket.
/// Don't point to or try to insert a plug into an infusion socket. It won't work.
    pub socket_index: i32,
    /// Plugs are never instanced (except in infusion). So with the hash alone, we should be able to: 1) Infer whether the player actually needs to have the item, or if it's a reusable plug 2) Perform any operation needed to use the Plug, including removing the plug item and running reward sheets.
    pub plug_item_hash: u32,
    /// This property, combined with the socketIndex, tells us which socket we are referring to (since operations can be performed on both Intrinsic and "default" sockets, and they occupy different arrays in the Inventory Item Definition). I know, I know. Don't give me that look.
    pub socket_array_type: crate::generated::models::destiny::requests::actions::DestinySocketArrayType,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyItemActionRequest {

    /// The instance ID of the item for this action request.
    #[serde(with = "crate::unfuck_js::stringified_numbers")]
    pub item_id: i64,
    /// No documentation provided.
    #[serde(with = "crate::unfuck_js::stringified_numbers")]
    pub character_id: i64,
    /// No documentation provided.
    pub membership_type: crate::generated::models::BungieMembershipType,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyItemSetActionRequest {

    /// No documentation provided.
    pub item_ids: i32,
    /// No documentation provided.
    #[serde(with = "crate::unfuck_js::stringified_numbers")]
    pub character_id: i64,
    /// No documentation provided.
    pub membership_type: crate::generated::models::BungieMembershipType,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyItemStateRequest {

    /// The instance ID of the item for this action request.
    #[serde(with = "crate::unfuck_js::stringified_numbers")]
    pub item_id: i64,
    /// No documentation provided.
    #[serde(with = "crate::unfuck_js::stringified_numbers")]
    pub character_id: i64,
    /// No documentation provided.
    pub state: bool,
    /// No documentation provided.
    pub membership_type: crate::generated::models::BungieMembershipType,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyLoadoutActionRequest {

    /// No documentation provided.
    #[serde(with = "crate::unfuck_js::stringified_numbers")]
    pub character_id: i64,
    /// The index of the loadout for this action request.
    pub loadout_index: i32,
    /// No documentation provided.
    pub membership_type: crate::generated::models::BungieMembershipType,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyLoadoutUpdateActionRequest {

    /// The index of the loadout for this action request.
    pub loadout_index: i32,
    /// No documentation provided.
    pub name_hash: Option<u32>,
    /// No documentation provided.
    pub membership_type: crate::generated::models::BungieMembershipType,
    /// No documentation provided.
    pub color_hash: Option<u32>,
    /// No documentation provided.
    pub icon_hash: Option<u32>,
    /// No documentation provided.
    #[serde(with = "crate::unfuck_js::stringified_numbers")]
    pub character_id: i64,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyPostmasterTransferRequest {

    /// The instance ID of the item for this action request.
    #[serde(with = "crate::unfuck_js::stringified_numbers")]
    pub item_id: i64,
    /// No documentation provided.
    pub membership_type: crate::generated::models::BungieMembershipType,
    /// No documentation provided.
    #[serde(with = "crate::unfuck_js::stringified_numbers")]
    pub character_id: i64,
    /// No documentation provided.
    pub stack_size: i32,
    /// No documentation provided.
    pub item_reference_hash: u32,
}

/// If you look in the DestinyInventoryItemDefinition's "sockets" property, you'll see that there are two types of sockets: intrinsic, and "socketEntry."
/// Unfortunately, because Intrinsic sockets are a whole separate array, it is no longer sufficient to know the index into that array to know which socket we're talking about. You have to know whether it's in the default "socketEntries" or if it's in the "intrinsic" list.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum DestinySocketArrayType {
    /// No documentation provided.
    Default = 0,
    /// No documentation provided.
    Intrinsic = 1,
}
