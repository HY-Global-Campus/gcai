# AzureChatExtensionsMessageContext

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**citations** | Option<[**Vec<models::AzureChatExtensionDataSourceResponseCitation>**](AzureChatExtensionDataSourceResponseCitation.md)> |   The contextual information associated with the Azure chat extensions used for a chat completions request.   These messages describe the data source retrievals, plugin invocations, and other intermediate steps taken in the   course of generating a chat completions response that was augmented by capabilities from Azure OpenAI chat   extensions. | [optional]
**intent** | Option<**String**> | The detected intent from the chat history, used to pass to the next turn to carry over the context. | [optional]
**all_retrieved_documents** | Option<[**Vec<models::AzureChatExtensionRetrievedDocument>**](AzureChatExtensionRetrievedDocument.md)> | All the retrieved documents. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


