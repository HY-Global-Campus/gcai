# Upload

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The Upload unique identifier, which can be referenced in API endpoints. | 
**created_at** | **i32** | The Unix timestamp (in seconds) for when the Upload was created. | 
**filename** | **String** | The name of the file to be uploaded. | 
**bytes** | **i64** | The intended number of bytes to be uploaded. | 
**purpose** | **String** | The intended purpose of the file. | 
**status** | **String** | The status of the Upload. | 
**expires_at** | **i32** | The Unix timestamp (in seconds) for when the Upload was created. | 
**object** | Option<**String**> | The object type, which is always \"upload\". | [optional]
**file** | Option<[**models::OpenAiFile**](OpenAIFile.md)> | The ready File object after the Upload is completed. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


