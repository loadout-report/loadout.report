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
pub enum ForumsPeriodForumFlagsEnum {
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
    #[serde(rename = "32")]
    Variant32,
    #[serde(rename = "64")]
    Variant64,
    #[serde(rename = "128")]
    Variant128,

}

impl ToString for ForumsPeriodForumFlagsEnum {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0 => String::from("0"),
            Self::Variant1 => String::from("1"),
            Self::Variant2 => String::from("2"),
            Self::Variant4 => String::from("4"),
            Self::Variant8 => String::from("8"),
            Self::Variant16 => String::from("16"),
            Self::Variant32 => String::from("32"),
            Self::Variant64 => String::from("64"),
            Self::Variant128 => String::from("128"),
        }
    }
}

impl Default for ForumsPeriodForumFlagsEnum {
    fn default() -> ForumsPeriodForumFlagsEnum {
        Self::Variant0
    }
}




