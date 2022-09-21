# DestinyPeriodDestinyTalentNode

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**node_index** | Option<**i32**> | The index of the Talent Node being referred to (an index into DestinyTalentGridDefinition.nodes[]). CONTENT VERSION DEPENDENT. | [optional]
**node_hash** | Option<**i32**> | The hash of the Talent Node being referred to (in DestinyTalentGridDefinition.nodes). Deceptively CONTENT VERSION DEPENDENT. We have no guarantee of the hash's immutability between content versions. | [optional]
**state** | Option<**i32**> | An DestinyTalentNodeState enum value indicating the node's state: whether it can be activated or swapped, and why not if neither can be performed. | [optional]
**is_activated** | Option<**bool**> | If true, the node is activated: it's current step then provides its benefits. | [optional]
**step_index** | Option<**i32**> | The currently relevant Step for the node. It is this step that has rendering data for the node and the benefits that are provided if the node is activated. (the actual rules for benefits provided are extremely complicated in theory, but with how Talent Grids are being used in Destiny 2 you don't have to worry about a lot of those old Destiny 1 rules.) This is an index into: DestinyTalentGridDefinition.nodes[nodeIndex].steps[stepIndex] | [optional]
**materials_to_upgrade** | Option<[**Vec<crate::models::DestinyPeriodDefinitionsPeriodDestinyMaterialRequirement>**](Destiny.Definitions.DestinyMaterialRequirement.md)> | If the node has material requirements to be activated, this is the list of those requirements. | [optional]
**activation_grid_level** | Option<**i32**> | The progression level required on the Talent Grid in order to be able to activate this talent node. Talent Grids have their own Progression - similar to Character Level, but in this case it is experience related to the item itself. | [optional]
**progress_percent** | Option<**f32**> | If you want to show a progress bar or circle for how close this talent node is to being activate-able, this is the percentage to show. It follows the node's underlying rules about when the progress bar should first show up, and when it should be filled. | [optional]
**hidden** | Option<**bool**> | Whether or not the talent node is actually visible in the game's UI. Whether you want to show it in your own UI is up to you! I'm not gonna tell you who to sock it to. | [optional]
**node_stats_block** | Option<[**crate::models::DestinyDestinyTalentNodeNodeStatsBlock**](Destiny_DestinyTalentNode_nodeStatsBlock.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


