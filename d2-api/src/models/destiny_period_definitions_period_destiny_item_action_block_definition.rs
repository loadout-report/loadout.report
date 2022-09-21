/*
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * The version of the OpenAPI document: 2.16.0
 * Contact: support@bungie.com
 * Generated by: https://openapi-generator.tech
 */

/// DestinyPeriodDefinitionsPeriodDestinyItemActionBlockDefinition : If an item can have an action performed on it (like \"Dismantle\"), it will be defined here if you care.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DestinyPeriodDefinitionsPeriodDestinyItemActionBlockDefinition {
    /// Localized text for the verb of the action being performed.
    #[serde(rename = "verbName", skip_serializing_if = "Option::is_none")]
    pub verb_name: Option<String>,
    /// Localized text describing the action being performed.
    #[serde(rename = "verbDescription", skip_serializing_if = "Option::is_none")]
    pub verb_description: Option<String>,
    /// The content has this property, however it's not entirely clear how it is used.
    #[serde(rename = "isPositive", skip_serializing_if = "Option::is_none")]
    pub is_positive: Option<bool>,
    /// If the action has an overlay screen associated with it, this is the name of that screen. Unfortunately, we cannot return the screen's data itself.
    #[serde(rename = "overlayScreenName", skip_serializing_if = "Option::is_none")]
    pub overlay_screen_name: Option<String>,
    /// The icon associated with the overlay screen for the action, if any.
    #[serde(rename = "overlayIcon", skip_serializing_if = "Option::is_none")]
    pub overlay_icon: Option<String>,
    /// The number of seconds to delay before allowing this action to be performed again.
    #[serde(rename = "requiredCooldownSeconds", skip_serializing_if = "Option::is_none")]
    pub required_cooldown_seconds: Option<i32>,
    /// If the action requires other items to exist or be destroyed, this is the list of those items and requirements.
    #[serde(rename = "requiredItems", skip_serializing_if = "Option::is_none")]
    pub required_items: Option<Vec<crate::models::DestinyPeriodDefinitionsPeriodDestinyItemActionRequiredItemDefinition>>,
    /// If performing this action earns you Progression, this is the list of progressions and values granted for those progressions by performing this action.
    #[serde(rename = "progressionRewards", skip_serializing_if = "Option::is_none")]
    pub progression_rewards: Option<Vec<crate::models::DestinyPeriodDefinitionsPeriodDestinyProgressionRewardDefinition>>,
    /// The internal identifier for the action.
    #[serde(rename = "actionTypeLabel", skip_serializing_if = "Option::is_none")]
    pub action_type_label: Option<String>,
    /// Theoretically, an item could have a localized string for a hint about the location in which the action should be performed. In practice, no items yet have this property.
    #[serde(rename = "requiredLocation", skip_serializing_if = "Option::is_none")]
    pub required_location: Option<String>,
    /// The identifier hash for the Cooldown associated with this action. We have not pulled this data yet for you to have more data to use for cooldowns.
    #[serde(rename = "requiredCooldownHash", skip_serializing_if = "Option::is_none")]
    pub required_cooldown_hash: Option<i32>,
    /// If true, the item is deleted when the action completes.
    #[serde(rename = "deleteOnAction", skip_serializing_if = "Option::is_none")]
    pub delete_on_action: Option<bool>,
    /// If true, the entire stack is deleted when the action completes.
    #[serde(rename = "consumeEntireStack", skip_serializing_if = "Option::is_none")]
    pub consume_entire_stack: Option<bool>,
    /// If true, this action will be performed as soon as you earn this item. Some rewards work this way, providing you a single item to pick up from a reward-granting vendor in-game and then immediately consuming itself to provide you multiple items.
    #[serde(rename = "useOnAcquire", skip_serializing_if = "Option::is_none")]
    pub use_on_acquire: Option<bool>,
}

impl DestinyPeriodDefinitionsPeriodDestinyItemActionBlockDefinition {
    /// If an item can have an action performed on it (like \"Dismantle\"), it will be defined here if you care.
    pub fn new() -> DestinyPeriodDefinitionsPeriodDestinyItemActionBlockDefinition {
        DestinyPeriodDefinitionsPeriodDestinyItemActionBlockDefinition {
            verb_name: None,
            verb_description: None,
            is_positive: None,
            overlay_screen_name: None,
            overlay_icon: None,
            required_cooldown_seconds: None,
            required_items: None,
            progression_rewards: None,
            action_type_label: None,
            required_location: None,
            required_cooldown_hash: None,
            delete_on_action: None,
            consume_entire_stack: None,
            use_on_acquire: None,
        }
    }
}


