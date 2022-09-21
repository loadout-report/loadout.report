# SearchResultOfGroupMember

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**results** | Option<[**Vec<crate::models::GroupsV2PeriodGroupMember>**](GroupsV2.GroupMember.md)> |  | [optional]
**total_results** | Option<**i32**> |  | [optional]
**has_more** | Option<**bool**> |  | [optional]
**query** | Option<[**crate::models::QueriesPeriodPagedQuery**](Queries.PagedQuery.md)> |  | [optional]
**replacement_continuation_token** | Option<**String**> |  | [optional]
**use_total_results** | Option<**bool**> | If useTotalResults is true, then totalResults represents an accurate count.  If False, it does not, and may be estimated/only the size of the current page.  Either way, you should probably always only trust hasMore.  This is a long-held historical throwback to when we used to do paging with known total results. Those queries toasted our database, and we were left to hastily alter our endpoints and create backward- compatible shims, of which useTotalResults is one. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


