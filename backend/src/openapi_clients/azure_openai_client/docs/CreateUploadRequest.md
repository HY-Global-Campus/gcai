# CreateUploadRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**filename** | **String** | The name of the file to upload. | 
**purpose** | **String** | The intended purpose of the uploaded file.  Use 'assistants' for Assistants and Message files, 'vision' for Assistants image file inputs, 'batch' for Batch API, and 'fine-tune' for Fine-tuning. | 
**bytes** | **i32** | The number of bytes in the file you are uploading. | 
**mime_type** | **String** | The MIME type of the file.  This must fall within the supported MIME types for your file purpose. See the supported MIME types for assistants and vision. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


