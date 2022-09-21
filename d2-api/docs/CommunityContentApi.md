# \CommunityContentApi

All URIs are relative to *https://www.bungie.net/Platform*

Method | HTTP request | Description
------------- | ------------- | -------------
[**community_content_period_get_community_content**](CommunityContentApi.md#community_content_period_get_community_content) | **GET** /CommunityContent/Get/{sort}/{mediaFilter}/{page}/ | 



## community_content_period_get_community_content

> crate::models::ForumGetTopicsPaged200Response community_content_period_get_community_content(media_filter, page, sort)


Returns community content.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**media_filter** | **i32** | The type of media to get | [required] |
**page** | **i32** | Zero based page | [required] |
**sort** | **i32** | The sort mode. | [required] |

### Return type

[**crate::models::ForumGetTopicsPaged200Response**](Forum_GetTopicsPaged_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

