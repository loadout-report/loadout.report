# \DefaultApi

All URIs are relative to *https://www.bungie.net/Platform*

Method | HTTP request | Description
------------- | ------------- | -------------
[**period_get_available_locales**](DefaultApi.md#period_get_available_locales) | **GET** /GetAvailableLocales/ | 
[**period_get_common_settings**](DefaultApi.md#period_get_common_settings) | **GET** /Settings/ | 
[**period_get_global_alerts**](DefaultApi.md#period_get_global_alerts) | **GET** /GlobalAlerts/ | 
[**period_get_user_system_overrides**](DefaultApi.md#period_get_user_system_overrides) | **GET** /UserSystemOverrides/ | 



## period_get_available_locales

> crate::models::UserGetSanitizedPlatformDisplayNames200Response period_get_available_locales()


List of available localization cultures

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::UserGetSanitizedPlatformDisplayNames200Response**](User_GetSanitizedPlatformDisplayNames_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## period_get_common_settings

> crate::models::GetCommonSettings200Response period_get_common_settings()


Get the common settings used by the Bungie.Net environment.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GetCommonSettings200Response**](_GetCommonSettings_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## period_get_global_alerts

> crate::models::GetGlobalAlerts200Response period_get_global_alerts(includestreaming)


Gets any active global alert for display in the forum banners, help pages, etc. Usually used for DOC alerts.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**includestreaming** | Option<**bool**> | Determines whether Streaming Alerts are included in results |  |

### Return type

[**crate::models::GetGlobalAlerts200Response**](_GetGlobalAlerts_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## period_get_user_system_overrides

> crate::models::GetUserSystemOverrides200Response period_get_user_system_overrides()


Get the user-specific system overrides that should be respected alongside common systems.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GetUserSystemOverrides200Response**](_GetUserSystemOverrides_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

