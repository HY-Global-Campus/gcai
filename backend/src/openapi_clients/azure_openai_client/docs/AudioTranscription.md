# AudioTranscription

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**text** | **String** | The transcribed text for the provided audio data. | 
**task** | Option<[**models::AudioTaskLabel**](AudioTaskLabel.md)> |  | [optional]
**language** | Option<**String**> | The spoken language that was detected in the transcribed audio data. This is expressed as a two-letter ISO-639-1 language code like 'en' or 'fr'. | [optional]
**duration** | Option<**f32**> | The total duration of the audio processed to produce accompanying transcription information. | [optional]
**segments** | Option<[**Vec<models::AudioTranscriptionSegment>**](AudioTranscriptionSegment.md)> | A collection of information about the timing, probabilities, and other detail of each processed audio segment. | [optional]
**words** | Option<[**Vec<models::AudioTranscriptionWord>**](AudioTranscriptionWord.md)> | A collection of information about the timing of each processed word. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


