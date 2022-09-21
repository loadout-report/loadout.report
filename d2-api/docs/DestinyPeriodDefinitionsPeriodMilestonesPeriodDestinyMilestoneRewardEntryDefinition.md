# DestinyPeriodDefinitionsPeriodMilestonesPeriodDestinyMilestoneRewardEntryDefinition

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**reward_entry_hash** | Option<**i32**> | The identifier for this reward entry. Runtime data will refer to reward entries by this hash. Only guaranteed unique within the specific Milestone. | [optional]
**reward_entry_identifier** | Option<**String**> | The string identifier, if you care about it. Only guaranteed unique within the specific Milestone. | [optional]
**items** | Option<[**Vec<crate::models::DestinyPeriodDestinyItemQuantity>**](Destiny.DestinyItemQuantity.md)> | The items you will get as rewards, and how much of it you'll get. | [optional]
**vendor_hash** | Option<**i32**> | If this reward is redeemed at a Vendor, this is the hash of the Vendor to go to in order to redeem the reward. Use this hash to look up the DestinyVendorDefinition. | [optional]
**display_properties** | Option<[**crate::models::DestinyDefinitionsMilestonesDestinyMilestoneRewardEntryDefinitionDisplayProperties**](Destiny_Definitions_Milestones_DestinyMilestoneRewardEntryDefinition_displayProperties.md)> |  | [optional]
**order** | Option<**i32**> | If you want to follow BNet's ordering of these rewards, use this number within a given category to order the rewards. Yeah, I know. I feel dirty too. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


