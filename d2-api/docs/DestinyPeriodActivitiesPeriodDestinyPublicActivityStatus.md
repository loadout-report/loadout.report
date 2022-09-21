# DestinyPeriodActivitiesPeriodDestinyPublicActivityStatus

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**challenge_objective_hashes** | Option<**Vec<i32>**> | Active Challenges for the activity, if any - represented as hashes for DestinyObjectiveDefinitions. | [optional]
**modifier_hashes** | Option<**Vec<i32>**> | The active modifiers on this activity, if any - represented as hashes for DestinyActivityModifierDefinitions. | [optional]
**reward_tooltip_items** | Option<[**Vec<crate::models::DestinyPeriodDestinyItemQuantity>**](Destiny.DestinyItemQuantity.md)> | If the activity itself provides any specific \"mock\" rewards, this will be the items and their quantity.  Why \"mock\", you ask? Because these are the rewards as they are represented in the tooltip of the Activity.  These are often pointers to fake items that look good in a tooltip, but represent an abstract concept of what you will get for a reward rather than the specific items you may obtain. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


