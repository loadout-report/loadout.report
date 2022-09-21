# DestinyPeriodDefinitionsPeriodDestinyProgressionStepDefinition

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**step_name** | Option<**String**> | Very rarely, Progressions will have localized text describing the Level of the progression. This will be that localized text, if it exists. Otherwise, the standard appears to be to simply show the level numerically. | [optional]
**display_effect_type** | Option<**i32**> | This appears to be, when you \"level up\", whether a visual effect will display and on what entity. See DestinyProgressionStepDisplayEffect for slightly more info. | [optional]
**progress_total** | Option<**i32**> | The total amount of progression points/\"experience\" you will need to initially reach this step. If this is the last step and the progression is repeating indefinitely (DestinyProgressionDefinition.repeatLastStep), this will also be the progress needed to level it up further by repeating this step again. | [optional]
**reward_items** | Option<[**Vec<crate::models::DestinyPeriodDestinyItemQuantity>**](Destiny.DestinyItemQuantity.md)> | A listing of items rewarded as a result of reaching this level. | [optional]
**icon** | Option<**String**> | If this progression step has a specific icon related to it, this is the icon to show. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


