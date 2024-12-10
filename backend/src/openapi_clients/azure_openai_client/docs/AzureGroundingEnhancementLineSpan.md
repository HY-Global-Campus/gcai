# AzureGroundingEnhancementLineSpan

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**text** | **String** | The text content of the span that represents the detected object. | 
**offset** | **i32** | The character offset within the text where the span begins. This offset is defined as the position of the first character of the span, counting from the start of the text as Unicode codepoints. | 
**length** | **i32** | The length of the span in characters, measured in Unicode codepoints. | 
**polygon** | [**Vec<models::AzureGroundingEnhancementCoordinatePoint>**](AzureGroundingEnhancementCoordinatePoint.md) | An array of objects representing points in the polygon that encloses the detected object. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


