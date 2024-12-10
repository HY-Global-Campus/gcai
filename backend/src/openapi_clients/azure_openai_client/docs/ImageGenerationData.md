# ImageGenerationData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**url** | Option<**String**> | The URL that provides temporary access to download the generated image. | [optional]
**b64_json** | Option<**String**> | The complete data for an image, represented as a base64-encoded string. | [optional]
**content_filter_results** | Option<[**models::ImageGenerationContentFilterResults**](ImageGenerationContentFilterResults.md)> |  | [optional]
**revised_prompt** | Option<**String**> | The final prompt used by the model to generate the image. Only provided with dall-3-models and only when revisions were made to the prompt. | [optional]
**prompt_filter_results** | Option<[**models::ImageGenerationPromptFilterResults**](ImageGenerationPromptFilterResults.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


