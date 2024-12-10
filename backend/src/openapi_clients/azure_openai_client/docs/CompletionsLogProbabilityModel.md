# CompletionsLogProbabilityModel

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**tokens** | **Vec<String>** | The textual forms of tokens evaluated in this probability model. | 
**token_logprobs** | **Vec<f32>** | A collection of log probability values for the tokens in this completions data. | 
**top_logprobs** | [**Vec<std::collections::HashMap<String, f32>>**](std::collections::HashMap.md) | A mapping of tokens to maximum log probability values in this completions data. | 
**text_offset** | **Vec<i32>** | The text offsets associated with tokens in this completions data. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


