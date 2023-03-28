use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};
use serde_with::{serde_as, DisplayFromStr};

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AwaAuthorizationResult {

    /// MembershipType from the permission request.
    pub membership_type: crate::generated::models::BungieMembershipType,
    /// Indication of how the user responded to the request. If the value is "Approved" the actionToken will contain the token that can be presented when performing the advanced write action.
    pub user_selection: crate::generated::models::destiny::advanced::AwaUserSelection,
    /// Time, UTC, when token expires.
    pub valid_until: Option<chrono::DateTime<chrono::Utc>>,
    /// No documentation provided.
    pub response_reason: crate::generated::models::destiny::advanced::AwaResponseReason,
    /// Advanced Write Action Type from the permission request.
    pub r#type: crate::generated::models::destiny::advanced::AwaType,
    /// This token may be used to perform the requested action this number of times, at a maximum. If this value is 0, then there is no limit.
    pub maximum_number_of_uses: i32,
    /// Credential used to prove the user authorized an advanced write action.
    pub action_token: String,
    /// Message to the app developer to help understand the response.
    pub developer_note: String,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AwaInitializeResponse {

    /// ID used to get the token. Present this ID to the user as it will identify this specific request on their device.
    pub correlation_id: String,
    /// True if the PUSH message will only be sent to the device that made this request.
    pub sent_to_self: bool,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AwaPermissionRequested {

    /// Destiny character ID, if applicable, that will be affected by the action.
    #[serde(with = "crate::unfuck_js::nullable_stringified_numbers")]
    pub character_id: Option<i64>,
    /// Destiny membership type of the account to modify.
    pub membership_type: crate::generated::models::BungieMembershipType,
    /// Item instance ID the action shall be applied to. This is optional for all but a new AwaType values. Rule of thumb is to provide the item instance ID if one is available.
    #[serde(with = "crate::unfuck_js::nullable_stringified_numbers")]
    pub affected_item_id: Option<i64>,
    /// Type of advanced write action.
    pub r#type: crate::generated::models::destiny::advanced::AwaType,
}

/// No documentation provided.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum AwaResponseReason {
    /// No documentation provided.
    None = 0,
    /// User provided an answer
    Answered = 1,
    /// The HTTP request timed out, a new request may be made and an answer may still be provided.
    TimedOut = 2,
    /// This request was replaced by another request.
    Replaced = 3,
}

/// No documentation provided.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum AwaType {
    /// No documentation provided.
    None = 0,
    /// Insert plugs into sockets.
    InsertPlugs = 1,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AwaUserResponse {

    /// Indication of the selection the user has made (Approving or rejecting the action)
    pub selection: crate::generated::models::destiny::advanced::AwaUserSelection,
    /// Secret nonce received via the PUSH notification.
    pub nonce: i32,
    /// Correlation ID of the request
    pub correlation_id: String,
}

/// No documentation provided.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum AwaUserSelection {
    /// No documentation provided.
    None = 0,
    /// No documentation provided.
    Rejected = 1,
    /// No documentation provided.
    Approved = 2,
}
