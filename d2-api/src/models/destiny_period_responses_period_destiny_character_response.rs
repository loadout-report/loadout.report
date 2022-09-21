/*
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * The version of the OpenAPI document: 2.16.0
 * Contact: support@bungie.com
 * Generated by: https://openapi-generator.tech
 */

/// DestinyPeriodResponsesPeriodDestinyCharacterResponse : The response contract for GetDestinyCharacter, with components that can be returned for character and item-level data.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DestinyPeriodResponsesPeriodDestinyCharacterResponse {
    #[serde(rename = "inventory", skip_serializing_if = "Option::is_none")]
    pub inventory: Option<Box<crate::models::DestinyResponsesDestinyCharacterResponseInventory>>,
    #[serde(rename = "character", skip_serializing_if = "Option::is_none")]
    pub character: Option<Box<crate::models::DestinyResponsesDestinyCharacterResponseCharacter>>,
    #[serde(rename = "progressions", skip_serializing_if = "Option::is_none")]
    pub progressions: Option<Box<crate::models::DestinyResponsesDestinyCharacterResponseProgressions>>,
    #[serde(rename = "renderData", skip_serializing_if = "Option::is_none")]
    pub render_data: Option<Box<crate::models::DestinyResponsesDestinyCharacterResponseRenderData>>,
    #[serde(rename = "activities", skip_serializing_if = "Option::is_none")]
    pub activities: Option<Box<crate::models::DestinyResponsesDestinyCharacterResponseActivities>>,
    #[serde(rename = "equipment", skip_serializing_if = "Option::is_none")]
    pub equipment: Option<Box<crate::models::DestinyResponsesDestinyCharacterResponseEquipment>>,
    #[serde(rename = "kiosks", skip_serializing_if = "Option::is_none")]
    pub kiosks: Option<Box<crate::models::DestinyResponsesDestinyCharacterResponseKiosks>>,
    #[serde(rename = "plugSets", skip_serializing_if = "Option::is_none")]
    pub plug_sets: Option<Box<crate::models::DestinyResponsesDestinyCharacterResponsePlugSets>>,
    #[serde(rename = "presentationNodes", skip_serializing_if = "Option::is_none")]
    pub presentation_nodes: Option<Box<crate::models::DestinyResponsesDestinyProfileResponseProfilePresentationNodes>>,
    #[serde(rename = "records", skip_serializing_if = "Option::is_none")]
    pub records: Option<Box<crate::models::DestinyResponsesDestinyCharacterResponseRecords>>,
    #[serde(rename = "collectibles", skip_serializing_if = "Option::is_none")]
    pub collectibles: Option<Box<crate::models::DestinyResponsesDestinyCharacterResponseCollectibles>>,
    #[serde(rename = "itemComponents", skip_serializing_if = "Option::is_none")]
    pub item_components: Option<Box<crate::models::DestinyResponsesDestinyCharacterResponseItemComponents>>,
    #[serde(rename = "uninstancedItemComponents", skip_serializing_if = "Option::is_none")]
    pub uninstanced_item_components: Option<Box<crate::models::DestinyResponsesDestinyCharacterResponseUninstancedItemComponents>>,
    #[serde(rename = "currencyLookups", skip_serializing_if = "Option::is_none")]
    pub currency_lookups: Option<Box<crate::models::DestinyResponsesDestinyCharacterResponseCurrencyLookups>>,
}

impl DestinyPeriodResponsesPeriodDestinyCharacterResponse {
    /// The response contract for GetDestinyCharacter, with components that can be returned for character and item-level data.
    pub fn new() -> DestinyPeriodResponsesPeriodDestinyCharacterResponse {
        DestinyPeriodResponsesPeriodDestinyCharacterResponse {
            inventory: None,
            character: None,
            progressions: None,
            render_data: None,
            activities: None,
            equipment: None,
            kiosks: None,
            plug_sets: None,
            presentation_nodes: None,
            records: None,
            collectibles: None,
            item_components: None,
            uninstanced_item_components: None,
            currency_lookups: None,
        }
    }
}


