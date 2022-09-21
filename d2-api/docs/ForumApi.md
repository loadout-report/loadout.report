# \ForumApi

All URIs are relative to *https://www.bungie.net/Platform*

Method | HTTP request | Description
------------- | ------------- | -------------
[**forum_period_get_core_topics_paged**](ForumApi.md#forum_period_get_core_topics_paged) | **GET** /Forum/GetCoreTopicsPaged/{page}/{sort}/{quickDate}/{categoryFilter}/ | 
[**forum_period_get_forum_tag_suggestions**](ForumApi.md#forum_period_get_forum_tag_suggestions) | **GET** /Forum/GetForumTagSuggestions/ | 
[**forum_period_get_poll**](ForumApi.md#forum_period_get_poll) | **GET** /Forum/Poll/{topicId}/ | 
[**forum_period_get_post_and_parent**](ForumApi.md#forum_period_get_post_and_parent) | **GET** /Forum/GetPostAndParent/{childPostId}/ | 
[**forum_period_get_post_and_parent_awaiting_approval**](ForumApi.md#forum_period_get_post_and_parent_awaiting_approval) | **GET** /Forum/GetPostAndParentAwaitingApproval/{childPostId}/ | 
[**forum_period_get_posts_threaded_paged**](ForumApi.md#forum_period_get_posts_threaded_paged) | **GET** /Forum/GetPostsThreadedPaged/{parentPostId}/{page}/{pageSize}/{replySize}/{getParentPost}/{rootThreadMode}/{sortMode}/ | 
[**forum_period_get_posts_threaded_paged_from_child**](ForumApi.md#forum_period_get_posts_threaded_paged_from_child) | **GET** /Forum/GetPostsThreadedPagedFromChild/{childPostId}/{page}/{pageSize}/{replySize}/{rootThreadMode}/{sortMode}/ | 
[**forum_period_get_recruitment_thread_summaries**](ForumApi.md#forum_period_get_recruitment_thread_summaries) | **POST** /Forum/Recruit/Summaries/ | 
[**forum_period_get_topic_for_content**](ForumApi.md#forum_period_get_topic_for_content) | **GET** /Forum/GetTopicForContent/{contentId}/ | 
[**forum_period_get_topics_paged**](ForumApi.md#forum_period_get_topics_paged) | **GET** /Forum/GetTopicsPaged/{page}/{pageSize}/{group}/{sort}/{quickDate}/{categoryFilter}/ | 



## forum_period_get_core_topics_paged

> crate::models::ForumGetTopicsPaged200Response forum_period_get_core_topics_paged(category_filter, page, quick_date, sort, locales)


Gets a listing of all topics marked as part of the core group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**category_filter** | **i32** | The category filter. | [required] |
**page** | **i32** | Zero base page | [required] |
**quick_date** | **i32** | The date filter. | [required] |
**sort** | **i32** | The sort mode. | [required] |
**locales** | Option<**String**> | Comma seperated list of locales posts must match to return in the result list. Default 'en' |  |

### Return type

[**crate::models::ForumGetTopicsPaged200Response**](Forum_GetTopicsPaged_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## forum_period_get_forum_tag_suggestions

> crate::models::ForumGetForumTagSuggestions200Response forum_period_get_forum_tag_suggestions(partialtag)


Gets tag suggestions based on partial text entry, matching them with other tags previously used in the forums.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**partialtag** | Option<**String**> | The partial tag input to generate suggestions from. |  |

### Return type

[**crate::models::ForumGetForumTagSuggestions200Response**](Forum_GetForumTagSuggestions_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## forum_period_get_poll

> crate::models::ForumGetTopicsPaged200Response forum_period_get_poll(topic_id)


Gets the specified forum poll.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**topic_id** | **i64** | The post id of the topic that has the poll. | [required] |

### Return type

[**crate::models::ForumGetTopicsPaged200Response**](Forum_GetTopicsPaged_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## forum_period_get_post_and_parent

> crate::models::ForumGetTopicsPaged200Response forum_period_get_post_and_parent(child_post_id, showbanned)


Returns the post specified and its immediate parent.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**child_post_id** | **i64** |  | [required] |
**showbanned** | Option<**String**> | If this value is not null or empty, banned posts are requested to be returned |  |

### Return type

[**crate::models::ForumGetTopicsPaged200Response**](Forum_GetTopicsPaged_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## forum_period_get_post_and_parent_awaiting_approval

> crate::models::ForumGetTopicsPaged200Response forum_period_get_post_and_parent_awaiting_approval(child_post_id, showbanned)


Returns the post specified and its immediate parent of posts that are awaiting approval.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**child_post_id** | **i64** |  | [required] |
**showbanned** | Option<**String**> | If this value is not null or empty, banned posts are requested to be returned |  |

### Return type

[**crate::models::ForumGetTopicsPaged200Response**](Forum_GetTopicsPaged_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## forum_period_get_posts_threaded_paged

> crate::models::ForumGetTopicsPaged200Response forum_period_get_posts_threaded_paged(get_parent_post, page, page_size, parent_post_id, reply_size, root_thread_mode, sort_mode, showbanned)


Returns a thread of posts at the given parent, optionally returning replies to those posts as well as the original parent.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_parent_post** | **bool** |  | [required] |
**page** | **i32** |  | [required] |
**page_size** | **i32** |  | [required] |
**parent_post_id** | **i64** |  | [required] |
**reply_size** | **i32** |  | [required] |
**root_thread_mode** | **bool** |  | [required] |
**sort_mode** | **i32** |  | [required] |
**showbanned** | Option<**String**> | If this value is not null or empty, banned posts are requested to be returned |  |

### Return type

[**crate::models::ForumGetTopicsPaged200Response**](Forum_GetTopicsPaged_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## forum_period_get_posts_threaded_paged_from_child

> crate::models::ForumGetTopicsPaged200Response forum_period_get_posts_threaded_paged_from_child(child_post_id, page, page_size, reply_size, root_thread_mode, sort_mode, showbanned)


Returns a thread of posts starting at the topicId of the input childPostId, optionally returning replies to those posts as well as the original parent.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**child_post_id** | **i64** |  | [required] |
**page** | **i32** |  | [required] |
**page_size** | **i32** |  | [required] |
**reply_size** | **i32** |  | [required] |
**root_thread_mode** | **bool** |  | [required] |
**sort_mode** | **i32** |  | [required] |
**showbanned** | Option<**String**> | If this value is not null or empty, banned posts are requested to be returned |  |

### Return type

[**crate::models::ForumGetTopicsPaged200Response**](Forum_GetTopicsPaged_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## forum_period_get_recruitment_thread_summaries

> crate::models::ForumGetRecruitmentThreadSummaries200Response forum_period_get_recruitment_thread_summaries()


Allows the caller to get a list of to 25 recruitment thread summary information objects.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ForumGetRecruitmentThreadSummaries200Response**](Forum_GetRecruitmentThreadSummaries_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## forum_period_get_topic_for_content

> crate::models::ForumGetTopicForContent200Response forum_period_get_topic_for_content(content_id)


Gets the post Id for the given content item's comments, if it exists.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**content_id** | **i64** |  | [required] |

### Return type

[**crate::models::ForumGetTopicForContent200Response**](Forum_GetTopicForContent_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## forum_period_get_topics_paged

> crate::models::ForumGetTopicsPaged200Response forum_period_get_topics_paged(category_filter, group, page, page_size, quick_date, sort, locales, tagstring)


Get topics from any forum.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**category_filter** | **i32** | A category filter | [required] |
**group** | **i64** | The group, if any. | [required] |
**page** | **i32** | Zero paged page number | [required] |
**page_size** | **i32** | Unused | [required] |
**quick_date** | **i32** | A date filter. | [required] |
**sort** | **i32** | The sort mode. | [required] |
**locales** | Option<**String**> | Comma seperated list of locales posts must match to return in the result list. Default 'en' |  |
**tagstring** | Option<**String**> | The tags to search, if any. |  |

### Return type

[**crate::models::ForumGetTopicsPaged200Response**](Forum_GetTopicsPaged_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

