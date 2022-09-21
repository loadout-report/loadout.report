/*
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * The version of the OpenAPI document: 2.16.0
 * Contact: support@bungie.com
 * Generated by: https://openapi-generator.tech
 */

/// DestinyPeriodMilestonesPeriodDestinyMilestoneContentItemCategory : Part of our dynamic, localized Milestone content is arbitrary categories of items. These are built in our content management system, and thus aren't the same as programmatically generated rewards.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DestinyPeriodMilestonesPeriodDestinyMilestoneContentItemCategory {
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "itemHashes", skip_serializing_if = "Option::is_none")]
    pub item_hashes: Option<Vec<i32>>,
}

impl DestinyPeriodMilestonesPeriodDestinyMilestoneContentItemCategory {
    /// Part of our dynamic, localized Milestone content is arbitrary categories of items. These are built in our content management system, and thus aren't the same as programmatically generated rewards.
    pub fn new() -> DestinyPeriodMilestonesPeriodDestinyMilestoneContentItemCategory {
        DestinyPeriodMilestonesPeriodDestinyMilestoneContentItemCategory {
            title: None,
            item_hashes: None,
        }
    }
}


