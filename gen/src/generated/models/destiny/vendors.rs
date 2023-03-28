use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};
use serde_with::{serde_as, DisplayFromStr};

/// If a character purchased an item that is refundable, a Vendor Receipt will be created on the user's Destiny Profile. These expire after a configurable period of time, but until then can be used to get refunds on items. BNet does not provide the ability to refund a purchase *yet*, but you know.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyVendorReceipt {

    /// The identifier of this receipt.
    pub sequence_number: i32,
    /// The item that was received, and its quantity.
    pub item_received: crate::generated::models::destiny::DestinyItemQuantity,
    /// The date at which this receipt is rendered invalid.
    pub expires_on: chrono::DateTime<chrono::Utc>,
    /// The seconds since epoch at which this receipt is rendered invalid.
    #[serde(with = "crate::unfuck_js::stringified_numbers")]
    pub time_to_expiration: i64,
    /// The ID of the character who made the purchase.
    #[serde(with = "crate::unfuck_js::stringified_numbers")]
    pub purchased_by_character_id: i64,
    /// Whether you can get a refund, and what happens in order for the refund to be received. See the DestinyVendorItemRefundPolicy enum for details.
    pub refund_policy: crate::generated::models::destiny::DestinyVendorItemRefundPolicy,
    /// The amount paid for the item, in terms of items that were consumed in the purchase and their quantity.
    pub currency_paid: i32,
    /// The unlock flag used to determine whether you still have the purchased item.
    pub license_unlock_hash: u32,
}
