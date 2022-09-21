# \AppApi

All URIs are relative to *https://www.bungie.net/Platform*

Method | HTTP request | Description
------------- | ------------- | -------------
[**app_period_get_application_api_usage**](AppApi.md#app_period_get_application_api_usage) | **GET** /App/ApiUsage/{applicationId}/ | 
[**app_period_get_bungie_applications**](AppApi.md#app_period_get_bungie_applications) | **GET** /App/FirstParty/ | 



## app_period_get_application_api_usage

> crate::models::AppGetApplicationApiUsage200Response app_period_get_application_api_usage(application_id, end, start)


Get API usage by application for time frame specified. You can go as far back as 30 days ago, and can ask for up to a 48 hour window of time in a single request. You must be authenticated with at least the ReadUserData permission to access this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_id** | **i32** | ID of the application to get usage statistics. | [required] |
**end** | Option<**String**> | End time for query. Goes to now if not specified. |  |
**start** | Option<**String**> | Start time for query. Goes to 24 hours ago if not specified. |  |

### Return type

[**crate::models::AppGetApplicationApiUsage200Response**](App_GetApplicationApiUsage_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_period_get_bungie_applications

> crate::models::AppGetBungieApplications200Response app_period_get_bungie_applications()


Get list of applications created by Bungie.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::AppGetBungieApplications200Response**](App_GetBungieApplications_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

