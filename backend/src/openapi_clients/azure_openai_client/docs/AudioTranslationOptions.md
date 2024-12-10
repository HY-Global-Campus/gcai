# AudioTranslationOptions

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**file** | **String** | The audio data to translate. This must be the binary content of a file in one of the supported media formats:  flac, mp3, mp4, mpeg, mpga, m4a, ogg, wav, webm. | 
**filename** | Option<**String**> | The optional filename or descriptive identifier to associate with with the audio data. | [optional]
**response_format** | Option<[**models::AudioTranslationFormat**](AudioTranslationFormat.md)> |  | [optional]
**prompt** | Option<**String**> | An optional hint to guide the model's style or continue from a prior audio segment. The written language of the prompt should match the primary spoken language of the audio data. | [optional]
**temperature** | Option<**f32**> | The sampling temperature, between 0 and 1. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic. If set to 0, the model will use log probability to automatically increase the temperature until certain thresholds are hit. | [optional]
**model** | Option<**String**> | The model to use for this translation request. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


