# DestinyMilestonesDestinyMilestoneQuestStatus

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**quest_hash** | Option<**i32**> | The hash identifier for the Quest Item. (Note: Quests are defined as Items, and thus you would use this to look up the quest's DestinyInventoryItemDefinition). For information on all steps in the quest, you can then examine its DestinyInventoryItemDefinition.setData property for Quest Steps (which are *also* items). You can use the Item Definition to display human readable data about the overall quest. | [optional]
**step_hash** | Option<**i32**> | The hash identifier of the current Quest Step, which is also a DestinyInventoryItemDefinition. You can use this to get human readable data about the current step and what to do in that step. | [optional]
**step_objectives** | Option<[**Vec<crate::models::DestinyPeriodQuestsPeriodDestinyObjectiveProgress>**](Destiny.Quests.DestinyObjectiveProgress.md)> | A step can have multiple objectives. This will give you the progress for each objective in the current step, in the order in which they are rendered in-game. | [optional]
**tracked** | Option<**bool**> | Whether or not the quest is tracked | [optional]
**item_instance_id** | Option<**i64**> | The current Quest Step will be an instanced item in the player's inventory. If you care about that, this is the instance ID of that item. | [optional]
**completed** | Option<**bool**> | Whether or not the whole quest has been completed, regardless of whether or not you have redeemed the rewards for the quest. | [optional]
**redeemed** | Option<**bool**> | Whether or not you have redeemed rewards for this quest. | [optional]
**started** | Option<**bool**> | Whether or not you have started this quest. | [optional]
**vendor_hash** | Option<**i32**> | If the quest has a related Vendor that you should talk to in order to initiate the quest/earn rewards/continue the quest, this will be the hash identifier of that Vendor. Look it up its DestinyVendorDefinition. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


