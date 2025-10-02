//! Response structures for GPT-5 API

use crate::enums::{ContentType, FormatType, OutputType, ReasoningEffort, Role, Status};
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;

/// Response structure from /v1/responses
///
/// Contains the complete response from the GPT-5 API including
/// text content, function calls, usage statistics, and metadata.
///
/// # Examples
///
/// ```rust
/// use gpt5::Gpt5Response;
/// use serde_json::json;
///
/// // Example response data
/// let response_data = json!({
///     "id": "chatcmpl-123",
///     "object": "chat.completion",
///     "created": 1677652288,
///     "model": "gpt-5",
///     "status": "completed",
///     "output": [{
///         "type": "message",
///         "text": "Hello, world!"
///     }]
/// });
///
/// let response: Gpt5Response = serde_json::from_value(response_data).unwrap();
///
/// // Check if response is completed
/// if response.is_completed() {
///     if let Some(text) = response.text() {
///         println!("Response: {}", text);
///     }
/// }
/// ```
#[derive(Debug, Clone, Deserialize)]
pub struct Gpt5Response {
    /// Unique identifier assigned by OpenAI for this response instance
    pub id: Option<String>,
    /// Type of object returned (typically "response")
    pub object: Option<String>,
    /// Unix timestamp (seconds) when the response was created
    pub created_at: Option<u64>,
    /// Current status of the response lifecycle
    pub status: Option<Status>,
    /// Raw error payload when the request fails
    pub error: Option<Value>,
    /// Additional details explaining why a response may be incomplete
    pub incomplete_details: Option<Value>,
    /// System instructions that were used to generate this output
    pub instructions: Option<String>,
    /// Maximum tokens that were allowed for output generation
    pub max_output_tokens: Option<u32>,
    /// Model identifier that produced the response
    pub model: Option<String>,
    /// Collection of outputs such as messages and tool invocations
    pub output: Option<Vec<ResponseOutput>>,
    /// Indicates whether the model may issue multiple tool calls in parallel
    pub parallel_tool_calls: Option<bool>,
    /// Identifier referencing a prior response in a conversation chain
    pub previous_response_id: Option<String>,
    /// Metadata describing the model's reasoning effort and summary
    pub reasoning: Option<ResponseReasoning>,
    /// Whether OpenAI stored this response server-side
    pub store: Option<bool>,
    /// Aggregated text block supplied alongside the structured output array
    pub text: Option<ResponseText>,
    /// Tool selection strategy that was applied to this response
    pub tool_choice: Option<String>,
    /// Raw tool descriptors returned alongside the response
    pub tools: Option<Vec<Value>>,
    /// Top-p sampling value that was used for the request
    pub top_p: Option<f64>,
    /// Reason the response may have been truncated early, if any
    pub truncation: Option<String>,
    /// Token usage and cost accounting details
    pub usage: Option<ResponseUsage>,
    /// Identifier supplied by the caller in the original request
    pub user: Option<String>,
    /// Arbitrary key/value metadata returned by the API
    pub metadata: Option<HashMap<String, Value>>,
}

/// Output content in the response
///
/// Represents individual output items in the GPT-5 response,
/// which can be messages or function calls.
#[derive(Debug, Clone, Deserialize)]
pub struct ResponseOutput {
    /// Discriminator indicating whether this output is a message or tool call
    #[serde(rename = "type")]
    pub output_type: OutputType,
    /// Unique identifier for the output element
    pub id: Option<String>,
    /// Identifier linking tool call outputs to follow-up responses
    pub call_id: Option<String>,
    /// Function or tool name associated with this output
    pub name: Option<String>,
    /// JSON-encoded arguments supplied to a function call
    pub arguments: Option<String>,
    /// Execution status reported by the API for this output item
    pub status: Option<Status>,
    /// Message role (`user`, `assistant`, `tool`, etc.) when applicable
    pub role: Option<Role>,
    /// Rich content segments that make up the response message
    pub content: Option<Vec<OutputContent>>,
}

/// Content within an output message
///
/// Represents the actual content of a message output,
/// typically containing text or other media.
#[derive(Debug, Clone, Deserialize)]
pub struct OutputContent {
    /// Type of content (for example, `output_text`)
    #[serde(rename = "type")]
    pub content_type: ContentType,
    /// Textual data provided for text outputs
    pub text: Option<String>,
    /// Optional inline annotations such as citations or tool metadata
    pub annotations: Option<Vec<Value>>,
}

/// Reasoning information in the response
///
/// Contains details about the reasoning process used
/// to generate the response.
#[derive(Debug, Clone, Deserialize)]
pub struct ResponseReasoning {
    /// Reported level of reasoning effort spent on the request
    pub effort: Option<ReasoningEffort>,
    /// Natural language summary of the model's reasoning process
    pub summary: Option<String>,
}

/// Text formatting information
///
/// Specifies how the text content should be formatted.
#[derive(Debug, Clone, Deserialize)]
pub struct ResponseText {
    /// Formatting metadata describing how the client should render text output
    pub format: Option<ResponseTextFormat>,
}

/// Text format specification
///
/// Defines the specific format type for text content.
#[derive(Debug, Clone, Deserialize)]
pub struct ResponseTextFormat {
    /// Concrete text format such as Markdown or plain text
    #[serde(rename = "type")]
    pub format_type: FormatType,
}

/// Token usage statistics
///
/// Contains information about token usage for the request and response.
#[derive(Debug, Clone, Deserialize)]
pub struct ResponseUsage {
    /// Count of tokens consumed by input content
    pub input_tokens: u32,
    /// Detailed breakdown of input token consumption
    pub input_tokens_details: Option<InputTokenDetails>,
    /// Count of tokens generated in the output
    pub output_tokens: u32,
    /// Detailed breakdown of output token consumption
    pub output_tokens_details: Option<ResponseTokenDetails>,
    /// Total tokens billed for the interaction
    pub total_tokens: u32,
}

/// Input token details
///
/// Additional information about input token usage.
#[derive(Debug, Clone, Deserialize)]
pub struct InputTokenDetails {
    /// Number of tokens served from cache rather than newly processed
    pub cached_tokens: Option<u32>,
}

/// Output token details
///
/// Additional information about output token usage.
#[derive(Debug, Clone, Deserialize)]
pub struct ResponseTokenDetails {
    /// Tokens allocated specifically to reasoning traces
    pub reasoning_tokens: Option<u32>,
}

/// Error response structure from OpenAI
///
/// Represents error responses from the OpenAI API.
#[derive(Debug, Clone, Deserialize)]
pub struct OpenAiError {
    /// Structured error payload returned by the API
    pub error: OpenAiErrorDetails,
}

/// Error details
///
/// Contains specific information about an API error.
#[derive(Debug, Clone, Deserialize)]
pub struct OpenAiErrorDetails {
    /// Human-readable explanation of the failure
    pub message: String,
    /// Error category identifier provided by the API
    #[serde(rename = "type")]
    pub error_type: String,
    /// Parameter that triggered the error, if known
    pub param: Option<String>,
    /// Additional error code that can aid debugging
    pub code: Option<String>,
}

impl Gpt5Response {
    /// Extract text content from the response
    ///
    /// Returns the first text content found in the response output.
    ///
    /// # Returns
    ///
    /// * `Option<String>` - The text content if found, None otherwise
    ///
    /// # Examples
    ///
    /// ```rust
    /// use gpt5::Gpt5Response;
    /// use serde_json::json;
    ///
    /// let response_data = json!({
    ///     "output": [{ "type": "message", "text": "Hello, world!" }]
    /// });
    /// let response: Gpt5Response = serde_json::from_value(response_data).unwrap();
    ///
    /// if let Some(text) = response.text() {
    ///     println!("Response text: {}", text);
    /// }
    /// ```
    pub fn text(&self) -> Option<String> {
        // Look for text content in the output array
        if let Some(outputs) = &self.output {
            for output in outputs {
                if output.output_type == OutputType::Message {
                    if let Some(content_array) = &output.content {
                        for content in content_array {
                            if content.content_type == ContentType::OutputText {
                                if let Some(text) = &content.text {
                                    return Some(text.clone());
                                }
                            }
                        }
                    }
                }
            }
        }
        None
    }

    /// Extract all text content from the response
    ///
    /// Returns all text content found in the response output.
    ///
    /// # Returns
    ///
    /// * `Vec<String>` - Vector of all text content found
    ///
    /// # Examples
    ///
    /// ```rust
    /// use gpt5::Gpt5Response;
    /// use serde_json::json;
    ///
    /// let response_data = json!({
    ///     "output": [
    ///         { "type": "message", "text": "Hello" },
    ///         { "type": "message", "text": "World" }
    ///     ]
    /// });
    /// let response: Gpt5Response = serde_json::from_value(response_data).unwrap();
    ///
    /// let all_texts = response.all_text();
    /// for text in all_texts {
    ///     println!("Text: {}", text);
    /// }
    /// ```
    pub fn all_text(&self) -> Vec<String> {
        let mut texts = Vec::new();
        if let Some(outputs) = &self.output {
            for output in outputs {
                if output.output_type == OutputType::Message {
                    if let Some(content_array) = &output.content {
                        for content in content_array {
                            if content.content_type == ContentType::OutputText {
                                if let Some(text) = &content.text {
                                    texts.push(text.clone());
                                }
                            }
                        }
                    }
                }
            }
        }
        texts
    }

    /// Get function calls from the response
    ///
    /// Returns all function calls made in the response.
    ///
    /// # Returns
    ///
    /// * `Vec<&ResponseOutput>` - Vector of function call outputs
    ///
    /// # Examples
    ///
    /// ```rust
    /// use gpt5::Gpt5Response;
    /// use serde_json::json;
    ///
    /// let response_data = json!({
    ///     "output": [{
    ///         "type": "function_call",
    ///         "function_call": {
    ///             "name": "get_weather",
    ///             "arguments": "{\"city\": \"Boston\"}"
    ///         }
    ///     }]
    /// });
    /// let response: Gpt5Response = serde_json::from_value(response_data).unwrap();
    ///
    /// let function_calls = response.function_calls();
    /// for call in function_calls {
    ///     println!("Function: {:?}", call.name);
    /// }
    /// ```
    pub fn function_calls(&self) -> Vec<&ResponseOutput> {
        if let Some(outputs) = &self.output {
            outputs
                .iter()
                .filter(|output| output.output_type == OutputType::FunctionCall)
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Get reasoning token usage
    ///
    /// Returns the number of tokens used for reasoning.
    ///
    /// # Returns
    ///
    /// * `Option<u32>` - Number of reasoning tokens if available
    ///
    /// # Examples
    ///
    /// ```rust
    /// use gpt5::Gpt5Response;
    /// use serde_json::json;
    ///
    /// let response_data = json!({
    ///     "usage": {
    ///         "input_tokens": 10,
    ///         "output_tokens": 20,
    ///         "total_tokens": 30,
    ///         "output_tokens_details": {
    ///             "reasoning_tokens": 150
    ///         }
    ///     }
    /// });
    /// let response: Gpt5Response = serde_json::from_value(response_data).unwrap();
    ///
    /// if let Some(reasoning_tokens) = response.reasoning_tokens() {
    ///     println!("Reasoning tokens: {}", reasoning_tokens);
    /// }
    /// ```
    pub fn reasoning_tokens(&self) -> Option<u32> {
        self.usage
            .as_ref()
            .and_then(|usage| usage.output_tokens_details.as_ref())
            .and_then(|details| details.reasoning_tokens)
    }

    /// Get total token usage
    ///
    /// Returns the total number of tokens used.
    ///
    /// # Returns
    ///
    /// * `u32` - Total number of tokens used
    ///
    /// # Examples
    ///
    /// ```rust
    /// use gpt5::Gpt5Response;
    /// use serde_json::json;
    ///
    /// let response_data = json!({
    ///     "usage": {
    ///         "input_tokens": 10,
    ///         "output_tokens": 20,
    ///         "total_tokens": 100
    ///     }
    /// });
    /// let response: Gpt5Response = serde_json::from_value(response_data).unwrap();
    ///
    /// println!("Total tokens: {}", response.total_tokens());
    /// ```
    pub fn total_tokens(&self) -> u32 {
        self.usage.as_ref().map(|u| u.total_tokens).unwrap_or(0)
    }

    /// Check if the response is completed
    ///
    /// Returns true if the response status is "completed".
    ///
    /// # Returns
    ///
    /// * `bool` - True if completed, false otherwise
    ///
    /// # Examples
    ///
    /// ```rust
    /// use gpt5::Gpt5Response;
    /// use serde_json::json;
    ///
    /// let response_data = json!({
    ///     "status": "completed"
    /// });
    /// let response: Gpt5Response = serde_json::from_value(response_data).unwrap();
    ///
    /// if response.is_completed() {
    ///     println!("Response completed successfully");
    /// } else {
    ///     println!("Response is still processing");
    /// }
    /// ```
    pub fn is_completed(&self) -> bool {
        self.status
            .as_ref()
            .map(|s| *s == Status::Completed)
            .unwrap_or(false)
    }

    /// Check if the response has an error
    ///
    /// Returns true if the response contains an error.
    ///
    /// # Returns
    ///
    /// * `bool` - True if there's an error, false otherwise
    ///
    /// # Examples
    ///
    /// ```rust
    /// use gpt5::Gpt5Response;
    /// use serde_json::json;
    ///
    /// let response_data = json!({
    ///     "error": { "message": "API key invalid" }
    /// });
    /// let response: Gpt5Response = serde_json::from_value(response_data).unwrap();
    ///
    /// if response.has_error() {
    ///     println!("Response contains an error");
    /// }
    /// ```
    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }
}
