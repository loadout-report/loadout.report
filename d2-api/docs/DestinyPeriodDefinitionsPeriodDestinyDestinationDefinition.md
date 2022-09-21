# DestinyPeriodDefinitionsPeriodDestinyDestinationDefinition

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**display_properties** | Option<[**crate::models::DestinyPeriodDefinitionsPeriodCommonPeriodDestinyDisplayPropertiesDefinition**](Destiny.Definitions.Common.DestinyDisplayPropertiesDefinition.md)> |  | [optional]
**place_hash** | Option<**i32**> | The place that \"owns\" this Destination. Use this hash to look up the DestinyPlaceDefinition. | [optional]
**default_freeroam_activity_hash** | Option<**i32**> | If this Destination has a default Free-Roam activity, this is the hash for that Activity. Use it to look up the DestinyActivityDefintion. | [optional]
**activity_graph_entries** | Option<[**Vec<crate::models::DestinyPeriodDefinitionsPeriodDestinyActivityGraphListEntryDefinition>**](Destiny.Definitions.DestinyActivityGraphListEntryDefinition.md)> | If the Destination has default Activity Graphs (i.e. \"Map\") that should be shown in the director, this is the list of those Graphs. At most, only one should be active at any given time for a Destination: these would represent, for example, different variants on a Map if the Destination is changing on a macro level based on game state. | [optional]
**bubble_settings** | Option<[**Vec<crate::models::DestinyPeriodDefinitionsPeriodDestinyDestinationBubbleSettingDefinition>**](Destiny.Definitions.DestinyDestinationBubbleSettingDefinition.md)> | A Destination may have many \"Bubbles\" zones with human readable properties.  We don't get as much info as I'd like about them - I'd love to return info like where on the map they are located - but at least this gives you the name of those bubbles. bubbleSettings and bubbles both have the identical number of entries, and you should match up their indexes to provide matching bubble and bubbleSettings data.  DEPRECATED - Just use bubbles, it now has this data. | [optional]
**bubbles** | Option<[**Vec<crate::models::DestinyPeriodDefinitionsPeriodDestinyBubbleDefinition>**](Destiny.Definitions.DestinyBubbleDefinition.md)> | This provides the unique identifiers for every bubble in the destination (only guaranteed unique within the destination), and any intrinsic properties of the bubble.  bubbleSettings and bubbles both have the identical number of entries, and you should match up their indexes to provide matching bubble and bubbleSettings data. | [optional]
**hash** | Option<**i32**> | The unique identifier for this entity. Guaranteed to be unique for the type of entity, but not globally.  When entities refer to each other in Destiny content, it is this hash that they are referring to. | [optional]
**index** | Option<**i32**> | The index of the entity as it was found in the investment tables. | [optional]
**redacted** | Option<**bool**> | If this is true, then there is an entity with this identifier/type combination, but BNet is not yet allowed to show it. Sorry! | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


