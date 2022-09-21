# FireteamPeriodFireteamUserInfoCard

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**fireteam_display_name** | Option<**String**> |  | [optional]
**fireteam_membership_type** | Option<**i32**> |  | [optional]
**supplemental_display_name** | Option<**String**> | A platform specific additional display name - ex: psn Real Name, bnet Unique Name, etc. | [optional]
**icon_path** | Option<**String**> | URL the Icon if available. | [optional]
**cross_save_override** | Option<**i32**> | If there is a cross save override in effect, this value will tell you the type that is overridding this one. | [optional]
**applicable_membership_types** | Option<**Vec<i32>**> | The list of Membership Types indicating the platforms on which this Membership can be used.   Not in Cross Save = its original membership type. Cross Save Primary = Any membership types it is overridding, and its original membership type Cross Save Overridden = Empty list | [optional]
**is_public** | Option<**bool**> | If True, this is a public user membership. | [optional]
**membership_type** | Option<**i32**> | Type of the membership. Not necessarily the native type. | [optional]
**membership_id** | Option<**i64**> | Membership ID as they user is known in the Accounts service | [optional]
**display_name** | Option<**String**> | Display Name the player has chosen for themselves. The display name is optional when the data type is used as input to a platform API. | [optional]
**bungie_global_display_name** | Option<**String**> | The bungie global display name, if set. | [optional]
**bungie_global_display_name_code** | Option<**i32**> | The bungie global display name code, if set. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


