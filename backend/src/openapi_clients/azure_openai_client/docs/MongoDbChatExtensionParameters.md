# MongoDbChatExtensionParameters

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**top_n_documents** | Option<**i32**> | The configured top number of documents to feature for the configured query. | [optional]
**in_scope** | Option<**bool**> | Whether queries should be restricted to use of indexed data. | [optional]
**strictness** | Option<**i32**> | The configured strictness of the search relevance filtering. The higher of strictness, the higher of the precision but lower recall of the answer. | [optional]
**max_search_queries** | Option<**i32**> | The max number of rewritten queries should be send to search provider for one user message. If not specified, the system will decide the number of queries to send. | [optional]
**allow_partial_result** | Option<**bool**> | If specified as true, the system will allow partial search results to be used and the request fails if all the queries fail. If not specified, or specified as false, the request will fail if any search query fails. | [optional][default to false]
**include_contexts** | Option<[**Vec<models::OnYourDataContextProperty>**](OnYourDataContextProperty.md)> | The included properties of the output context. If not specified, the default value is `citations` and `intent`. | [optional]
**authentication** | Option<[**models::OnYourDataUsernameAndPasswordAuthenticationOptions**](OnYourDataUsernameAndPasswordAuthenticationOptions.md)> |  | [optional]
**endpoint** | **String** | The endpoint name for MongoDB. | 
**collection_name** | **String** | The collection name for MongoDB. | 
**database_name** | **String** | The database name for MongoDB. | 
**app_name** | **String** | The app name for MongoDB. | 
**index_name** | **String** | The name of the MongoDB index. | 
**fields_mapping** | [**models::MongoDbChatExtensionParametersFieldsMapping**](MongoDBChatExtensionParameters_fields_mapping.md) |  | 
**embedding_dependency** | [**serde_json::Value**](.md) | The vectorization source to use with the MongoDB chat extension. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


