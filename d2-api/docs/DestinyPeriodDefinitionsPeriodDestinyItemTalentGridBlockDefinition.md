# DestinyPeriodDefinitionsPeriodDestinyItemTalentGridBlockDefinition

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**talent_grid_hash** | Option<**i32**> | The hash identifier of the DestinyTalentGridDefinition attached to this item. | [optional]
**item_detail_string** | Option<**String**> | This is meant to be a subtitle for looking at the talent grid. In practice, somewhat frustratingly, this always merely says the localized word for \"Details\". Great. Maybe it'll have more if talent grids ever get used for more than builds and subclasses again. | [optional]
**build_name** | Option<**String**> | A shortcut string identifier for the \"build\" in question, if this talent grid has an associated build. Doesn't map to anything we can expose at the moment. | [optional]
**hud_damage_type** | Option<**i32**> | If the talent grid implies a damage type, this is the enum value for that damage type. | [optional]
**hud_icon** | Option<**String**> | If the talent grid has a special icon that's shown in the game UI (like builds, funny that), this is the identifier for that icon. Sadly, we don't actually get that icon right now. I'll be looking to replace this with a path to the actual icon itself. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


