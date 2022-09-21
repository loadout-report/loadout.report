/*
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * The version of the OpenAPI document: 2.16.0
 * Contact: support@bungie.com
 * Generated by: https://openapi-generator.tech
 */

/// DestinyPeriodEquipFailureReason : The reasons why an item cannot be equipped, if any. Many flags can be set, or \"None\" if

/// The reasons why an item cannot be equipped, if any. Many flags can be set, or \"None\" if
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DestinyPeriodEquipFailureReason {
    #[serde(rename = "0")]
    Variant0,
    #[serde(rename = "1")]
    Variant1,
    #[serde(rename = "2")]
    Variant2,
    #[serde(rename = "4")]
    Variant4,
    #[serde(rename = "8")]
    Variant8,
    #[serde(rename = "16")]
    Variant16,

}

impl ToString for DestinyPeriodEquipFailureReason {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0 => String::from("0"),
            Self::Variant1 => String::from("1"),
            Self::Variant2 => String::from("2"),
            Self::Variant4 => String::from("4"),
            Self::Variant8 => String::from("8"),
            Self::Variant16 => String::from("16"),
        }
    }
}

impl Default for DestinyPeriodEquipFailureReason {
    fn default() -> DestinyPeriodEquipFailureReason {
        Self::Variant0
    }
}




