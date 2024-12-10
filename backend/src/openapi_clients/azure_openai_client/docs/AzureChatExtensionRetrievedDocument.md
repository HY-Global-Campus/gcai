# AzureChatExtensionRetrievedDocument

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**content** | **String** | The content of the citation. | 
**title** | Option<**String**> | The title of the citation. | [optional]
**url** | Option<**String**> | The URL of the citation. | [optional]
**filepath** | Option<**String**> | The file path of the citation. | [optional]
**chunk_id** | Option<**String**> | The chunk ID of the citation. | [optional]
**rerank_score** | Option<**f64**> | The rerank score of the retrieved document. | [optional]
**search_queries** | **Vec<String>** | The search queries used to retrieve the document. | 
**data_source_index** | **i32** | The index of the data source. | 
**original_search_score** | Option<**f64**> | The original search score of the retrieved document. | [optional]
**filter_reason** | Option<[**models::AzureChatExtensionRetrieveDocumentFilterReason**](AzureChatExtensionRetrieveDocumentFilterReason.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


