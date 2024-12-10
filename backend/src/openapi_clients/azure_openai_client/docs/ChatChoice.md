# ChatChoice

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**message** | Option<[**models::ChatResponseMessage**](ChatResponseMessage.md)> |  | [optional]
**logprobs** | Option<[**models::ChatChoiceLogProbabilityInfo**](ChatChoiceLogProbabilityInfo.md)> | The log probability information for this choice, as enabled via the 'logprobs' request option. | 
**index** | **i32** | The ordered index associated with this chat completions choice. | 
**finish_reason** | [**models::CompletionsFinishReason**](CompletionsFinishReason.md) |  | 
**finish_details** | Option<[**models::ChatFinishDetails**](ChatFinishDetails.md)> |  | [optional]
**delta** | Option<[**models::ChatResponseMessage**](ChatResponseMessage.md)> |  | [optional]
**content_filter_results** | Option<[**models::ContentFilterResultsForChoice**](ContentFilterResultsForChoice.md)> |  | [optional]
**enhancements** | Option<[**models::AzureChatEnhancements**](AzureChatEnhancements.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


