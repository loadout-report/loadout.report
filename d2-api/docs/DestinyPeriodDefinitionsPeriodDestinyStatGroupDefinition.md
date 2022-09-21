# DestinyPeriodDefinitionsPeriodDestinyStatGroupDefinition

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**maximum_value** | Option<**i32**> | The maximum possible value that any stat in this group can be transformed into.  This is used by stats that *don't* have scaledStats entries below, but that still need to be displayed as a progress bar, in which case this is used as the upper bound for said progress bar. (the lower bound is always 0) | [optional]
**ui_position** | Option<**i32**> | This apparently indicates the position of the stats in the UI? I've returned it in case anyone can use it, but it's not of any use to us on BNet. Something's being lost in translation with this value. | [optional]
**scaled_stats** | Option<[**Vec<crate::models::DestinyPeriodDefinitionsPeriodDestinyStatDisplayDefinition>**](Destiny.Definitions.DestinyStatDisplayDefinition.md)> | Any stat that requires scaling to be transformed from an \"Investment\" stat to a \"Display\" stat will have an entry in this list. For more information on what those types of stats mean and the transformation process, see DestinyStatDefinition.  In retrospect, I wouldn't mind if this was a dictionary keyed by the stat hash instead. But I'm going to leave it be because [[After Apple Picking]]. | [optional]
**overrides** | Option<[**::std::collections::HashMap<String, crate::models::DestinyPeriodDefinitionsPeriodDestinyStatOverrideDefinition>**](Destiny.Definitions.DestinyStatOverrideDefinition.md)> | The game has the ability to override, based on the stat group, what the localized text is that is displayed for Stats being shown on the item.  Mercifully, no Stat Groups use this feature currently. If they start using them, we'll all need to start using them (and those of you who are more prudent than I am can go ahead and start pre-checking for this.) | [optional]
**hash** | Option<**i32**> | The unique identifier for this entity. Guaranteed to be unique for the type of entity, but not globally.  When entities refer to each other in Destiny content, it is this hash that they are referring to. | [optional]
**index** | Option<**i32**> | The index of the entity as it was found in the investment tables. | [optional]
**redacted** | Option<**bool**> | If this is true, then there is an entity with this identifier/type combination, but BNet is not yet allowed to show it. Sorry! | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


