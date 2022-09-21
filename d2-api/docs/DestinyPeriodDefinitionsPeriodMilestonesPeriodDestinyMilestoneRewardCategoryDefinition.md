# DestinyPeriodDefinitionsPeriodMilestonesPeriodDestinyMilestoneRewardCategoryDefinition

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**category_hash** | Option<**i32**> | Identifies the reward category. Only guaranteed unique within this specific component! | [optional]
**category_identifier** | Option<**String**> | The string identifier for the category, if you want to use it for some end. Guaranteed unique within the specific component. | [optional]
**display_properties** | Option<[**crate::models::DestinyDefinitionsMilestonesDestinyMilestoneRewardCategoryDefinitionDisplayProperties**](Destiny_Definitions_Milestones_DestinyMilestoneRewardCategoryDefinition_displayProperties.md)> |  | [optional]
**reward_entries** | Option<[**::std::collections::HashMap<String, crate::models::DestinyPeriodDefinitionsPeriodMilestonesPeriodDestinyMilestoneRewardEntryDefinition>**](Destiny.Definitions.Milestones.DestinyMilestoneRewardEntryDefinition.md)> | If this milestone can provide rewards, this will define the sets of rewards that can be earned, the conditions under which they can be acquired, internal data that we'll use at runtime to determine whether you've already earned or redeemed this set of rewards, and the category that this reward should be placed under. | [optional]
**order** | Option<**i32**> | If you want to use BNet's recommended order for rendering categories programmatically, use this value and compare it to other categories to determine the order in which they should be rendered. I don't feel great about putting this here, I won't lie. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


