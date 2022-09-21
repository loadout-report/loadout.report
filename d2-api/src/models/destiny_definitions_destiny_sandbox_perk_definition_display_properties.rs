/*
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * The version of the OpenAPI document: 2.16.0
 * Contact: support@bungie.com
 * Generated by: https://openapi-generator.tech
 */

/// DestinyDefinitionsDestinySandboxPerkDefinitionDisplayProperties : These display properties are by no means guaranteed to be populated. Usually when it is, it's only because we back-filled them with the displayProperties of some Talent Node or Plug item that happened to be uniquely providing that perk.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DestinyDefinitionsDestinySandboxPerkDefinitionDisplayProperties {
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Note that \"icon\" is sometimes misleading, and should be interpreted in the context of the entity. For instance, in Destiny 1 the DestinyRecordBookDefinition's icon was a big picture of a book.  But usually, it will be a small square image that you can use as... well, an icon.  They are currently represented as 96px x 96px images.
    #[serde(rename = "icon", skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(rename = "iconSequences", skip_serializing_if = "Option::is_none")]
    pub icon_sequences: Option<Vec<crate::models::DestinyPeriodDefinitionsPeriodCommonPeriodDestinyIconSequenceDefinition>>,
    /// If this item has a high-res icon (at least for now, many things won't), then the path to that icon will be here.
    #[serde(rename = "highResIcon", skip_serializing_if = "Option::is_none")]
    pub high_res_icon: Option<String>,
    #[serde(rename = "hasIcon", skip_serializing_if = "Option::is_none")]
    pub has_icon: Option<bool>,
}

impl DestinyDefinitionsDestinySandboxPerkDefinitionDisplayProperties {
    /// These display properties are by no means guaranteed to be populated. Usually when it is, it's only because we back-filled them with the displayProperties of some Talent Node or Plug item that happened to be uniquely providing that perk.
    pub fn new() -> DestinyDefinitionsDestinySandboxPerkDefinitionDisplayProperties {
        DestinyDefinitionsDestinySandboxPerkDefinitionDisplayProperties {
            description: None,
            name: None,
            icon: None,
            icon_sequences: None,
            high_res_icon: None,
            has_icon: None,
        }
    }
}


