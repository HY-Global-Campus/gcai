# AzureMachineLearningIndexChatExtensionParameters

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**top_n_documents** | Option<**i32**> | The configured top number of documents to feature for the configured query. | [optional]
**in_scope** | Option<**bool**> | Whether queries should be restricted to use of indexed data. | [optional]
**strictness** | Option<**i32**> | The configured strictness of the search relevance filtering. The higher of strictness, the higher of the precision but lower recall of the answer. | [optional]
**role_information** | Option<**String**> | Give the model instructions about how it should behave and any context it should reference when generating a response. You can describe the assistant's personality and tell it how to format responses. There's a 100 token limit for it, and it counts against the overall token limit. | [optional]
**max_search_queries** | Option<**i32**> | The max number of rewritten queries should be send to search provider for one user message. If not specified, the system will decide the number of queries to send. | [optional]
**allow_partial_result** | Option<**bool**> | If specified as true, the system will allow partial search results to be used and the request fails if all the queries fail. If not specified, or specified as false, the request will fail if any search query fails. | [optional][default to false]
**include_contexts** | Option<[**Vec<models::OnYourDataContextProperty>**](OnYourDataContextProperty.md)> | The included properties of the output context. If not specified, the default value is `citations` and `intent`. | [optional]
**authentication** | Option<[**models::OnYourDataAuthenticationOptions**](OnYourDataAuthenticationOptions.md)> |  | [optional]
**project_resource_id** | **String** | The resource ID of the Azure Machine Learning project. | 
**name** | **String** | The Azure Machine Learning vector index name. | 
**version** | **String** | The version of the Azure Machine Learning vector index. | 
**filter** | Option<**String**> | Search filter. Only supported if the Azure Machine Learning vector index is of type AzureSearch. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


