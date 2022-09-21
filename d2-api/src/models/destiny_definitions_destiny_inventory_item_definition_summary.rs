/*
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * The version of the OpenAPI document: 2.16.0
 * Contact: support@bungie.com
 * Generated by: https://openapi-generator.tech
 */

/// DestinyDefinitionsDestinyInventoryItemDefinitionSummary : Summary data about the item.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DestinyDefinitionsDestinyInventoryItemDefinitionSummary {
    /// Apparently when rendering an item in a reward, this should be used as a sort priority. We're not doing it presently.
    #[serde(rename = "sortPriority", skip_serializing_if = "Option::is_none")]
    pub sort_priority: Option<i32>,
}

impl DestinyDefinitionsDestinyInventoryItemDefinitionSummary {
    /// Summary data about the item.
    pub fn new() -> DestinyDefinitionsDestinyInventoryItemDefinitionSummary {
        DestinyDefinitionsDestinyInventoryItemDefinitionSummary {
            sort_priority: None,
        }
    }
}


