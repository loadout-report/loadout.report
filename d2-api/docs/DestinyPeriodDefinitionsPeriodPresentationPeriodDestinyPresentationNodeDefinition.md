# DestinyPeriodDefinitionsPeriodPresentationPeriodDestinyPresentationNodeDefinition

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**display_properties** | Option<[**crate::models::DestinyPeriodDefinitionsPeriodCommonPeriodDestinyDisplayPropertiesDefinition**](Destiny.Definitions.Common.DestinyDisplayPropertiesDefinition.md)> |  | [optional]
**original_icon** | Option<**String**> | The original icon for this presentation node, before we futzed with it. | [optional]
**root_view_icon** | Option<**String**> | Some presentation nodes are meant to be explicitly shown on the \"root\" or \"entry\" screens for the feature to which they are related. You should use this icon when showing them on such a view, if you have a similar \"entry point\" view in your UI. If you don't have a UI, then I guess it doesn't matter either way does it? | [optional]
**node_type** | Option<**i32**> |  | [optional]
**scope** | Option<**i32**> | Indicates whether this presentation node's state is determined on a per-character or on an account-wide basis. | [optional]
**objective_hash** | Option<**i32**> | If this presentation node shows a related objective (for instance, if it tracks the progress of its children), the objective being tracked is indicated here. | [optional]
**completion_record_hash** | Option<**i32**> | If this presentation node has an associated \"Record\" that you can accomplish for completing its children, this is the identifier of that Record. | [optional]
**children** | Option<[**crate::models::DestinyDefinitionsPresentationDestinyPresentationNodeDefinitionChildren**](Destiny_Definitions_Presentation_DestinyPresentationNodeDefinition_children.md)> |  | [optional]
**display_style** | Option<**i32**> | A hint for how to display this presentation node when it's shown in a list. | [optional]
**screen_style** | Option<**i32**> | A hint for how to display this presentation node when it's shown in its own detail screen. | [optional]
**requirements** | Option<[**crate::models::DestinyDefinitionsPresentationDestinyPresentationNodeDefinitionRequirements**](Destiny_Definitions_Presentation_DestinyPresentationNodeDefinition_requirements.md)> |  | [optional]
**disable_child_subscreen_navigation** | Option<**bool**> | If this presentation node has children, but the game doesn't let you inspect the details of those children, that is indicated here. | [optional]
**max_category_record_score** | Option<**i32**> |  | [optional]
**presentation_node_type** | Option<**i32**> |  | [optional]
**trait_ids** | Option<**Vec<String>**> |  | [optional]
**trait_hashes** | Option<**Vec<i32>**> |  | [optional]
**parent_node_hashes** | Option<**Vec<i32>**> | A quick reference to presentation nodes that have this node as a child. Presentation nodes can be parented under multiple parents. | [optional]
**hash** | Option<**i32**> | The unique identifier for this entity. Guaranteed to be unique for the type of entity, but not globally.  When entities refer to each other in Destiny content, it is this hash that they are referring to. | [optional]
**index** | Option<**i32**> | The index of the entity as it was found in the investment tables. | [optional]
**redacted** | Option<**bool**> | If this is true, then there is an entity with this identifier/type combination, but BNet is not yet allowed to show it. Sorry! | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


