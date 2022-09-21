# DestinyPeriodDefinitionsPeriodDestinyVendorItemSocketOverride

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**single_item_hash** | Option<**i32**> | If this is populated, the socket will be overridden with a specific plug.  If this isn't populated, it's being overridden by something more complicated that is only known by the Game Server and God, which means we can't tell you in advance what it'll be. | [optional]
**randomized_options_count** | Option<**i32**> | If this is greater than -1, the number of randomized plugs on this socket will be set to this quantity instead of whatever it's set to by default. | [optional]
**socket_type_hash** | Option<**i32**> | This appears to be used to select which socket ultimately gets the override defined here. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


