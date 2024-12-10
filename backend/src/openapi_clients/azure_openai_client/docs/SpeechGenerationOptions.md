# SpeechGenerationOptions

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**input** | **String** | The text to generate audio for. The maximum length is 4096 characters. | 
**voice** | [**models::SpeechVoice**](SpeechVoice.md) |  | 
**response_format** | Option<**String**> | The audio output format for the spoken text. By default, the MP3 format will be used. | [optional][default to Mp3]
**speed** | Option<**f32**> | The speed of speech for generated audio. Values are valid in the range from 0.25 to 4.0, with 1.0 the default and higher values corresponding to faster speech. | [optional][default to 1.0]
**model** | Option<**String**> | The model to use for this text-to-speech request. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


