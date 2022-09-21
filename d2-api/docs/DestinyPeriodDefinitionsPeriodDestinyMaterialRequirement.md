# DestinyPeriodDefinitionsPeriodDestinyMaterialRequirement

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**item_hash** | Option<**i32**> | The hash identifier of the material required. Use it to look up the material's DestinyInventoryItemDefinition. | [optional]
**delete_on_action** | Option<**bool**> | If True, the material will be removed from the character's inventory when the action is performed. | [optional]
**count** | Option<**i32**> | The amount of the material required. | [optional]
**count_is_constant** | Option<**bool**> | If true, the material requirement count value is constant. Since The Witch Queen expansion, some material requirement counts can be dynamic and will need to be returned with an API call. | [optional]
**omit_from_requirements** | Option<**bool**> | If True, this requirement is \"silent\": don't bother showing it in a material requirements display. I mean, I'm not your mom: I'm not going to tell you you *can't* show it. But we won't show it in our UI. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


