pub mod actions;

use serde::{Serialize, Deserialize};


/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyItemTransferRequest {

    /// No documentation provided.
    pub transfer_to_vault: bool,
    /// No documentation provided.
    #[serde(with = "crate::unfuck_js::stringified_numbers")]
    pub character_id: i64,
    /// The instance ID of the item for this action request.
    #[serde(with = "crate::unfuck_js::stringified_numbers")]
    pub item_id: i64,
    /// No documentation provided.
    pub membership_type: i32,
    /// No documentation provided.
    pub stack_size: i32,
    /// No documentation provided.
    pub item_reference_hash: u32,
}
