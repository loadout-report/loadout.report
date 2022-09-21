/*
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * The version of the OpenAPI document: 2.16.0
 * Contact: support@bungie.com
 * Generated by: https://openapi-generator.tech
 */

/// DestinyResponsesDestinyCollectibleNodeDetailResponseCollectibleItemComponents : Item components, keyed by the item hash of the items pointed at collectibles found under the requested Presentation Node.  NOTE: I had a lot of hemming and hawing about whether these should be keyed by collectible hash or item hash... but ultimately having it be keyed by item hash meant that UI that already uses DestinyItemComponentSet data wouldn't have to have a special override to do the collectible -> item lookup once you delve into an item's details, and it also meant that you didn't have to remember that the Hash being used as the key for plugSets was different from the Hash being used for the other Dictionaries. As a result, using the Item Hash felt like the least crappy solution.  We may all come to regret this decision. We will see.  COMPONENT TYPE: [See inside the DestinyItemComponentSet contract for component types.]



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DestinyResponsesDestinyCollectibleNodeDetailResponseCollectibleItemComponents {
    #[serde(rename = "instances", skip_serializing_if = "Option::is_none")]
    pub instances: Option<Box<crate::models::DictionaryComponentResponseOfuint32AndDestinyItemInstanceComponent>>,
    #[serde(rename = "renderData", skip_serializing_if = "Option::is_none")]
    pub render_data: Option<Box<crate::models::DictionaryComponentResponseOfuint32AndDestinyItemRenderComponent>>,
    #[serde(rename = "stats", skip_serializing_if = "Option::is_none")]
    pub stats: Option<Box<crate::models::DictionaryComponentResponseOfuint32AndDestinyItemStatsComponent>>,
    #[serde(rename = "sockets", skip_serializing_if = "Option::is_none")]
    pub sockets: Option<Box<crate::models::DictionaryComponentResponseOfuint32AndDestinyItemSocketsComponent>>,
    #[serde(rename = "reusablePlugs", skip_serializing_if = "Option::is_none")]
    pub reusable_plugs: Option<Box<crate::models::DictionaryComponentResponseOfuint32AndDestinyItemReusablePlugsComponent>>,
    #[serde(rename = "plugObjectives", skip_serializing_if = "Option::is_none")]
    pub plug_objectives: Option<Box<crate::models::DictionaryComponentResponseOfuint32AndDestinyItemPlugObjectivesComponent>>,
    #[serde(rename = "talentGrids", skip_serializing_if = "Option::is_none")]
    pub talent_grids: Option<Box<crate::models::DictionaryComponentResponseOfuint32AndDestinyItemTalentGridComponent>>,
    #[serde(rename = "plugStates", skip_serializing_if = "Option::is_none")]
    pub plug_states: Option<Box<crate::models::DictionaryComponentResponseOfuint32AndDestinyItemPlugComponent>>,
    #[serde(rename = "objectives", skip_serializing_if = "Option::is_none")]
    pub objectives: Option<Box<crate::models::DictionaryComponentResponseOfuint32AndDestinyItemObjectivesComponent>>,
    #[serde(rename = "perks", skip_serializing_if = "Option::is_none")]
    pub perks: Option<Box<crate::models::DictionaryComponentResponseOfuint32AndDestinyItemPerksComponent>>,
}

impl DestinyResponsesDestinyCollectibleNodeDetailResponseCollectibleItemComponents {
    /// Item components, keyed by the item hash of the items pointed at collectibles found under the requested Presentation Node.  NOTE: I had a lot of hemming and hawing about whether these should be keyed by collectible hash or item hash... but ultimately having it be keyed by item hash meant that UI that already uses DestinyItemComponentSet data wouldn't have to have a special override to do the collectible -> item lookup once you delve into an item's details, and it also meant that you didn't have to remember that the Hash being used as the key for plugSets was different from the Hash being used for the other Dictionaries. As a result, using the Item Hash felt like the least crappy solution.  We may all come to regret this decision. We will see.  COMPONENT TYPE: [See inside the DestinyItemComponentSet contract for component types.]
    pub fn new() -> DestinyResponsesDestinyCollectibleNodeDetailResponseCollectibleItemComponents {
        DestinyResponsesDestinyCollectibleNodeDetailResponseCollectibleItemComponents {
            instances: None,
            render_data: None,
            stats: None,
            sockets: None,
            reusable_plugs: None,
            plug_objectives: None,
            talent_grids: None,
            plug_states: None,
            objectives: None,
            perks: None,
        }
    }
}


