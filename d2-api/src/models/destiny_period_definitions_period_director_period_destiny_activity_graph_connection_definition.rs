/*
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * The version of the OpenAPI document: 2.16.0
 * Contact: support@bungie.com
 * Generated by: https://openapi-generator.tech
 */

/// DestinyPeriodDefinitionsPeriodDirectorPeriodDestinyActivityGraphConnectionDefinition : Nodes on a graph can be visually connected: this appears to be the information about which nodes to link. It appears to lack more detailed information, such as the path for that linking.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DestinyPeriodDefinitionsPeriodDirectorPeriodDestinyActivityGraphConnectionDefinition {
    #[serde(rename = "sourceNodeHash", skip_serializing_if = "Option::is_none")]
    pub source_node_hash: Option<i32>,
    #[serde(rename = "destNodeHash", skip_serializing_if = "Option::is_none")]
    pub dest_node_hash: Option<i32>,
}

impl DestinyPeriodDefinitionsPeriodDirectorPeriodDestinyActivityGraphConnectionDefinition {
    /// Nodes on a graph can be visually connected: this appears to be the information about which nodes to link. It appears to lack more detailed information, such as the path for that linking.
    pub fn new() -> DestinyPeriodDefinitionsPeriodDirectorPeriodDestinyActivityGraphConnectionDefinition {
        DestinyPeriodDefinitionsPeriodDirectorPeriodDestinyActivityGraphConnectionDefinition {
            source_node_hash: None,
            dest_node_hash: None,
        }
    }
}


