# AudioTranscriptionSegment

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** | The 0-based index of this segment within a transcription. | 
**start** | **f32** | The time at which this segment started relative to the beginning of the transcribed audio. | 
**end** | **f32** | The time at which this segment ended relative to the beginning of the transcribed audio. | 
**text** | **String** | The transcribed text that was part of this audio segment. | 
**temperature** | **f32** | The temperature score associated with this audio segment. | 
**avg_logprob** | **f32** | The average log probability associated with this audio segment. | 
**compression_ratio** | **f32** | The compression ratio of this audio segment. | 
**no_speech_prob** | **f32** | The probability of no speech detection within this audio segment. | 
**tokens** | **Vec<i32>** | The token IDs matching the transcribed text in this audio segment. | 
**seek** | **i32** | The seek position associated with the processing of this audio segment. Seek positions are expressed as hundredths of seconds. The model may process several segments from a single seek position, so while the seek position will never represent a later time than the segment's start, the segment's start may represent a significantly later time than the segment's associated seek position. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


