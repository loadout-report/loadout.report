/*
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * The version of the OpenAPI document: 2.16.0
 * Contact: support@bungie.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Destiny2GetHistoricalStatsForAccount200Response {
    #[serde(rename = "Response", skip_serializing_if = "Option::is_none")]
    pub response: Option<Box<crate::models::DestinyPeriodHistoricalStatsPeriodDestinyHistoricalStatsAccountResult>>,
    #[serde(rename = "ErrorCode", skip_serializing_if = "Option::is_none")]
    pub error_code: Option<i32>,
    #[serde(rename = "ThrottleSeconds", skip_serializing_if = "Option::is_none")]
    pub throttle_seconds: Option<i32>,
    #[serde(rename = "ErrorStatus", skip_serializing_if = "Option::is_none")]
    pub error_status: Option<String>,
    #[serde(rename = "Message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "MessageData", skip_serializing_if = "Option::is_none")]
    pub message_data: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "DetailedErrorTrace", skip_serializing_if = "Option::is_none")]
    pub detailed_error_trace: Option<String>,
}

impl Destiny2GetHistoricalStatsForAccount200Response {
    pub fn new() -> Destiny2GetHistoricalStatsForAccount200Response {
        Destiny2GetHistoricalStatsForAccount200Response {
            response: None,
            error_code: None,
            throttle_seconds: None,
            error_status: None,
            message: None,
            message_data: None,
            detailed_error_trace: None,
        }
    }
}


