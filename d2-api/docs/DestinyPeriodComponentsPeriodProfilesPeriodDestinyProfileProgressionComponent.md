# DestinyPeriodComponentsPeriodProfilesPeriodDestinyProfileProgressionComponent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**checklists** | Option<[**::std::collections::HashMap<String, ::std::collections::HashMap<String, bool>>**](map.md)> | The set of checklists that can be examined on a profile-wide basis, keyed by the hash identifier of the Checklist (DestinyChecklistDefinition)  For each checklist returned, its value is itself a Dictionary keyed by the checklist's hash identifier with the value being a boolean indicating if it's been discovered yet. | [optional]
**seasonal_artifact** | Option<[**crate::models::DestinyComponentsProfilesDestinyProfileProgressionComponentSeasonalArtifact**](Destiny_Components_Profiles_DestinyProfileProgressionComponent_seasonalArtifact.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


