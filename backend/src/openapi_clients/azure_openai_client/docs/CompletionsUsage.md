# CompletionsUsage

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**completion_tokens** | **i32** | The number of tokens generated across all completions emissions. | 
**prompt_tokens** | **i32** | The number of tokens in the provided prompts for the completions request. | 
**total_tokens** | **i32** | The total number of tokens processed for the completions request and response. | 
**prompt_tokens_details** | Option<[**models::CompletionsUsagePromptTokensDetails**](CompletionsUsage_prompt_tokens_details.md)> |  | [optional]
**completion_tokens_details** | Option<[**models::CompletionsUsageCompletionTokensDetails**](CompletionsUsage_completion_tokens_details.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


