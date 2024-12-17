# OpenAiFile

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**object** | **String** | The object type, which is always 'file'. | 
**id** | **String** | The identifier, which can be referenced in API endpoints. | 
**bytes** | **i32** | The size of the file, in bytes. | 
**filename** | **String** | The name of the file. | 
**created_at** | **i32** | The Unix timestamp, in seconds, representing when this object was created. | 
**purpose** | [**models::FilePurpose**](FilePurpose.md) |  | 
**status** | Option<[**models::FileState**](FileState.md)> |  | [optional]
**status_details** | Option<**String**> | The error message with details in case processing of this file failed. This field is available in Azure OpenAI only. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


