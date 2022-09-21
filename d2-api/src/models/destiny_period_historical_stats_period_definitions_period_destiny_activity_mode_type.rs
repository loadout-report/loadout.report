/*
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * The version of the OpenAPI document: 2.16.0
 * Contact: support@bungie.com
 * Generated by: https://openapi-generator.tech
 */

/// DestinyPeriodHistoricalStatsPeriodDefinitionsPeriodDestinyActivityModeType : For historical reasons, this list will have both D1 and D2-relevant Activity Modes in it. Please don't take this to mean that some D1-only feature is coming back!

/// For historical reasons, this list will have both D1 and D2-relevant Activity Modes in it. Please don't take this to mean that some D1-only feature is coming back!
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DestinyPeriodHistoricalStatsPeriodDefinitionsPeriodDestinyActivityModeType {
    #[serde(rename = "0")]
    Variant0,
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
    #[serde(rename = "15")]
    Variant15,
    #[serde(rename = "16")]
    Variant16,
    #[serde(rename = "17")]
    Variant17,
    #[serde(rename = "18")]
    Variant18,
    #[serde(rename = "19")]
    Variant19,
    #[serde(rename = "20")]
    Variant20,
    #[serde(rename = "21")]
    Variant21,
    #[serde(rename = "22")]
    Variant22,
    #[serde(rename = "24")]
    Variant24,
    #[serde(rename = "25")]
    Variant25,
    #[serde(rename = "26")]
    Variant26,
    #[serde(rename = "27")]
    Variant27,
    #[serde(rename = "28")]
    Variant28,
    #[serde(rename = "29")]
    Variant29,
    #[serde(rename = "30")]
    Variant30,
    #[serde(rename = "31")]
    Variant31,
    #[serde(rename = "32")]
    Variant32,
    #[serde(rename = "37")]
    Variant37,
    #[serde(rename = "38")]
    Variant38,
    #[serde(rename = "39")]
    Variant39,
    #[serde(rename = "40")]
    Variant40,
    #[serde(rename = "41")]
    Variant41,
    #[serde(rename = "42")]
    Variant42,
    #[serde(rename = "43")]
    Variant43,
    #[serde(rename = "44")]
    Variant44,
    #[serde(rename = "45")]
    Variant45,
    #[serde(rename = "46")]
    Variant46,
    #[serde(rename = "47")]
    Variant47,
    #[serde(rename = "48")]
    Variant48,
    #[serde(rename = "49")]
    Variant49,
    #[serde(rename = "50")]
    Variant50,
    #[serde(rename = "51")]
    Variant51,
    #[serde(rename = "52")]
    Variant52,
    #[serde(rename = "53")]
    Variant53,
    #[serde(rename = "54")]
    Variant54,
    #[serde(rename = "55")]
    Variant55,
    #[serde(rename = "56")]
    Variant56,
    #[serde(rename = "57")]
    Variant57,
    #[serde(rename = "58")]
    Variant58,
    #[serde(rename = "59")]
    Variant59,
    #[serde(rename = "60")]
    Variant60,
    #[serde(rename = "61")]
    Variant61,
    #[serde(rename = "62")]
    Variant62,
    #[serde(rename = "63")]
    Variant63,
    #[serde(rename = "64")]
    Variant64,
    #[serde(rename = "65")]
    Variant65,
    #[serde(rename = "66")]
    Variant66,
    #[serde(rename = "67")]
    Variant67,
    #[serde(rename = "68")]
    Variant68,
    #[serde(rename = "69")]
    Variant69,
    #[serde(rename = "70")]
    Variant70,
    #[serde(rename = "71")]
    Variant71,
    #[serde(rename = "72")]
    Variant72,
    #[serde(rename = "73")]
    Variant73,
    #[serde(rename = "74")]
    Variant74,
    #[serde(rename = "75")]
    Variant75,
    #[serde(rename = "76")]
    Variant76,
    #[serde(rename = "77")]
    Variant77,
    #[serde(rename = "78")]
    Variant78,
    #[serde(rename = "79")]
    Variant79,
    #[serde(rename = "80")]
    Variant80,
    #[serde(rename = "81")]
    Variant81,
    #[serde(rename = "82")]
    Variant82,
    #[serde(rename = "83")]
    Variant83,
    #[serde(rename = "84")]
    Variant84,
    #[serde(rename = "85")]
    Variant85,
    #[serde(rename = "86")]
    Variant86,
    #[serde(rename = "87")]
    Variant87,
    #[serde(rename = "88")]
    Variant88,
    #[serde(rename = "89")]
    Variant89,
    #[serde(rename = "90")]
    Variant90,

}

impl ToString for DestinyPeriodHistoricalStatsPeriodDefinitionsPeriodDestinyActivityModeType {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0 => String::from("0"),
            Self::Variant2 => String::from("2"),
            Self::Variant3 => String::from("3"),
            Self::Variant4 => String::from("4"),
            Self::Variant5 => String::from("5"),
            Self::Variant6 => String::from("6"),
            Self::Variant7 => String::from("7"),
            Self::Variant9 => String::from("9"),
            Self::Variant10 => String::from("10"),
            Self::Variant11 => String::from("11"),
            Self::Variant12 => String::from("12"),
            Self::Variant13 => String::from("13"),
            Self::Variant15 => String::from("15"),
            Self::Variant16 => String::from("16"),
            Self::Variant17 => String::from("17"),
            Self::Variant18 => String::from("18"),
            Self::Variant19 => String::from("19"),
            Self::Variant20 => String::from("20"),
            Self::Variant21 => String::from("21"),
            Self::Variant22 => String::from("22"),
            Self::Variant24 => String::from("24"),
            Self::Variant25 => String::from("25"),
            Self::Variant26 => String::from("26"),
            Self::Variant27 => String::from("27"),
            Self::Variant28 => String::from("28"),
            Self::Variant29 => String::from("29"),
            Self::Variant30 => String::from("30"),
            Self::Variant31 => String::from("31"),
            Self::Variant32 => String::from("32"),
            Self::Variant37 => String::from("37"),
            Self::Variant38 => String::from("38"),
            Self::Variant39 => String::from("39"),
            Self::Variant40 => String::from("40"),
            Self::Variant41 => String::from("41"),
            Self::Variant42 => String::from("42"),
            Self::Variant43 => String::from("43"),
            Self::Variant44 => String::from("44"),
            Self::Variant45 => String::from("45"),
            Self::Variant46 => String::from("46"),
            Self::Variant47 => String::from("47"),
            Self::Variant48 => String::from("48"),
            Self::Variant49 => String::from("49"),
            Self::Variant50 => String::from("50"),
            Self::Variant51 => String::from("51"),
            Self::Variant52 => String::from("52"),
            Self::Variant53 => String::from("53"),
            Self::Variant54 => String::from("54"),
            Self::Variant55 => String::from("55"),
            Self::Variant56 => String::from("56"),
            Self::Variant57 => String::from("57"),
            Self::Variant58 => String::from("58"),
            Self::Variant59 => String::from("59"),
            Self::Variant60 => String::from("60"),
            Self::Variant61 => String::from("61"),
            Self::Variant62 => String::from("62"),
            Self::Variant63 => String::from("63"),
            Self::Variant64 => String::from("64"),
            Self::Variant65 => String::from("65"),
            Self::Variant66 => String::from("66"),
            Self::Variant67 => String::from("67"),
            Self::Variant68 => String::from("68"),
            Self::Variant69 => String::from("69"),
            Self::Variant70 => String::from("70"),
            Self::Variant71 => String::from("71"),
            Self::Variant72 => String::from("72"),
            Self::Variant73 => String::from("73"),
            Self::Variant74 => String::from("74"),
            Self::Variant75 => String::from("75"),
            Self::Variant76 => String::from("76"),
            Self::Variant77 => String::from("77"),
            Self::Variant78 => String::from("78"),
            Self::Variant79 => String::from("79"),
            Self::Variant80 => String::from("80"),
            Self::Variant81 => String::from("81"),
            Self::Variant82 => String::from("82"),
            Self::Variant83 => String::from("83"),
            Self::Variant84 => String::from("84"),
            Self::Variant85 => String::from("85"),
            Self::Variant86 => String::from("86"),
            Self::Variant87 => String::from("87"),
            Self::Variant88 => String::from("88"),
            Self::Variant89 => String::from("89"),
            Self::Variant90 => String::from("90"),
        }
    }
}

impl Default for DestinyPeriodHistoricalStatsPeriodDefinitionsPeriodDestinyActivityModeType {
    fn default() -> DestinyPeriodHistoricalStatsPeriodDefinitionsPeriodDestinyActivityModeType {
        Self::Variant0
    }
}




