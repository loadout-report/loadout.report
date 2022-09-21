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
pub enum DestinyPeriodDestinyActivityNavPointType {
    #[serde(rename = "0")]
    Variant0,
    #[serde(rename = "1")]
    Variant1,
    #[serde(rename = "2")]
    Variant2,
    #[serde(rename = "3")]
    Variant3,
    #[serde(rename = "4")]
    Variant4,
    #[serde(rename = "5")]
    Variant5,
    #[serde(rename = "6")]
    Variant6,
    #[serde(rename = "7")]
    Variant7,
    #[serde(rename = "8")]
    Variant8,
    #[serde(rename = "9")]
    Variant9,
    #[serde(rename = "10")]
    Variant10,
    #[serde(rename = "11")]
    Variant11,
    #[serde(rename = "12")]
    Variant12,
    #[serde(rename = "13")]
    Variant13,
    #[serde(rename = "14")]
    Variant14,
    #[serde(rename = "15")]
    Variant15,
    #[serde(rename = "16")]
    Variant16,

}

impl ToString for DestinyPeriodDestinyActivityNavPointType {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0 => String::from("0"),
            Self::Variant1 => String::from("1"),
            Self::Variant2 => String::from("2"),
            Self::Variant3 => String::from("3"),
            Self::Variant4 => String::from("4"),
            Self::Variant5 => String::from("5"),
            Self::Variant6 => String::from("6"),
            Self::Variant7 => String::from("7"),
            Self::Variant8 => String::from("8"),
            Self::Variant9 => String::from("9"),
            Self::Variant10 => String::from("10"),
            Self::Variant11 => String::from("11"),
            Self::Variant12 => String::from("12"),
            Self::Variant13 => String::from("13"),
            Self::Variant14 => String::from("14"),
            Self::Variant15 => String::from("15"),
            Self::Variant16 => String::from("16"),
        }
    }
}

impl Default for DestinyPeriodDestinyActivityNavPointType {
    fn default() -> DestinyPeriodDestinyActivityNavPointType {
        Self::Variant0
    }
}




