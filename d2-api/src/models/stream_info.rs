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
pub struct StreamInfo {
    #[serde(rename = "ChannelName", skip_serializing_if = "Option::is_none")]
    pub channel_name: Option<String>,
}

impl StreamInfo {
    pub fn new() -> StreamInfo {
        StreamInfo {
            channel_name: None,
        }
    }
}


