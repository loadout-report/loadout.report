/*
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * The version of the OpenAPI document: 2.16.0
 * Contact: support@bungie.com
 * Generated by: https://openapi-generator.tech
 */

/// DestinyPeriodDefinitionsPeriodDestinyEntitySearchResultItem : An individual Destiny Entity returned from the entity search.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DestinyPeriodDefinitionsPeriodDestinyEntitySearchResultItem {
    /// The hash identifier of the entity. You will use this to look up the DestinyDefinition relevant for the entity found.
    #[serde(rename = "hash", skip_serializing_if = "Option::is_none")]
    pub hash: Option<i32>,
    /// The type of entity, returned as a string matching the DestinyDefinition's contract class name. You'll have to have your own mapping from class names to actually looking up those definitions in the manifest databases.
    #[serde(rename = "entityType", skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<String>,
    #[serde(rename = "displayProperties", skip_serializing_if = "Option::is_none")]
    pub display_properties: Option<Box<crate::models::DestinyDefinitionsDestinyEntitySearchResultItemDisplayProperties>>,
    /// The ranking value for sorting that we calculated using our relevance formula. This will hopefully get better with time and iteration.
    #[serde(rename = "weight", skip_serializing_if = "Option::is_none")]
    pub weight: Option<f64>,
}

impl DestinyPeriodDefinitionsPeriodDestinyEntitySearchResultItem {
    /// An individual Destiny Entity returned from the entity search.
    pub fn new() -> DestinyPeriodDefinitionsPeriodDestinyEntitySearchResultItem {
        DestinyPeriodDefinitionsPeriodDestinyEntitySearchResultItem {
            hash: None,
            entity_type: None,
            display_properties: None,
            weight: None,
        }
    }
}


