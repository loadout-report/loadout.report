# DestinyDefinitionsDestinyInventoryItemDefinitionAction

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**verb_name** | Option<**String**> | Localized text for the verb of the action being performed. | [optional]
**verb_description** | Option<**String**> | Localized text describing the action being performed. | [optional]
**is_positive** | Option<**bool**> | The content has this property, however it's not entirely clear how it is used. | [optional]
**overlay_screen_name** | Option<**String**> | If the action has an overlay screen associated with it, this is the name of that screen. Unfortunately, we cannot return the screen's data itself. | [optional]
**overlay_icon** | Option<**String**> | The icon associated with the overlay screen for the action, if any. | [optional]
**required_cooldown_seconds** | Option<**i32**> | The number of seconds to delay before allowing this action to be performed again. | [optional]
**required_items** | Option<[**Vec<crate::models::DestinyPeriodDefinitionsPeriodDestinyItemActionRequiredItemDefinition>**](Destiny.Definitions.DestinyItemActionRequiredItemDefinition.md)> | If the action requires other items to exist or be destroyed, this is the list of those items and requirements. | [optional]
**progression_rewards** | Option<[**Vec<crate::models::DestinyPeriodDefinitionsPeriodDestinyProgressionRewardDefinition>**](Destiny.Definitions.DestinyProgressionRewardDefinition.md)> | If performing this action earns you Progression, this is the list of progressions and values granted for those progressions by performing this action. | [optional]
**action_type_label** | Option<**String**> | The internal identifier for the action. | [optional]
**required_location** | Option<**String**> | Theoretically, an item could have a localized string for a hint about the location in which the action should be performed. In practice, no items yet have this property. | [optional]
**required_cooldown_hash** | Option<**i32**> | The identifier hash for the Cooldown associated with this action. We have not pulled this data yet for you to have more data to use for cooldowns. | [optional]
**delete_on_action** | Option<**bool**> | If true, the item is deleted when the action completes. | [optional]
**consume_entire_stack** | Option<**bool**> | If true, the entire stack is deleted when the action completes. | [optional]
**use_on_acquire** | Option<**bool**> | If true, this action will be performed as soon as you earn this item. Some rewards work this way, providing you a single item to pick up from a reward-granting vendor in-game and then immediately consuming itself to provide you multiple items. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


