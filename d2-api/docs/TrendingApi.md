# \TrendingApi

All URIs are relative to *https://www.bungie.net/Platform*

Method | HTTP request | Description
------------- | ------------- | -------------
[**trending_period_get_trending_categories**](TrendingApi.md#trending_period_get_trending_categories) | **GET** /Trending/Categories/ | 
[**trending_period_get_trending_category**](TrendingApi.md#trending_period_get_trending_category) | **GET** /Trending/Categories/{categoryId}/{pageNumber}/ | 
[**trending_period_get_trending_entry_detail**](TrendingApi.md#trending_period_get_trending_entry_detail) | **GET** /Trending/Details/{trendingEntryType}/{identifier}/ | 



## trending_period_get_trending_categories

> crate::models::TrendingGetTrendingCategories200Response trending_period_get_trending_categories()


Returns trending items for Bungie.net, collapsed into the first page of items per category. For pagination within a category, call GetTrendingCategory.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::TrendingGetTrendingCategories200Response**](Trending_GetTrendingCategories_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## trending_period_get_trending_category

> crate::models::TrendingGetTrendingCategory200Response trending_period_get_trending_category(category_id, page_number)


Returns paginated lists of trending items for a category.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**category_id** | **String** | The ID of the category for whom you want additional results. | [required] |
**page_number** | **i32** | The page # of results to return. | [required] |

### Return type

[**crate::models::TrendingGetTrendingCategory200Response**](Trending_GetTrendingCategory_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## trending_period_get_trending_entry_detail

> crate::models::TrendingGetTrendingEntryDetail200Response trending_period_get_trending_entry_detail(identifier, trending_entry_type)


Returns the detailed results for a specific trending entry. Note that trending entries are uniquely identified by a combination of *both* the TrendingEntryType *and* the identifier: the identifier alone is not guaranteed to be globally unique.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**identifier** | **String** | The identifier for the entity to be returned. | [required] |
**trending_entry_type** | **i32** | The type of entity to be returned. | [required] |

### Return type

[**crate::models::TrendingGetTrendingEntryDetail200Response**](Trending_GetTrendingEntryDetail_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

