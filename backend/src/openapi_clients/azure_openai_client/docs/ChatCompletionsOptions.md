# ChatCompletionsOptions

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**messages** | [**Vec<models::ChatRequestMessage>**](ChatRequestMessage.md) | The collection of context messages associated with this chat completions request. Typical usage begins with a chat message for the System role that provides instructions for the behavior of the assistant, followed by alternating messages between the User and Assistant roles. | 
**functions** | Option<[**Vec<models::FunctionDefinition>**](FunctionDefinition.md)> | A list of functions the model may generate JSON inputs for. | [optional]
**function_call** | Option<[**serde_json::Value**](.md)> | Controls how the model responds to function calls. \"none\" means the model does not call a function, and responds to the end-user. \"auto\" means the model can pick between an end-user or calling a function.  Specifying a particular function via `{\"name\": \"my_function\"}` forces the model to call that function.  \"none\" is the default when no functions are present. \"auto\" is the default if functions are present. | [optional]
**max_tokens** | Option<**i32**> | The maximum number of tokens allowed for the generated answer. By default, the number of tokens the model can return will be (4096 - prompt tokens).  This value is now deprecated in favor of `max_completion_tokens`, and is not compatible with o1 series models. | [optional]
**max_completion_tokens** | Option<**i32**> | An upper bound for the number of tokens that can be generated for a completion, including visible output tokens and reasoning tokens. | [optional]
**temperature** | Option<**f32**> | The sampling temperature to use that controls the apparent creativity of generated completions. Higher values will make output more random while lower values will make results more focused and deterministic. It is not recommended to modify temperature and top_p for the same completions request as the interaction of these two settings is difficult to predict. | [optional]
**top_p** | Option<**f32**> | An alternative to sampling with temperature called nucleus sampling. This value causes the model to consider the results of tokens with the provided probability mass. As an example, a value of 0.15 will cause only the tokens comprising the top 15% of probability mass to be considered. It is not recommended to modify temperature and top_p for the same completions request as the interaction of these two settings is difficult to predict. | [optional]
**logit_bias** | Option<**std::collections::HashMap<String, i32>**> | A map between GPT token IDs and bias scores that influences the probability of specific tokens appearing in a completions response. Token IDs are computed via external tokenizer tools, while bias scores reside in the range of -100 to 100 with minimum and maximum values corresponding to a full ban or exclusive selection of a token, respectively. The exact behavior of a given bias score varies by model. | [optional]
**user** | Option<**String**> | An identifier for the caller or end user of the operation. This may be used for tracking or rate-limiting purposes. | [optional]
**n** | Option<**i32**> | The number of chat completions choices that should be generated for a chat completions response. Because this setting can generate many completions, it may quickly consume your token quota. Use carefully and ensure reasonable settings for max_tokens and stop. | [optional]
**stop** | Option<**Vec<String>**> | A collection of textual sequences that will end completions generation. | [optional]
**presence_penalty** | Option<**f32**> | A value that influences the probability of generated tokens appearing based on their existing presence in generated text. Positive values will make tokens less likely to appear when they already exist and increase the model's likelihood to output new topics. | [optional]
**frequency_penalty** | Option<**f32**> | A value that influences the probability of generated tokens appearing based on their cumulative frequency in generated text. Positive values will make tokens less likely to appear as their frequency increases and decrease the likelihood of the model repeating the same statements verbatim. | [optional]
**stream** | Option<**bool**> | A value indicating whether chat completions should be streamed for this request. | [optional]
**stream_options** | Option<[**models::ChatCompletionStreamOptions**](ChatCompletionStreamOptions.md)> | Options for streaming response. Only set this when you set `stream: true`. | [optional]
**model** | Option<**String**> | The model name to provide as part of this completions request. Not applicable to Azure OpenAI, where deployment information should be included in the Azure resource URI that's connected to. | [optional]
**data_sources** | Option<[**Vec<models::AzureChatExtensionConfiguration>**](AzureChatExtensionConfiguration.md)> |   The configuration entries for Azure OpenAI chat extensions that use them.   This additional specification is only compatible with Azure OpenAI. | [optional]
**enhancements** | Option<[**models::AzureChatEnhancementConfiguration**](AzureChatEnhancementConfiguration.md)> |  | [optional]
**seed** | Option<**i64**> | If specified, the system will make a best effort to sample deterministically such that repeated requests with the same seed and parameters should return the same result. Determinism is not guaranteed, and you should refer to the system_fingerprint response parameter to monitor changes in the backend.\" | [optional]
**logprobs** | Option<**bool**> | Whether to return log probabilities of the output tokens or not. If true, returns the log probabilities of each output token returned in the `content` of `message`. This option is currently not available on the `gpt-4-vision-preview` model. | [optional][default to false]
**top_logprobs** | Option<**i32**> | An integer between 0 and 5 specifying the number of most likely tokens to return at each token position, each with an associated log probability. `logprobs` must be set to `true` if this parameter is used. | [optional]
**response_format** | Option<[**models::ChatCompletionsResponseFormat**](ChatCompletionsResponseFormat.md)> |  | [optional]
**tools** | Option<[**Vec<models::ChatCompletionsToolDefinition>**](ChatCompletionsToolDefinition.md)> | The available tool definitions that the chat completions request can use, including caller-defined functions. | [optional]
**tool_choice** | Option<[**serde_json::Value**](.md)> | If specified, the model will configure which of the provided tools it can use for the chat completions response. | [optional]
**parallel_tool_calls** | Option<**bool**> | Whether to enable parallel function calling during tool use. | [optional][default to true]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


