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
pub struct DictionaryComponentResponseOfuint32AndDestinyItemReusablePlugsComponent {
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<::std::collections::HashMap<String, crate::models::DestinyPeriodComponentsPeriodItemsPeriodDestinyItemReusablePlugsComponent>>,
    #[serde(rename = "privacy", skip_serializing_if = "Option::is_none")]
    pub privacy: Option<i32>,
    /// If true, this component is disabled.
    #[serde(rename = "disabled", skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
}

impl DictionaryComponentResponseOfuint32AndDestinyItemReusablePlugsComponent {
    pub fn new() -> DictionaryComponentResponseOfuint32AndDestinyItemReusablePlugsComponent {
        DictionaryComponentResponseOfuint32AndDestinyItemReusablePlugsComponent {
            data: None,
            privacy: None,
            disabled: None,
        }
    }
}


