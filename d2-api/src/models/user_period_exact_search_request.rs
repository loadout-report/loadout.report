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
pub struct UserPeriodExactSearchRequest {
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "displayNameCode", skip_serializing_if = "Option::is_none")]
    pub display_name_code: Option<i32>,
}

impl UserPeriodExactSearchRequest {
    pub fn new() -> UserPeriodExactSearchRequest {
        UserPeriodExactSearchRequest {
            display_name: None,
            display_name_code: None,
        }
    }
}


