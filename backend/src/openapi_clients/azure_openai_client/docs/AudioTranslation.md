# AudioTranslation

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**text** | **String** | The translated text for the provided audio data. | 
**task** | Option<[**models::AudioTaskLabel**](AudioTaskLabel.md)> |  | [optional]
**language** | Option<**String**> | The spoken language that was detected in the translated audio data. This is expressed as a two-letter ISO-639-1 language code like 'en' or 'fr'. | [optional]
**duration** | Option<**f32**> | The total duration of the audio processed to produce accompanying translation information. | [optional]
**segments** | Option<[**Vec<models::AudioTranslationSegment>**](AudioTranslationSegment.md)> | A collection of information about the timing, probabilities, and other detail of each processed audio segment. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


