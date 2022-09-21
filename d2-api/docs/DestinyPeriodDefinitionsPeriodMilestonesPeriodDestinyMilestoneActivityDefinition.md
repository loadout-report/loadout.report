# DestinyPeriodDefinitionsPeriodMilestonesPeriodDestinyMilestoneActivityDefinition

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**conceptual_activity_hash** | Option<**i32**> | The \"Conceptual\" activity hash. Basically, we picked the lowest level activity and are treating it as the canonical definition of the activity for rendering purposes.  If you care about the specific difficulty modes and variations, use the activities under \"Variants\". | [optional]
**variants** | Option<[**::std::collections::HashMap<String, crate::models::DestinyPeriodDefinitionsPeriodMilestonesPeriodDestinyMilestoneActivityVariantDefinition>**](Destiny.Definitions.Milestones.DestinyMilestoneActivityVariantDefinition.md)> | A milestone-referenced activity can have many variants, such as Tiers or alternative modes of play.  Even if there is only a single variant, the details for these are represented within as a variant definition.  It is assumed that, if this DestinyMilestoneActivityDefinition is active, then all variants should be active.  If a Milestone could ever split the variants' active status conditionally, they should all have their own DestinyMilestoneActivityDefinition instead! The potential duplication will be worth it for the obviousness of processing and use. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


