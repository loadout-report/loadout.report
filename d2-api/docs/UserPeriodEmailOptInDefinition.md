# UserPeriodEmailOptInDefinition

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | The unique identifier for this opt-in category. | [optional]
**value** | Option<**i64**> | The flag value for this opt-in category. For historical reasons, this is defined as a flags enum. | [optional]
**set_by_default** | Option<**bool**> | If true, this opt-in setting should be set by default in situations where accounts are created without explicit choices about what they're opting into. | [optional]
**dependent_subscriptions** | Option<[**Vec<crate::models::UserPeriodEmailSubscriptionDefinition>**](User.EmailSubscriptionDefinition.md)> | Information about the dependent subscriptions for this opt-in. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


