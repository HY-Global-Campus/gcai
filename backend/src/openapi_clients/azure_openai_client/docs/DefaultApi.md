# \DefaultApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**generate_speech_from_text**](DefaultApi.md#generate_speech_from_text) | **POST** /deployments/{deploymentId}/audio/speech | 
[**get_audio_transcription_as_plain_text**](DefaultApi.md#get_audio_transcription_as_plain_text) | **POST** /deployments/{deploymentId}/audio/transcriptions | 
[**get_audio_translation_as_plain_text**](DefaultApi.md#get_audio_translation_as_plain_text) | **POST** /deployments/{deploymentId}/audio/translations | 
[**get_chat_completions**](DefaultApi.md#get_chat_completions) | **POST** /deployments/{deploymentId}/chat/completions | 
[**get_completions**](DefaultApi.md#get_completions) | **POST** /deployments/{deploymentId}/completions | 
[**get_embeddings**](DefaultApi.md#get_embeddings) | **POST** /deployments/{deploymentId}/embeddings | 
[**get_image_generations**](DefaultApi.md#get_image_generations) | **POST** /deployments/{deploymentId}/images/generations | 



## generate_speech_from_text

> std::path::PathBuf generate_speech_from_text(api_version, deployment_id, body)


Generates text-to-speech audio from the input text.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | The API version to use for this operation. | [required] |
**deployment_id** | **String** | Specifies either the model deployment name (when using Azure OpenAI) or model name (when using non-Azure OpenAI) to use for this request. | [required] |
**body** | [**SpeechGenerationOptions**](SpeechGenerationOptions.md) | A representation of the request options that control the behavior of a text-to-speech operation. | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [OAuth2Auth](../README.md#OAuth2Auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/octet-stream, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_audio_transcription_as_plain_text

> String get_audio_transcription_as_plain_text(api_version, deployment_id, file, filename, response_format, language, prompt, temperature, timestamp_granularities, model)


Gets transcribed text and associated metadata from provided spoken audio data. Audio will be transcribed in the written language corresponding to the language it was spoken in.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | The API version to use for this operation. | [required] |
**deployment_id** | **String** | Specifies either the model deployment name (when using Azure OpenAI) or model name (when using non-Azure OpenAI) to use for this request. | [required] |
**file** | **std::path::PathBuf** | The audio data to transcribe. This must be the binary content of a file in one of the supported media formats:  flac, mp3, mp4, mpeg, mpga, m4a, ogg, wav, webm. | [required] |
**filename** | Option<**String**> | The optional filename or descriptive identifier to associate with with the audio data. |  |
**response_format** | Option<**String**> | The requested format of the transcription response data, which will influence the content and detail of the result. |  |
**language** | Option<**String**> | The primary spoken language of the audio data to be transcribed, supplied as a two-letter ISO-639-1 language code such as 'en' or 'fr'. Providing this known input language is optional but may improve the accuracy and/or latency of transcription. |  |
**prompt** | Option<**String**> | An optional hint to guide the model's style or continue from a prior audio segment. The written language of the prompt should match the primary spoken language of the audio data. |  |
**temperature** | Option<**f32**> | The sampling temperature, between 0 and 1. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic. If set to 0, the model will use log probability to automatically increase the temperature until certain thresholds are hit. |  |
**timestamp_granularities** | Option<[**Vec<String>**](String.md)> | The timestamp granularities to populate for this transcription. `response_format` must be set `verbose_json` to use timestamp granularities. Either or both of these options are supported: `word`, or `segment`. Note: There is no additional latency for segment timestamps, but generating word timestamps incurs additional latency. |  |
**model** | Option<**String**> | The model to use for this transcription request. |  |

### Return type

**String**

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [OAuth2Auth](../README.md#OAuth2Auth)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: text/plain, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_audio_translation_as_plain_text

> String get_audio_translation_as_plain_text(api_version, deployment_id, file, filename, response_format, prompt, temperature, model)


Gets English language transcribed text and associated metadata from provided spoken audio data.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | The API version to use for this operation. | [required] |
**deployment_id** | **String** | Specifies either the model deployment name (when using Azure OpenAI) or model name (when using non-Azure OpenAI) to use for this request. | [required] |
**file** | **std::path::PathBuf** | The audio data to translate. This must be the binary content of a file in one of the supported media formats:  flac, mp3, mp4, mpeg, mpga, m4a, ogg, wav, webm. | [required] |
**filename** | Option<**String**> | The optional filename or descriptive identifier to associate with with the audio data. |  |
**response_format** | Option<**String**> | The requested format of the translation response data, which will influence the content and detail of the result. |  |
**prompt** | Option<**String**> | An optional hint to guide the model's style or continue from a prior audio segment. The written language of the prompt should match the primary spoken language of the audio data. |  |
**temperature** | Option<**f32**> | The sampling temperature, between 0 and 1. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic. If set to 0, the model will use log probability to automatically increase the temperature until certain thresholds are hit. |  |
**model** | Option<**String**> | The model to use for this translation request. |  |

### Return type

**String**

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [OAuth2Auth](../README.md#OAuth2Auth)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: text/plain, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_chat_completions

> models::ChatCompletions get_chat_completions(api_version, deployment_id, body)


Gets chat completions for the provided chat messages. Completions support a wide variety of tasks and generate text that continues from or \"completes\" provided prompt data.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | The API version to use for this operation. | [required] |
**deployment_id** | **String** | Specifies either the model deployment name (when using Azure OpenAI) or model name (when using non-Azure OpenAI) to use for this request. | [required] |
**body** | [**ChatCompletionsOptions**](ChatCompletionsOptions.md) | The configuration information for a chat completions request. Completions support a wide variety of tasks and generate text that continues from or \"completes\" provided prompt data. | [required] |

### Return type

[**models::ChatCompletions**](ChatCompletions.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [OAuth2Auth](../README.md#OAuth2Auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_completions

> models::Completions get_completions(api_version, deployment_id, body)


Gets completions for the provided input prompts. Completions support a wide variety of tasks and generate text that continues from or \"completes\" provided prompt data.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | The API version to use for this operation. | [required] |
**deployment_id** | **String** | Specifies either the model deployment name (when using Azure OpenAI) or model name (when using non-Azure OpenAI) to use for this request. | [required] |
**body** | [**CompletionsOptions**](CompletionsOptions.md) | The configuration information for a completions request. Completions support a wide variety of tasks and generate text that continues from or \"completes\" provided prompt data. | [required] |

### Return type

[**models::Completions**](Completions.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [OAuth2Auth](../README.md#OAuth2Auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_embeddings

> models::Embeddings get_embeddings(api_version, deployment_id, body)


Return the embeddings for a given prompt.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | The API version to use for this operation. | [required] |
**deployment_id** | **String** | Specifies either the model deployment name (when using Azure OpenAI) or model name (when using non-Azure OpenAI) to use for this request. | [required] |
**body** | [**EmbeddingsOptions**](EmbeddingsOptions.md) | The configuration information for an embeddings request. Embeddings measure the relatedness of text strings and are commonly used for search, clustering, recommendations, and other similar scenarios. | [required] |

### Return type

[**models::Embeddings**](Embeddings.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [OAuth2Auth](../README.md#OAuth2Auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_image_generations

> models::ImageGenerations get_image_generations(api_version, deployment_id, body)


Creates an image given a prompt.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | The API version to use for this operation. | [required] |
**deployment_id** | **String** | Specifies either the model deployment name (when using Azure OpenAI) or model name (when using non-Azure OpenAI) to use for this request. | [required] |
**body** | [**ImageGenerationOptions**](ImageGenerationOptions.md) | Represents the request data used to generate images. | [required] |

### Return type

[**models::ImageGenerations**](ImageGenerations.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [OAuth2Auth](../README.md#OAuth2Auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

