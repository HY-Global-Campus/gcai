# ChatCompletions

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | A unique identifier associated with this chat completions response. | 
**created** | **i32** | The first timestamp associated with generation activity for this completions response, represented as seconds since the beginning of the Unix epoch of 00:00 on 1 Jan 1970. | 
**choices** | [**Vec<models::ChatChoice>**](ChatChoice.md) | The collection of completions choices associated with this completions response. Generally, `n` choices are generated per provided prompt with a default value of 1. Token limits and other settings may limit the number of choices generated. | 
**model** | Option<**String**> | The model name used for this completions request. | [optional]
**prompt_filter_results** | Option<[**Vec<models::ContentFilterResultsForPrompt>**](ContentFilterResultsForPrompt.md)> | Content filtering results for zero or more prompts in the request. In a streaming request, results for different prompts may arrive at different times or in different orders. | [optional]
**system_fingerprint** | Option<**String**> | Can be used in conjunction with the `seed` request parameter to understand when backend changes have been made that might impact determinism. | [optional]
**usage** | [**models::CompletionsUsage**](CompletionsUsage.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


