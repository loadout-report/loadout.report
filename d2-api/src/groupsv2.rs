use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct GroupUserInfoCard {
    /// This will be the display name the clan server last saw the user as.
    /// If the account is an active cross save override, this will be the display name to use.
    /// Otherwise, this will match the displayName property.
    #[serde(rename = "LastSeenDisplayName")]
    pub last_seen_display_name: String,
    /// The platform of the LastSeenDisplayName
    // todo: make id a type
    #[serde(rename = "LastSeenDisplayNameType")]
    pub last_seen_display_name_type: i32,

    #[serde(flatten)]
    pub info_card: crate::user::InfoCard,
}
