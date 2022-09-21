# DestinyPeriodResponsesPeriodDestinyLinkedProfilesResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**profiles** | Option<[**Vec<crate::models::DestinyPeriodResponsesPeriodDestinyProfileUserInfoCard>**](Destiny.Responses.DestinyProfileUserInfoCard.md)> | Any Destiny account for whom we could successfully pull characters will be returned here, as the Platform-level summary of user data. (no character data, no Destiny account data other than the Membership ID and Type so you can make further queries) | [optional]
**bnet_membership** | Option<[**crate::models::DestinyResponsesDestinyLinkedProfilesResponseBnetMembership**](Destiny_Responses_DestinyLinkedProfilesResponse_bnetMembership.md)> |  | [optional]
**profiles_with_errors** | Option<[**Vec<crate::models::DestinyPeriodResponsesPeriodDestinyErrorProfile>**](Destiny.Responses.DestinyErrorProfile.md)> | This is brief summary info for profiles that we believe have valid Destiny info, but who failed to return data for some other reason and thus we know that subsequent calls for their info will also fail. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


