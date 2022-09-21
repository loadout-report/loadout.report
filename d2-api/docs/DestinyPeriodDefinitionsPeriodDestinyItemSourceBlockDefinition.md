# DestinyPeriodDefinitionsPeriodDestinyItemSourceBlockDefinition

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**source_hashes** | Option<**Vec<i32>**> | The list of hash identifiers for Reward Sources that hint where the item can be found (DestinyRewardSourceDefinition). | [optional]
**sources** | Option<[**Vec<crate::models::DestinyPeriodDefinitionsPeriodSourcesPeriodDestinyItemSourceDefinition>**](Destiny.Definitions.Sources.DestinyItemSourceDefinition.md)> | A collection of details about the stats that were computed for the ways we found that the item could be spawned. | [optional]
**exclusive** | Option<**i32**> | If we found that this item is exclusive to a specific platform, this will be set to the BungieMembershipType enumeration that matches that platform. | [optional]
**vendor_sources** | Option<[**Vec<crate::models::DestinyPeriodDefinitionsPeriodDestinyItemVendorSourceReference>**](Destiny.Definitions.DestinyItemVendorSourceReference.md)> | A denormalized reference back to vendors that potentially sell this item. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


