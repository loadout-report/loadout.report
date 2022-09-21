/*
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * The version of the OpenAPI document: 2.16.0
 * Contact: support@bungie.com
 * Generated by: https://openapi-generator.tech
 */

/// DestinyEntitiesCharactersDestinyCharacterProgressionComponentSeasonalArtifact : Data related to your progress on the current season's artifact that can vary per character.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DestinyEntitiesCharactersDestinyCharacterProgressionComponentSeasonalArtifact {
    #[serde(rename = "artifactHash", skip_serializing_if = "Option::is_none")]
    pub artifact_hash: Option<i32>,
    #[serde(rename = "pointsUsed", skip_serializing_if = "Option::is_none")]
    pub points_used: Option<i32>,
    #[serde(rename = "resetCount", skip_serializing_if = "Option::is_none")]
    pub reset_count: Option<i32>,
    #[serde(rename = "tiers", skip_serializing_if = "Option::is_none")]
    pub tiers: Option<Vec<crate::models::DestinyPeriodArtifactsPeriodDestinyArtifactTier>>,
}

impl DestinyEntitiesCharactersDestinyCharacterProgressionComponentSeasonalArtifact {
    /// Data related to your progress on the current season's artifact that can vary per character.
    pub fn new() -> DestinyEntitiesCharactersDestinyCharacterProgressionComponentSeasonalArtifact {
        DestinyEntitiesCharactersDestinyCharacterProgressionComponentSeasonalArtifact {
            artifact_hash: None,
            points_used: None,
            reset_count: None,
            tiers: None,
        }
    }
}


