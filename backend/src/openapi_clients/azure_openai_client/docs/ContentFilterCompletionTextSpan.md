# ContentFilterCompletionTextSpan

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**completion_start_offset** | **i32** | Offset of the UTF32 code point which begins the span. | 
**completion_end_offset** | **i32** | Offset of the first UTF32 code point which is excluded from the span.  This field is always equal to completion_start_offset for empty spans.  This field is always larger than completion_start_offset for non-empty spans. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


