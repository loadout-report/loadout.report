/*
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * The version of the OpenAPI document: 2.16.0
 * Contact: support@bungie.com
 * Generated by: https://openapi-generator.tech
 */

/// DestinyPeriodDefinitionsPeriodDestinyDefinition : Provides common properties for destiny definitions.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DestinyPeriodDefinitionsPeriodDestinyDefinition {
    /// The unique identifier for this entity. Guaranteed to be unique for the type of entity, but not globally.  When entities refer to each other in Destiny content, it is this hash that they are referring to.
    #[serde(rename = "hash", skip_serializing_if = "Option::is_none")]
    pub hash: Option<i32>,
    /// The index of the entity as it was found in the investment tables.
    #[serde(rename = "index", skip_serializing_if = "Option::is_none")]
    pub index: Option<i32>,
    /// If this is true, then there is an entity with this identifier/type combination, but BNet is not yet allowed to show it. Sorry!
    #[serde(rename = "redacted", skip_serializing_if = "Option::is_none")]
    pub redacted: Option<bool>,
}

impl DestinyPeriodDefinitionsPeriodDestinyDefinition {
    /// Provides common properties for destiny definitions.
    pub fn new() -> DestinyPeriodDefinitionsPeriodDestinyDefinition {
        DestinyPeriodDefinitionsPeriodDestinyDefinition {
            hash: None,
            index: None,
            redacted: None,
        }
    }
}


