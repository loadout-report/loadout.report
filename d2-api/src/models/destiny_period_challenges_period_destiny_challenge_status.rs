/*
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * The version of the OpenAPI document: 2.16.0
 * Contact: support@bungie.com
 * Generated by: https://openapi-generator.tech
 */

/// DestinyPeriodChallengesPeriodDestinyChallengeStatus : Represents the status and other related information for a challenge that is - or was - available to a player.   A challenge is a bonus objective, generally tacked onto Quests or Activities, that provide additional variations on play.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DestinyPeriodChallengesPeriodDestinyChallengeStatus {
    #[serde(rename = "objective", skip_serializing_if = "Option::is_none")]
    pub objective: Option<Box<crate::models::DestinyChallengesDestinyChallengeStatusObjective>>,
}

impl DestinyPeriodChallengesPeriodDestinyChallengeStatus {
    /// Represents the status and other related information for a challenge that is - or was - available to a player.   A challenge is a bonus objective, generally tacked onto Quests or Activities, that provide additional variations on play.
    pub fn new() -> DestinyPeriodChallengesPeriodDestinyChallengeStatus {
        DestinyPeriodChallengesPeriodDestinyChallengeStatus {
            objective: None,
        }
    }
}


