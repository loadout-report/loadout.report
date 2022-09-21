/*
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * The version of the OpenAPI document: 2.16.0
 * Contact: support@bungie.com
 * Generated by: https://openapi-generator.tech
 */

/// DestinyPeriodDefinitionsPeriodDestinyActivityTypeDefinition : The definition for an Activity Type.  In Destiny 2, an Activity Type represents a conceptual categorization of Activities.  These are most commonly used in the game for the subtitle under Activities, but BNet uses them extensively to identify and group activities by their common properties.  Unfortunately, there has been a movement away from providing the richer data in Destiny 2 that we used to get in Destiny 1 for Activity Types. For instance, Nightfalls are grouped under the same Activity Type as regular Strikes.   For this reason, BNet will eventually migrate toward Activity Modes as a better indicator of activity category. But for the time being, it is still referred to in many places across our codebase.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DestinyPeriodDefinitionsPeriodDestinyActivityTypeDefinition {
    #[serde(rename = "displayProperties", skip_serializing_if = "Option::is_none")]
    pub display_properties: Option<Box<crate::models::DestinyPeriodDefinitionsPeriodCommonPeriodDestinyDisplayPropertiesDefinition>>,
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

impl DestinyPeriodDefinitionsPeriodDestinyActivityTypeDefinition {
    /// The definition for an Activity Type.  In Destiny 2, an Activity Type represents a conceptual categorization of Activities.  These are most commonly used in the game for the subtitle under Activities, but BNet uses them extensively to identify and group activities by their common properties.  Unfortunately, there has been a movement away from providing the richer data in Destiny 2 that we used to get in Destiny 1 for Activity Types. For instance, Nightfalls are grouped under the same Activity Type as regular Strikes.   For this reason, BNet will eventually migrate toward Activity Modes as a better indicator of activity category. But for the time being, it is still referred to in many places across our codebase.
    pub fn new() -> DestinyPeriodDefinitionsPeriodDestinyActivityTypeDefinition {
        DestinyPeriodDefinitionsPeriodDestinyActivityTypeDefinition {
            display_properties: None,
            hash: None,
            index: None,
            redacted: None,
        }
    }
}


