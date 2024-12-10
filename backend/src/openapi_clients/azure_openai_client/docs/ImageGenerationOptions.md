# ImageGenerationOptions

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**model** | Option<**String**> | The model name or Azure OpenAI model deployment name to use for image generation. If not specified, dall-e-2 will be inferred as a default. | [optional]
**prompt** | **String** | A description of the desired images. | 
**n** | Option<**i32**> | The number of images to generate. Dall-e-2 models support values between 1 and 10. Dall-e-3 models only support a value of 1. | [optional][default to 1]
**size** | Option<**String**> | The desired dimensions for generated images. Dall-e-2 models support 256x256, 512x512, or 1024x1024. Dall-e-3 models support 1024x1024, 1792x1024, or 1024x1792. | [optional][default to Variant1024x1024]
**response_format** | Option<**String**> | The format in which image generation response items should be presented. | [optional][default to Url]
**quality** | Option<**String**> | The desired image generation quality level to use. Only configurable with dall-e-3 models. | [optional][default to Standard]
**style** | Option<**String**> | The desired image generation style to use. Only configurable with dall-e-3 models. | [optional][default to Vivid]
**user** | Option<**String**> | A unique identifier representing your end-user, which can help to monitor and detect abuse. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


