# ContentPeriodContentItemPublicContract

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**content_id** | Option<**i64**> |  | [optional]
**c_type** | Option<**String**> |  | [optional]
**cms_path** | Option<**String**> |  | [optional]
**creation_date** | Option<**String**> |  | [optional]
**modify_date** | Option<**String**> |  | [optional]
**allow_comments** | Option<**bool**> |  | [optional]
**has_age_gate** | Option<**bool**> |  | [optional]
**minimum_age** | Option<**i32**> |  | [optional]
**rating_image_path** | Option<**String**> |  | [optional]
**author** | Option<[**crate::models::UserPeriodGeneralUser**](User.GeneralUser.md)> |  | [optional]
**auto_english_property_fallback** | Option<**bool**> |  | [optional]
**properties** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | Firehose content is really a collection of metadata and \"properties\", which are the potentially-but-not-strictly localizable data that comprises the meat of whatever content is being shown.  As Cole Porter would have crooned, \"Anything Goes\" with Firehose properties. They are most often strings, but they can theoretically be anything. They are JSON encoded, and could be JSON structures, simple strings, numbers etc... The Content Type of the item (cType) will describe the properties, and thus how they ought to be deserialized. | [optional]
**representations** | Option<[**Vec<crate::models::ContentPeriodContentRepresentation>**](Content.ContentRepresentation.md)> |  | [optional]
**tags** | Option<**Vec<String>**> | NOTE: Tags will always be lower case. | [optional]
**comment_summary** | Option<[**crate::models::ContentPeriodCommentSummary**](Content.CommentSummary.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


