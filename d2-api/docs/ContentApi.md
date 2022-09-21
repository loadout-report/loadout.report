# \ContentApi

All URIs are relative to *https://www.bungie.net/Platform*

Method | HTTP request | Description
------------- | ------------- | -------------
[**content_period_get_content_by_id**](ContentApi.md#content_period_get_content_by_id) | **GET** /Content/GetContentById/{id}/{locale}/ | 
[**content_period_get_content_by_tag_and_type**](ContentApi.md#content_period_get_content_by_tag_and_type) | **GET** /Content/GetContentByTagAndType/{tag}/{type}/{locale}/ | 
[**content_period_get_content_type**](ContentApi.md#content_period_get_content_type) | **GET** /Content/GetContentType/{type}/ | 
[**content_period_rss_news_articles**](ContentApi.md#content_period_rss_news_articles) | **GET** /Content/Rss/NewsArticles/{pageToken}/ | 
[**content_period_search_content_by_tag_and_type**](ContentApi.md#content_period_search_content_by_tag_and_type) | **GET** /Content/SearchContentByTagAndType/{tag}/{type}/{locale}/ | 
[**content_period_search_content_with_text**](ContentApi.md#content_period_search_content_with_text) | **GET** /Content/Search/{locale}/ | 
[**content_period_search_help_articles**](ContentApi.md#content_period_search_help_articles) | **GET** /Content/SearchHelpArticles/{searchtext}/{size}/ | 



## content_period_get_content_by_id

> crate::models::ContentGetContentById200Response content_period_get_content_by_id(id, locale, head)


Returns a content item referenced by id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** |  | [required] |
**locale** | **String** |  | [required] |
**head** | Option<**bool**> | false |  |

### Return type

[**crate::models::ContentGetContentById200Response**](Content_GetContentById_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## content_period_get_content_by_tag_and_type

> crate::models::ContentGetContentById200Response content_period_get_content_by_tag_and_type(locale, tag, r#type, head)


Returns the newest item that matches a given tag and Content Type.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**locale** | **String** |  | [required] |
**tag** | **String** |  | [required] |
**r#type** | **String** |  | [required] |
**head** | Option<**bool**> | Not used. |  |

### Return type

[**crate::models::ContentGetContentById200Response**](Content_GetContentById_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## content_period_get_content_type

> crate::models::ContentGetContentType200Response content_period_get_content_type(r#type)


Gets an object describing a particular variant of content.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#type** | **String** |  | [required] |

### Return type

[**crate::models::ContentGetContentType200Response**](Content_GetContentType_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## content_period_rss_news_articles

> crate::models::ContentRssNewsArticles200Response content_period_rss_news_articles(page_token)


Returns a JSON string response that is the RSS feed for news articles.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_token** | **String** | Zero-based pagination token for paging through result sets. | [required] |

### Return type

[**crate::models::ContentRssNewsArticles200Response**](Content_RssNewsArticles_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## content_period_search_content_by_tag_and_type

> crate::models::ContentSearchContentWithText200Response content_period_search_content_by_tag_and_type(locale, tag, r#type, currentpage, head, itemsperpage)


Searches for Content Items that match the given Tag and Content Type.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**locale** | **String** |  | [required] |
**tag** | **String** |  | [required] |
**r#type** | **String** |  | [required] |
**currentpage** | Option<**i32**> | Page number for the search results starting with page 1. |  |
**head** | Option<**bool**> | Not used. |  |
**itemsperpage** | Option<**i32**> | Not used. |  |

### Return type

[**crate::models::ContentSearchContentWithText200Response**](Content_SearchContentWithText_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## content_period_search_content_with_text

> crate::models::ContentSearchContentWithText200Response content_period_search_content_with_text(locale, ctype, currentpage, head, searchtext, source, tag)


Gets content based on querystring information passed in. Provides basic search and text search capabilities.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**locale** | **String** |  | [required] |
**ctype** | Option<**String**> | Content type tag: Help, News, etc. Supply multiple ctypes separated by space. |  |
**currentpage** | Option<**i32**> | Page number for the search results, starting with page 1. |  |
**head** | Option<**bool**> | Not used. |  |
**searchtext** | Option<**String**> | Word or phrase for the search. |  |
**source** | Option<**String**> | For analytics, hint at the part of the app that triggered the search. Optional. |  |
**tag** | Option<**String**> | Tag used on the content to be searched. |  |

### Return type

[**crate::models::ContentSearchContentWithText200Response**](Content_SearchContentWithText_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## content_period_search_help_articles

> crate::models::ContentSearchHelpArticles200Response content_period_search_help_articles(searchtext, size)


Search for Help Articles.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**searchtext** | **String** |  | [required] |
**size** | **String** |  | [required] |

### Return type

[**crate::models::ContentSearchHelpArticles200Response**](Content_SearchHelpArticles_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

