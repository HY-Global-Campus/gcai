# AudioTranscriptionOptions

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**file** | **String** | The audio data to transcribe. This must be the binary content of a file in one of the supported media formats:  flac, mp3, mp4, mpeg, mpga, m4a, ogg, wav, webm. | 
**filename** | Option<**String**> | The optional filename or descriptive identifier to associate with with the audio data. | [optional]
**response_format** | Option<[**models::AudioTranscriptionFormat**](AudioTranscriptionFormat.md)> |  | [optional]
**language** | Option<**String**> | The primary spoken language of the audio data to be transcribed, supplied as a two-letter ISO-639-1 language code such as 'en' or 'fr'. Providing this known input language is optional but may improve the accuracy and/or latency of transcription. | [optional]
**prompt** | Option<**String**> | An optional hint to guide the model's style or continue from a prior audio segment. The written language of the prompt should match the primary spoken language of the audio data. | [optional]
**temperature** | Option<**f32**> | The sampling temperature, between 0 and 1. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic. If set to 0, the model will use log probability to automatically increase the temperature until certain thresholds are hit. | [optional]
**timestamp_granularities** | Option<[**Vec<models::AudioTranscriptionTimestampGranularity>**](AudioTranscriptionTimestampGranularity.md)> | The timestamp granularities to populate for this transcription. `response_format` must be set `verbose_json` to use timestamp granularities. Either or both of these options are supported: `word`, or `segment`. Note: There is no additional latency for segment timestamps, but generating word timestamps incurs additional latency. | [optional]
**model** | Option<**String**> | The model to use for this transcription request. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


