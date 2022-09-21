# DestinyPeriodConstantsPeriodDestinyEnvironmentLocationMapping

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**location_hash** | Option<**i32**> | The location that is revealed on the director by this mapping. | [optional]
**activation_source** | Option<**String**> | A hint that the UI uses to figure out how this location is activated by the player. | [optional]
**item_hash** | Option<**i32**> | If this is populated, it is the item that you must possess for this location to be active because of this mapping. (theoretically, a location can have multiple mappings, and some might require an item while others don't) | [optional]
**objective_hash** | Option<**i32**> | If this is populated, this is an objective related to the location. | [optional]
**activity_hash** | Option<**i32**> | If this is populated, this is the activity you have to be playing in order to see this location appear because of this mapping. (theoretically, a location can have multiple mappings, and some might require you to be in a specific activity when others don't) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


