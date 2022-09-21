# DestinyPeriodDefinitionsPeriodDirectorPeriodDestinyActivityGraphNodeDefinition

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**node_id** | Option<**i32**> | An identifier for the Activity Graph Node, only guaranteed to be unique within its parent Activity Graph. | [optional]
**override_display** | Option<[**crate::models::DestinyDefinitionsDirectorDestinyActivityGraphNodeDefinitionOverrideDisplay**](Destiny_Definitions_Director_DestinyActivityGraphNodeDefinition_overrideDisplay.md)> |  | [optional]
**position** | Option<[**crate::models::DestinyDefinitionsDirectorDestinyActivityGraphNodeDefinitionPosition**](Destiny_Definitions_Director_DestinyActivityGraphNodeDefinition_position.md)> |  | [optional]
**featuring_states** | Option<[**Vec<crate::models::DestinyPeriodDefinitionsPeriodDirectorPeriodDestinyActivityGraphNodeFeaturingStateDefinition>**](Destiny.Definitions.Director.DestinyActivityGraphNodeFeaturingStateDefinition.md)> | The node may have various visual accents placed on it, or styles applied. These are the list of possible styles that the Node can have. The game iterates through each, looking for the first one that passes a check of the required game/character/account state in order to show that style, and then renders the node in that style. | [optional]
**activities** | Option<[**Vec<crate::models::DestinyPeriodDefinitionsPeriodDirectorPeriodDestinyActivityGraphNodeActivityDefinition>**](Destiny.Definitions.Director.DestinyActivityGraphNodeActivityDefinition.md)> | The node may have various possible activities that could be active for it, however only one may be active at a time. See the DestinyActivityGraphNodeActivityDefinition for details. | [optional]
**states** | Option<[**Vec<crate::models::DestinyPeriodDefinitionsPeriodDirectorPeriodDestinyActivityGraphNodeStateEntry>**](Destiny.Definitions.Director.DestinyActivityGraphNodeStateEntry.md)> | Represents possible states that the graph node can be in. These are combined with some checking that happens in the game client and server to determine which state is actually active at any given time. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


