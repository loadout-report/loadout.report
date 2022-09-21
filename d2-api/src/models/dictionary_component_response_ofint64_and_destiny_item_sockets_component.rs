/*
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * The version of the OpenAPI document: 2.16.0
 * Contact: support@bungie.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DictionaryComponentResponseOfint64AndDestinyItemSocketsComponent {
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<::std::collections::HashMap<String, crate::models::DestinyPeriodEntitiesPeriodItemsPeriodDestinyItemSocketsComponent>>,
    #[serde(rename = "privacy", skip_serializing_if = "Option::is_none")]
    pub privacy: Option<i32>,
    /// If true, this component is disabled.
    #[serde(rename = "disabled", skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
}

impl DictionaryComponentResponseOfint64AndDestinyItemSocketsComponent {
    pub fn new() -> DictionaryComponentResponseOfint64AndDestinyItemSocketsComponent {
        DictionaryComponentResponseOfint64AndDestinyItemSocketsComponent {
            data: None,
            privacy: None,
            disabled: None,
        }
    }
}


