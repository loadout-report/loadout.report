use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};
use serde_with::{serde_as, DisplayFromStr};
pub mod actions;
/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyItemTransferRequest {

    /// No documentation provided.
    #[serde(with = "crate::unfuck_js::stringified_numbers")]
    pub character_id: i64,
    /// The instance ID of the item for this action request.
    #[serde(with = "crate::unfuck_js::stringified_numbers")]
    pub item_id: i64,
    /// No documentation provided.
    pub item_reference_hash: crate::id::Id<crate::generated::models::destiny::definitions::DestinyInventoryItemDefinition>,
    /// No documentation provided.
    pub membership_type: crate::generated::models::BungieMembershipType,
    /// No documentation provided.
    pub stack_size: i32,
    /// No documentation provided.
    pub transfer_to_vault: bool,
}
