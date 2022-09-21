# DestinyPeriodEntitiesPeriodItemsPeriodDestinyItemTalentGridComponent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**talent_grid_hash** | Option<**i32**> | Most items don't have useful talent grids anymore, but Builds in particular still do.  You can use this hash to lookup the DestinyTalentGridDefinition attached to this item, which will be crucial for understanding the node values on the item. | [optional]
**nodes** | Option<[**Vec<crate::models::DestinyPeriodDestinyTalentNode>**](Destiny.DestinyTalentNode.md)> | Detailed information about the individual nodes in the talent grid.  A node represents a single visual \"pip\" in the talent grid or Build detail view, though each node may have multiple \"steps\" which indicate the actual bonuses and visual representation of that node. | [optional]
**is_grid_complete** | Option<**bool**> | Indicates whether the talent grid on this item is completed, and thus whether it should have a gold border around it.  Only will be true if the item actually *has* a talent grid, and only then if it is completed (i.e. every exclusive set has an activated node, and every non-exclusive set node has been activated) | [optional]
**grid_progression** | Option<[**crate::models::DestinyEntitiesItemsDestinyItemTalentGridComponentGridProgression**](Destiny_Entities_Items_DestinyItemTalentGridComponent_gridProgression.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


