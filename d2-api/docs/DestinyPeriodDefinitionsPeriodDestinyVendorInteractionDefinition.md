# DestinyPeriodDefinitionsPeriodDestinyVendorInteractionDefinition

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**interaction_index** | Option<**i32**> | The position of this interaction in its parent array. Note that this is NOT content agnostic, and should not be used as such. | [optional]
**replies** | Option<[**Vec<crate::models::DestinyPeriodDefinitionsPeriodDestinyVendorInteractionReplyDefinition>**](Destiny.Definitions.DestinyVendorInteractionReplyDefinition.md)> | The potential replies that the user can make to the interaction. | [optional]
**vendor_category_index** | Option<**i32**> | If >= 0, this is the category of sale items to show along with this interaction dialog. | [optional]
**questline_item_hash** | Option<**i32**> | If this interaction dialog is about a quest, this is the questline related to the interaction. You can use this to show the quest overview, or even the character's status with the quest if you use it to find the character's current Quest Step by checking their inventory against this questlineItemHash's DestinyInventoryItemDefinition.setData. | [optional]
**sack_interaction_list** | Option<[**Vec<crate::models::DestinyPeriodDefinitionsPeriodDestinyVendorInteractionSackEntryDefinition>**](Destiny.Definitions.DestinyVendorInteractionSackEntryDefinition.md)> | If this interaction is meant to show you sacks, this is the list of types of sacks to be shown. If empty, the interaction is not meant to show sacks. | [optional]
**ui_interaction_type** | Option<**i32**> | A UI hint for the behavior of the interaction screen. This is useful to determine what type of interaction is occurring, such as a prompt to receive a rank up reward or a prompt to choose a reward for completing a quest. The hash isn't as useful as the Enum in retrospect, well what can you do. Try using interactionType instead. | [optional]
**interaction_type** | Option<**i32**> | The enumerated version of the possible UI hints for vendor interactions, which is a little easier to grok than the hash found in uiInteractionType. | [optional]
**reward_block_label** | Option<**String**> | If this interaction is displaying rewards, this is the text to use for the header of the reward-displaying section of the interaction. | [optional]
**reward_vendor_category_index** | Option<**i32**> | If the vendor's reward list is sourced from one of his categories, this is the index into the category array of items to show. | [optional]
**flavor_line_one** | Option<**String**> | If the vendor interaction has flavor text, this is some of it. | [optional]
**flavor_line_two** | Option<**String**> | If the vendor interaction has flavor text, this is the rest of it. | [optional]
**header_display_properties** | Option<[**crate::models::DestinyDefinitionsDestinyVendorInteractionDefinitionHeaderDisplayProperties**](Destiny_Definitions_DestinyVendorInteractionDefinition_headerDisplayProperties.md)> |  | [optional]
**instructions** | Option<**String**> | The localized text telling the player what to do when they see this dialog. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


