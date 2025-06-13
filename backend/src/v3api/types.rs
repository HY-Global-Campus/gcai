use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;
use std::collections::HashMap;
use uuid::Uuid;

/// Options shared by both function and direct-model calls.
#[derive(Serialize, Deserialize, Debug)]
#[skip_serializing_none]
pub struct RequestOptions {
    pub additional_tools: Option<Vec<ToolDefinition>>,
    pub allowed_tools: Option<Vec<String>>,
    pub cache_options: Option<CacheOptions>,
    pub credentials: Option<HashMap<String, String>>,
    pub dryrun: Option<bool>,
    pub episode_id: Option<Uuid>,
    pub extra_body: Option<Vec<ExtraBodyEntry>>,
    pub extra_headers: Option<Vec<ExtraHeaderEntry>>,
    pub include_original_response: Option<bool>,
    pub output_schema: Option<Value>,
    pub params: Option<HashMap<String, Value>>,
    pub stream: Option<bool>,
    pub tags: Option<HashMap<String, String>>,
    pub tool_choice: Option<String>,
    pub variant_name: Option<String>,
    pub parallel_tool_calls: Option<bool>,
}

/// The core inference request, either a function call or a direct model call.
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum InferenceRequest {
    Function {
        #[serde(flatten)]
        options: RequestOptions,
        function_name: String,
        input: InferenceInput,
    },
    Model {
        #[serde(flatten)]
        options: RequestOptions,
        model_name: String,
        input: InferenceInput,
    },
}

/// Input payload when calling a chat or JSON function.
#[derive(Serialize, Deserialize, Debug)]
#[skip_serializing_none]
pub struct InferenceInput {
    pub system: Option<Value>,
    pub messages: Vec<Message>,
}

/// A single chat message.
#[derive(Serialize, Deserialize, Debug)]
#[skip_serializing_none]
pub struct Message {
    pub role: String,
    pub content: MessageContent,
}

/// Enum for message content, including blocks.
#[derive(Serialize, Deserialize, Debug)]
#[skip_serializing_none]
#[serde(untagged)]
pub enum MessageContent {
    Text(String),
    Blocks(Vec<ContentBlockInput>),
}

/// Structured content blocks for inputs.
#[derive(Serialize, Deserialize, Debug)]
#[skip_serializing_none]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ContentBlockInput {
    Text {
        text: Option<Value>,
        arguments: Option<Value>,
    },
    ToolCall {
        id: String,
        name: String,
        arguments: Value,
    },
    ToolResult {
        id: String,
        name: String,
        result: String,
    },
    Image {
        url: Option<String>,
        mime_type: Option<String>,
        data: Option<String>,
    },
    RawText {
        value: String,
    },
    Unknown {
        data: Value,
        model_provider_name: Option<String>,
    },
}

/// Caching options.
#[derive(Serialize, Deserialize, Debug)]
#[skip_serializing_none]
pub struct CacheOptions {
    pub enabled: Option<CacheMode>,
    pub max_age_s: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum CacheMode {
    WriteOnly,
    ReadOnly,
    On,
    Off,
}

/// Dynamically provided tool definitions.
#[derive(Serialize, Deserialize, Debug)]
#[skip_serializing_none]
pub struct ToolDefinition {
    pub name: String,
    pub description: String,
    pub parameters: Value,
    pub strict: bool,
}

/// Body-patch entries.
#[derive(Serialize, Deserialize, Debug)]
#[skip_serializing_none]
pub struct ExtraBodyEntry {
    pub variant_name: Option<String>,
    pub model_provider_name: Option<String>,
    pub pointer: String,
    pub value: Value,
}

/// Header-patch entries.
#[derive(Serialize, Deserialize, Debug)]
#[skip_serializing_none]
pub struct ExtraHeaderEntry {
    pub variant_name: Option<String>,
    pub model_provider_name: Option<String>,
    pub name: String,
    pub value: String,
}

/// Usage metrics.
#[derive(Serialize, Deserialize, Debug)]
pub struct Usage {
    pub input_tokens: u64,
    pub output_tokens: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[skip_serializing_none]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum InferenceResponse {
    Chat(ChatFunctionResponse),
    Json(JsonFunctionResponse),
}

/// Response structure for chat functions.
#[derive(Serialize, Deserialize, Debug)]
#[skip_serializing_none]
pub struct ChatFunctionResponse {
    pub inference_id: Uuid,
    pub episode_id: Uuid,
    pub variant_name: String,
    pub original_response: Option<String>,
    pub content: Vec<ContentBlockOutput>,
    pub usage: Option<Usage>,
}

/// Structured content blocks for outputs.
#[derive(Serialize, Deserialize, Debug)]
#[skip_serializing_none]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ContentBlockOutput {
    Text {
        text: String,
    },
    ToolCall {
        id: String,
        name: String,
        raw_name: String,
        raw_arguments: String,
        arguments: Option<Value>,
    },
    Thought {
        id: String,
        text: String,
    },
    Unknown {
        data: Value,
        model_provider_name: String,
    },
}

/// Response structure for JSON functions.
#[derive(Serialize, Deserialize, Debug)]
#[skip_serializing_none]
pub struct JsonFunctionResponse {
    pub inference_id: Uuid,
    pub episode_id: Uuid,
    pub variant_name: String,
    pub original_response: Option<String>,
    pub output: JsonOutput,
    pub usage: Option<Usage>,
}

/// Output object for JSON functions.
#[derive(Serialize, Deserialize, Debug)]
#[skip_serializing_none]
pub struct JsonOutput {
    pub raw: String,
    pub parsed: Option<Value>,
}

/// Streaming chunk for chat and JSON responses.
#[derive(Serialize, Deserialize, Debug)]
#[skip_serializing_none]
pub struct StreamChunk<C> {
    pub inference_id: Uuid,
    pub episode_id: Uuid,
    pub variant_name: String,
    pub content: Vec<C>,
    pub usage: Option<Usage>,
}

/// Streaming chunk for JSON functions only.
#[derive(Serialize, Deserialize, Debug)]
#[skip_serializing_none]
pub struct JsonStreamChunk {
    pub inference_id: Uuid,
    pub episode_id: Uuid,
    pub raw: String,
    pub usage: Option<Usage>,
}
