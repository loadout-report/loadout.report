# DestinyPeriodEntitiesPeriodItemsPeriodDestinyItemObjectivesComponent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**objectives** | Option<[**Vec<crate::models::DestinyPeriodQuestsPeriodDestinyObjectiveProgress>**](Destiny.Quests.DestinyObjectiveProgress.md)> | If the item has a hard association with objectives, your progress on them will be defined here.   Objectives are our standard way to describe a series of tasks that have to be completed for a reward. | [optional]
**flavor_objective** | Option<[**crate::models::DestinyComponentsKiosksDestinyKioskItemFlavorObjective**](Destiny_Components_Kiosks_DestinyKioskItem_flavorObjective.md)> |  | [optional]
**date_completed** | Option<**String**> | If we have any information on when these objectives were completed, this will be the date of that completion. This won't be on many items, but could be interesting for some items that do store this information. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


