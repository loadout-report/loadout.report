/*
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * The version of the OpenAPI document: 2.16.0
 * Contact: support@bungie.com
 * Generated by: https://openapi-generator.tech
 */

/// DestinyPeriodEntitiesPeriodCharactersPeriodDestinyCharacterRenderComponent : Only really useful if you're attempting to render the character's current appearance in 3D, this returns a bare minimum of information, pre-aggregated, that you'll need to perform that rendering. Note that you need to combine this with other 3D assets and data from our servers.  Examine the Javascript returned by https://bungie.net/sharedbundle/spasm to see how we use this data, but be warned: the rabbit hole goes pretty deep.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DestinyPeriodEntitiesPeriodCharactersPeriodDestinyCharacterRenderComponent {
    /// Custom dyes, calculated by iterating over the character's equipped items. Useful for pre-fetching all of the dye data needed from our server.
    #[serde(rename = "customDyes", skip_serializing_if = "Option::is_none")]
    pub custom_dyes: Option<Vec<crate::models::DestinyPeriodDyeReference>>,
    #[serde(rename = "customization", skip_serializing_if = "Option::is_none")]
    pub customization: Option<Box<crate::models::DestinyEntitiesCharactersDestinyCharacterRenderComponentCustomization>>,
    #[serde(rename = "peerView", skip_serializing_if = "Option::is_none")]
    pub peer_view: Option<Box<crate::models::DestinyEntitiesCharactersDestinyCharacterRenderComponentPeerView>>,
}

impl DestinyPeriodEntitiesPeriodCharactersPeriodDestinyCharacterRenderComponent {
    /// Only really useful if you're attempting to render the character's current appearance in 3D, this returns a bare minimum of information, pre-aggregated, that you'll need to perform that rendering. Note that you need to combine this with other 3D assets and data from our servers.  Examine the Javascript returned by https://bungie.net/sharedbundle/spasm to see how we use this data, but be warned: the rabbit hole goes pretty deep.
    pub fn new() -> DestinyPeriodEntitiesPeriodCharactersPeriodDestinyCharacterRenderComponent {
        DestinyPeriodEntitiesPeriodCharactersPeriodDestinyCharacterRenderComponent {
            custom_dyes: None,
            customization: None,
            peer_view: None,
        }
    }
}


