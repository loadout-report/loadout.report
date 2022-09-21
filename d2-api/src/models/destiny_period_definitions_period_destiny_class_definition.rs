/*
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * The version of the OpenAPI document: 2.16.0
 * Contact: support@bungie.com
 * Generated by: https://openapi-generator.tech
 */

/// DestinyPeriodDefinitionsPeriodDestinyClassDefinition : Defines a Character Class in Destiny 2. These are types of characters you can play, like Titan, Warlock, and Hunter.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DestinyPeriodDefinitionsPeriodDestinyClassDefinition {
    /// In Destiny 1, we added a convenience Enumeration for referring to classes. We've kept it, though mostly for posterity. This is the enum value for this definition's class.
    #[serde(rename = "classType", skip_serializing_if = "Option::is_none")]
    pub class_type: Option<i32>,
    #[serde(rename = "displayProperties", skip_serializing_if = "Option::is_none")]
    pub display_properties: Option<Box<crate::models::DestinyPeriodDefinitionsPeriodCommonPeriodDestinyDisplayPropertiesDefinition>>,
    /// A localized string referring to the singular form of the Class's name when referred to in gendered form. Keyed by the DestinyGender.
    #[serde(rename = "genderedClassNames", skip_serializing_if = "Option::is_none")]
    pub gendered_class_names: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "genderedClassNamesByGenderHash", skip_serializing_if = "Option::is_none")]
    pub gendered_class_names_by_gender_hash: Option<::std::collections::HashMap<String, String>>,
    /// Mentors don't really mean anything anymore. Don't expect this to be populated.
    #[serde(rename = "mentorVendorHash", skip_serializing_if = "Option::is_none")]
    pub mentor_vendor_hash: Option<i32>,
    /// The unique identifier for this entity. Guaranteed to be unique for the type of entity, but not globally.  When entities refer to each other in Destiny content, it is this hash that they are referring to.
    #[serde(rename = "hash", skip_serializing_if = "Option::is_none")]
    pub hash: Option<i32>,
    /// The index of the entity as it was found in the investment tables.
    #[serde(rename = "index", skip_serializing_if = "Option::is_none")]
    pub index: Option<i32>,
    /// If this is true, then there is an entity with this identifier/type combination, but BNet is not yet allowed to show it. Sorry!
    #[serde(rename = "redacted", skip_serializing_if = "Option::is_none")]
    pub redacted: Option<bool>,
}

impl DestinyPeriodDefinitionsPeriodDestinyClassDefinition {
    /// Defines a Character Class in Destiny 2. These are types of characters you can play, like Titan, Warlock, and Hunter.
    pub fn new() -> DestinyPeriodDefinitionsPeriodDestinyClassDefinition {
        DestinyPeriodDefinitionsPeriodDestinyClassDefinition {
            class_type: None,
            display_properties: None,
            gendered_class_names: None,
            gendered_class_names_by_gender_hash: None,
            mentor_vendor_hash: None,
            hash: None,
            index: None,
            redacted: None,
        }
    }
}


