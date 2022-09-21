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
pub struct SocialPeriodFriendsPeriodBungieFriendListResponse {
    #[serde(rename = "friends", skip_serializing_if = "Option::is_none")]
    pub friends: Option<Vec<crate::models::SocialPeriodFriendsPeriodBungieFriend>>,
}

impl SocialPeriodFriendsPeriodBungieFriendListResponse {
    pub fn new() -> SocialPeriodFriendsPeriodBungieFriendListResponse {
        SocialPeriodFriendsPeriodBungieFriendListResponse {
            friends: None,
        }
    }
}


