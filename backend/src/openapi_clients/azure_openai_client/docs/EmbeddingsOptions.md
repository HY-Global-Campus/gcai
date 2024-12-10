# EmbeddingsOptions

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**user** | Option<**String**> | An identifier for the caller or end user of the operation. This may be used for tracking or rate-limiting purposes. | [optional]
**model** | Option<**String**> | The model name to provide as part of this embeddings request. Not applicable to Azure OpenAI, where deployment information should be included in the Azure resource URI that's connected to. | [optional]
**input** | **Vec<String>** | Input texts to get embeddings for, encoded as a an array of strings. Each input must not exceed 2048 tokens in length.  Unless you are embedding code, we suggest replacing newlines (\\n) in your input with a single space, as we have observed inferior results when newlines are present. | 
**encoding_format** | Option<**String**> | The response encoding format to use for embedding data. | [optional][default to Float]
**dimensions** | Option<**i32**> | The number of dimensions the resulting output embeddings should have. Only supported in `text-embedding-3` and later models. | [optional]
**input_type** | Option<**String**> | When using Azure OpenAI, specifies the input type to use for embedding search. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


