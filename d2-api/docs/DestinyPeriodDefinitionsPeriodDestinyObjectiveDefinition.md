# DestinyPeriodDefinitionsPeriodDestinyObjectiveDefinition

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**display_properties** | Option<[**crate::models::DestinyDefinitionsDestinyObjectiveDefinitionDisplayProperties**](Destiny_Definitions_DestinyObjectiveDefinition_displayProperties.md)> |  | [optional]
**completion_value** | Option<**i32**> | The value that the unlock value defined in unlockValueHash must reach in order for the objective to be considered Completed. Used in calculating progress and completion status. | [optional]
**scope** | Option<**i32**> | A shortcut for determining the most restrictive gating that this Objective is set to use. This includes both the dynamic determination of progress and of completion values. See the DestinyGatingScope enum's documentation for more details. | [optional]
**location_hash** | Option<**i32**> | OPTIONAL: a hash identifier for the location at which this objective must be accomplished, if there is a location defined. Look up the DestinyLocationDefinition for this hash for that additional location info. | [optional]
**allow_negative_value** | Option<**bool**> | If true, the value is allowed to go negative. | [optional]
**allow_value_change_when_completed** | Option<**bool**> | If true, you can effectively \"un-complete\" this objective if you lose progress after crossing the completion threshold.   If False, once you complete the task it will remain completed forever by locking the value. | [optional]
**is_counting_downward** | Option<**bool**> | If true, completion means having an unlock value less than or equal to the completionValue.  If False, completion means having an unlock value greater than or equal to the completionValue. | [optional]
**value_style** | Option<**i32**> | The UI style applied to the objective. It's an enum, take a look at DestinyUnlockValueUIStyle for details of the possible styles. Use this info as you wish to customize your UI.  DEPRECATED: This is no longer populated by Destiny 2 game content. Please use inProgressValueStyle and completedValueStyle instead. | [optional]
**progress_description** | Option<**String**> | Text to describe the progress bar. | [optional]
**perks** | Option<[**crate::models::DestinyDefinitionsDestinyObjectiveDefinitionPerks**](Destiny_Definitions_DestinyObjectiveDefinition_perks.md)> |  | [optional]
**stats** | Option<[**crate::models::DestinyDefinitionsDestinyObjectiveDefinitionStats**](Destiny_Definitions_DestinyObjectiveDefinition_stats.md)> |  | [optional]
**minimum_visibility_threshold** | Option<**i32**> | If nonzero, this is the minimum value at which the objective's progression should be shown. Otherwise, don't show it yet. | [optional]
**allow_overcompletion** | Option<**bool**> | If True, the progress will continue even beyond the point where the objective met its minimum completion requirements. Your UI will have to accommodate it. | [optional]
**show_value_on_complete** | Option<**bool**> | If True, you should continue showing the progression value in the UI after it's complete. I mean, we already do that in BNet anyways, but if you want to be better behaved than us you could honor this flag. | [optional]
**completed_value_style** | Option<**i32**> | The style to use when the objective is completed. | [optional]
**in_progress_value_style** | Option<**i32**> | The style to use when the objective is still in progress. | [optional]
**ui_label** | Option<**String**> | Objectives can have arbitrary UI-defined identifiers that define the style applied to objectives. For convenience, known UI labels will be defined in the uiStyle enum value. | [optional]
**ui_style** | Option<**i32**> | If the objective has a known UI label value, this property will represent it. | [optional]
**hash** | Option<**i32**> | The unique identifier for this entity. Guaranteed to be unique for the type of entity, but not globally.  When entities refer to each other in Destiny content, it is this hash that they are referring to. | [optional]
**index** | Option<**i32**> | The index of the entity as it was found in the investment tables. | [optional]
**redacted** | Option<**bool**> | If this is true, then there is an entity with this identifier/type combination, but BNet is not yet allowed to show it. Sorry! | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


