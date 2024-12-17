# \DefaultApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_upload_part**](DefaultApi.md#add_upload_part) | **POST** /uploads/{upload_id}/parts | 
[**cancel_batch**](DefaultApi.md#cancel_batch) | **POST** /batches/{batchId}/cancel | 
[**cancel_upload**](DefaultApi.md#cancel_upload) | **POST** /uploads/{upload_id}/cancel | 
[**complete_upload**](DefaultApi.md#complete_upload) | **POST** /uploads/{upload_id}/complete | 
[**create_batch**](DefaultApi.md#create_batch) | **POST** /batches | 
[**create_upload**](DefaultApi.md#create_upload) | **POST** /uploads | 
[**delete_file**](DefaultApi.md#delete_file) | **DELETE** /files/{fileId} | 
[**generate_speech_from_text**](DefaultApi.md#generate_speech_from_text) | **POST** /deployments/{deploymentId}/audio/speech | 
[**get_audio_transcription_as_plain_text**](DefaultApi.md#get_audio_transcription_as_plain_text) | **POST** /deployments/{deploymentId}/audio/transcriptions | 
[**get_audio_translation_as_plain_text**](DefaultApi.md#get_audio_translation_as_plain_text) | **POST** /deployments/{deploymentId}/audio/translations | 
[**get_batch**](DefaultApi.md#get_batch) | **GET** /batches/{batchId} | 
[**get_chat_completions**](DefaultApi.md#get_chat_completions) | **POST** /deployments/{deploymentId}/chat/completions | 
[**get_completions**](DefaultApi.md#get_completions) | **POST** /deployments/{deploymentId}/completions | 
[**get_embeddings**](DefaultApi.md#get_embeddings) | **POST** /deployments/{deploymentId}/embeddings | 
[**get_file**](DefaultApi.md#get_file) | **GET** /files/{fileId} | 
[**get_file_content**](DefaultApi.md#get_file_content) | **GET** /files/{fileId}/content | 
[**get_image_generations**](DefaultApi.md#get_image_generations) | **POST** /deployments/{deploymentId}/images/generations | 
[**list_batches**](DefaultApi.md#list_batches) | **GET** /batches | 
[**list_files**](DefaultApi.md#list_files) | **GET** /files | 
[**upload_file**](DefaultApi.md#upload_file) | **POST** /files | 



## add_upload_part

> models::UploadPart add_upload_part(upload_id, data)


Adds a Part to an Upload object. A Part represents a chunk of bytes from the file you are trying to upload.  Each Part can be at most 64 MB, and you can add Parts until you hit the Upload maximum of 8 GB.  It is possible to add multiple Parts in parallel. You can decide the intended order of the Parts when you complete the Upload.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**upload_id** | **String** | The ID of the upload associated with this operation. | [required] |
**data** | **std::path::PathBuf** | The chunk of bytes for this Part. | [required] |

### Return type

[**models::UploadPart**](UploadPart.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [OAuth2Auth](../README.md#OAuth2Auth)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cancel_batch

> models::Batch cancel_batch(batch_id)


Gets details for a single batch specified by the given batchID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**batch_id** | **String** | The identifier of the batch. | [required] |

### Return type

[**models::Batch**](Batch.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [OAuth2Auth](../README.md#OAuth2Auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cancel_upload

> models::Upload cancel_upload(upload_id)


Cancels the Upload. No Parts may be added after an Upload is cancelled.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**upload_id** | **String** | The ID of the upload associated with this operation. | [required] |

### Return type

[**models::Upload**](Upload.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [OAuth2Auth](../README.md#OAuth2Auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## complete_upload

> models::Upload complete_upload(upload_id, request_body)


Completes the Upload.  Within the returned Upload object, there is a nested File object that is ready to use in the rest of the platform.  You can specify the order of the Parts by passing in an ordered list of the Part IDs.  The number of bytes uploaded upon completion must match the number of bytes initially specified when creating the Upload object. No Parts may be added after an Upload is completed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**upload_id** | **String** | The ID of the upload associated with this operation. | [required] |
**request_body** | [**CompleteUploadRequest**](CompleteUploadRequest.md) | The request body for the completion operation. | [required] |

### Return type

[**models::Upload**](Upload.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [OAuth2Auth](../README.md#OAuth2Auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_batch

> models::Batch create_batch(create_batch_request)


Creates and executes a batch from an uploaded file of requests. Response includes details of the enqueued job including job status. The ID of the result file is added to the response once complete.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_batch_request** | [**BatchCreateRequest**](BatchCreateRequest.md) | The specification of the batch to create and execute. | [required] |

### Return type

[**models::Batch**](Batch.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [OAuth2Auth](../README.md#OAuth2Auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_upload

> models::Upload create_upload(request_body)


Creates an intermediate Upload object that you can add Parts to. Currently, an Upload can accept at most 8 GB in total and expires after an hour after you create it.  Once you complete the Upload, we will create a File object that contains all the parts you uploaded. This File is usable in the rest of our platform as a regular File object.  For certain purposes, the correct mime_type must be specified. Please refer to documentation for the supported MIME types for your use case.  For guidance on the proper filename extensions for each purpose, please follow the documentation on creating a File.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_body** | [**CreateUploadRequest**](CreateUploadRequest.md) | The request body for the operation options. | [required] |

### Return type

[**models::Upload**](Upload.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [OAuth2Auth](../README.md#OAuth2Auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_file

> models::FileDeletionStatus delete_file(file_id)


Delete a previously uploaded file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_id** | **String** | The ID of the file to delete. | [required] |

### Return type

[**models::FileDeletionStatus**](FileDeletionStatus.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [OAuth2Auth](../README.md#OAuth2Auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


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


## get_batch

> models::Batch get_batch(batch_id)


Gets details for a single batch specified by the given batchID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**batch_id** | **String** | The identifier of the batch. | [required] |

### Return type

[**models::Batch**](Batch.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [OAuth2Auth](../README.md#OAuth2Auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

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


## get_file

> models::OpenAiFile get_file(file_id)


Returns information about a specific file. Does not retrieve file content.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_id** | **String** | The ID of the file to retrieve. | [required] |

### Return type

[**models::OpenAiFile**](OpenAIFile.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [OAuth2Auth](../README.md#OAuth2Auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_file_content

> String get_file_content(file_id)


Returns information about a specific file. Does not retrieve file content.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_id** | **String** | The ID of the file to retrieve. | [required] |

### Return type

**String**

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [OAuth2Auth](../README.md#OAuth2Auth)

### HTTP request headers

- **Content-Type**: Not defined
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


## list_batches

> models::ListBatches200Response list_batches(after, limit)


Gets a list of all batches owned by the Azure OpenAI resource.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**after** | Option<**String**> | Identifier for the last event from the previous pagination request. |  |
**limit** | Option<**i32**> | Number of batches to retrieve. Defaults to 20. |  |

### Return type

[**models::ListBatches200Response**](ListBatches_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [OAuth2Auth](../README.md#OAuth2Auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_files

> models::FileListResponse list_files(purpose)


Gets a list of previously uploaded files.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**purpose** | Option<**String**> | A value that, when provided, limits list results to files matching the corresponding purpose. |  |

### Return type

[**models::FileListResponse**](FileListResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [OAuth2Auth](../README.md#OAuth2Auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upload_file

> models::OpenAiFile upload_file(file, purpose, filename)


Uploads a file for use by other operations.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file** | **std::path::PathBuf** | The file data (not filename) to upload. | [required] |
**purpose** | **String** | The intended purpose of the file. | [required] |
**filename** | Option<**String**> | A filename to associate with the uploaded data. |  |

### Return type

[**models::OpenAiFile**](OpenAIFile.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [OAuth2Auth](../README.md#OAuth2Auth)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

