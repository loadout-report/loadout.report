/*
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * The version of the OpenAPI document: 2.16.0
 * Contact: support@bungie.com
 * Generated by: https://openapi-generator.tech
 */

/// DestinyResponsesDestinyCharacterResponsePlugSets : When sockets refer to reusable Plug Sets (see DestinyPlugSetDefinition for more info), this is the set of plugs and their states that are scoped to this character.  This comes back with ItemSockets, as it is needed for a complete picture of the sockets on requested items.  COMPONENT TYPE: ItemSockets



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DestinyResponsesDestinyCharacterResponsePlugSets {
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Box<crate::models::DestinyPeriodComponentsPeriodPlugSetsPeriodDestinyPlugSetsComponent>>,
    #[serde(rename = "privacy", skip_serializing_if = "Option::is_none")]
    pub privacy: Option<i32>,
    /// If true, this component is disabled.
    #[serde(rename = "disabled", skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
}

impl DestinyResponsesDestinyCharacterResponsePlugSets {
    /// When sockets refer to reusable Plug Sets (see DestinyPlugSetDefinition for more info), this is the set of plugs and their states that are scoped to this character.  This comes back with ItemSockets, as it is needed for a complete picture of the sockets on requested items.  COMPONENT TYPE: ItemSockets
    pub fn new() -> DestinyResponsesDestinyCharacterResponsePlugSets {
        DestinyResponsesDestinyCharacterResponsePlugSets {
            data: None,
            privacy: None,
            disabled: None,
        }
    }
}


