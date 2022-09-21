# DestinyPeriodDefinitionsPeriodDestinyStatDisplayDefinition

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**stat_hash** | Option<**i32**> | The hash identifier for the stat being transformed into a Display stat.  Use it to look up the DestinyStatDefinition, or key into a DestinyInventoryItemDefinition's stats property. | [optional]
**maximum_value** | Option<**i32**> | Regardless of the output of interpolation, this is the maximum possible value that the stat can be. It should also be used as the upper bound for displaying the stat as a progress bar (the minimum always being 0) | [optional]
**display_as_numeric** | Option<**bool**> | If this is true, the stat should be displayed as a number. Otherwise, display it as a progress bar. Or, you know, do whatever you want. There's no displayAsNumeric police. | [optional]
**display_interpolation** | Option<[**Vec<crate::models::InterpolationPeriodInterpolationPoint>**](Interpolation.InterpolationPoint.md)> | The interpolation table representing how the Investment Stat is transformed into a Display Stat.   See DestinyStatDefinition for a description of the stages of stat transformation. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


