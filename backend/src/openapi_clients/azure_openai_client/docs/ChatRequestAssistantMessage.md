# ChatRequestAssistantMessage

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**content** | [**serde_json::Value**](.md) | The content of the message. | 
**name** | Option<**String**> | An optional name for the participant. | [optional]
**tool_calls** | Option<[**Vec<models::ChatCompletionsToolCall>**](ChatCompletionsToolCall.md)> | The tool calls that must be resolved and have their outputs appended to subsequent input messages for the chat completions request to resolve as configured. | [optional]
**function_call** | Option<[**models::FunctionCall**](FunctionCall.md)> |  | [optional]
**refusal** | Option<**String**> | The refusal message by the assistant. | [optional]
**role** | [**models::ChatRole**](ChatRole.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


