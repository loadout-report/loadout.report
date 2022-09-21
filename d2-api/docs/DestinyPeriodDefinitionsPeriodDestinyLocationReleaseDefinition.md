# DestinyPeriodDefinitionsPeriodDestinyLocationReleaseDefinition

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**display_properties** | Option<[**crate::models::DestinyDefinitionsDestinyLocationReleaseDefinitionDisplayProperties**](Destiny_Definitions_DestinyLocationReleaseDefinition_displayProperties.md)> |  | [optional]
**small_transparent_icon** | Option<**String**> |  | [optional]
**map_icon** | Option<**String**> |  | [optional]
**large_transparent_icon** | Option<**String**> |  | [optional]
**spawn_point** | Option<**i32**> | If we had map information, this spawnPoint would be interesting. But sadly, we don't have that info. | [optional]
**destination_hash** | Option<**i32**> | The Destination being pointed to by this location. | [optional]
**activity_hash** | Option<**i32**> | The Activity being pointed to by this location. | [optional]
**activity_graph_hash** | Option<**i32**> | The Activity Graph being pointed to by this location. | [optional]
**activity_graph_node_hash** | Option<**i32**> | The Activity Graph Node being pointed to by this location. (Remember that Activity Graph Node hashes are only unique within an Activity Graph: so use the combination to find the node being spoken of) | [optional]
**activity_bubble_name** | Option<**i32**> | The Activity Bubble within the Destination. Look this up in the DestinyDestinationDefinition's bubbles and bubbleSettings properties. | [optional]
**activity_path_bundle** | Option<**i32**> | If we had map information, this would tell us something cool about the path this location wants you to take. I wish we had map information. | [optional]
**activity_path_destination** | Option<**i32**> | If we had map information, this would tell us about path information related to destination on the map. Sad. Maybe you can do something cool with it. Go to town man. | [optional]
**nav_point_type** | Option<**i32**> | The type of Nav Point that this represents. See the enumeration for more info. | [optional]
**world_position** | Option<**Vec<i32>**> | Looks like it should be the position on the map, but sadly it does not look populated... yet? | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


