# ChatTokenLogProbabilityResult

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**token** | **String** | The message content token. | 
**logprob** | **f32** | The log probability of the message content token. | 
**bytes** | Option<**Vec<i32>**> | A list of integers representing the UTF-8 bytes representation of the token. Useful in instances where characters are represented by multiple tokens and their byte representations must be combined to generate the correct text representation. Can be null if there is no bytes representation for the token. | 
**top_logprobs** | Option<[**Vec<models::ChatTokenLogProbabilityInfo>**](ChatTokenLogProbabilityInfo.md)> | The list of most likely tokens and their log probability information, as requested via 'top_logprobs'. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


