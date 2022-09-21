# DestinyPeriodDefinitionsPeriodChecklistsPeriodDestinyChecklistEntryDefinition

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**hash** | Option<**i32**> | The identifier for this Checklist entry. Guaranteed unique only within this Checklist Definition, and not globally/for all checklists. | [optional]
**display_properties** | Option<[**crate::models::DestinyDefinitionsChecklistsDestinyChecklistEntryDefinitionDisplayProperties**](Destiny_Definitions_Checklists_DestinyChecklistEntryDefinition_displayProperties.md)> |  | [optional]
**destination_hash** | Option<**i32**> |  | [optional]
**location_hash** | Option<**i32**> |  | [optional]
**bubble_hash** | Option<**i32**> | Note that a Bubble's hash doesn't uniquely identify a \"top level\" entity in Destiny. Only the combination of location and bubble can uniquely identify a place in the world of Destiny: so if bubbleHash is populated, locationHash must too be populated for it to have any meaning.  You can use this property if it is populated to look up the DestinyLocationDefinition's associated .locationReleases[].activityBubbleName property. | [optional]
**activity_hash** | Option<**i32**> |  | [optional]
**item_hash** | Option<**i32**> |  | [optional]
**vendor_hash** | Option<**i32**> |  | [optional]
**vendor_interaction_index** | Option<**i32**> |  | [optional]
**scope** | Option<**i32**> | The scope at which this specific entry can be computed. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


