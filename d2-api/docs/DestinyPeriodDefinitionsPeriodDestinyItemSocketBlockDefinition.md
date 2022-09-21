# DestinyPeriodDefinitionsPeriodDestinyItemSocketBlockDefinition

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**detail** | Option<**String**> | This was supposed to be a string that would give per-item details about sockets. In practice, it turns out that all this ever has is the localized word \"details\". ... that's lame, but perhaps it will become something cool in the future. | [optional]
**socket_entries** | Option<[**Vec<crate::models::DestinyPeriodDefinitionsPeriodDestinyItemSocketEntryDefinition>**](Destiny.Definitions.DestinyItemSocketEntryDefinition.md)> | Each non-intrinsic (or mutable) socket on an item is defined here. Check inside for more info. | [optional]
**intrinsic_sockets** | Option<[**Vec<crate::models::DestinyPeriodDefinitionsPeriodDestinyItemIntrinsicSocketEntryDefinition>**](Destiny.Definitions.DestinyItemIntrinsicSocketEntryDefinition.md)> | Each intrinsic (or immutable/permanent) socket on an item is defined here, along with the plug that is permanently affixed to the socket. | [optional]
**socket_categories** | Option<[**Vec<crate::models::DestinyPeriodDefinitionsPeriodDestinyItemSocketCategoryDefinition>**](Destiny.Definitions.DestinyItemSocketCategoryDefinition.md)> | A convenience property, that refers to the sockets in the \"sockets\" property, pre-grouped by category and ordered in the manner that they should be grouped in the UI. You could form this yourself with the existing data, but why would you want to? Enjoy life man. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


