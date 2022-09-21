/*
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * The version of the OpenAPI document: 2.16.0
 * Contact: support@bungie.com
 * Generated by: https://openapi-generator.tech
 */


/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DestinyPeriodAdvancedPeriodAwaType {
    #[serde(rename = "0")]
    Variant0,
    #[serde(rename = "1")]
    Variant1,

}

impl ToString for DestinyPeriodAdvancedPeriodAwaType {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0 => String::from("0"),
            Self::Variant1 => String::from("1"),
        }
    }
}

impl Default for DestinyPeriodAdvancedPeriodAwaType {
    fn default() -> DestinyPeriodAdvancedPeriodAwaType {
        Self::Variant0
    }
}




