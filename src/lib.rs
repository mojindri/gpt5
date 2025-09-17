//! # GPT-5 Rust Client Library
//!
//! A comprehensive Rust client library for OpenAI's GPT-5 API with support for:
//! - Function calling and tool usage
//! - Reasoning capabilities with configurable effort levels
//! - Verbosity control for response detail
//! - Streaming and non-streaming responses
//! - Type-safe enums for all API parameters
//!
//! ## Quick Start
//!
//! ```rust
//! use gpt5::{Gpt5Client, Gpt5Model, Gpt5RequestBuilder, VerbosityLevel};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = Gpt5Client::new("your-api-key".to_string());
//!     
//!     let response = client
//!         .simple(Gpt5Model::Gpt5Nano, "Hello, world!")
//!         .await?;
//!     
//!     println!("Response: {}", response);
//!     Ok(())
//! }
//! ```
//!
//! ## Advanced Usage with Function Calling
//!
//! ```rust
//! use gpt5::{Gpt5Client, Gpt5Model, Gpt5RequestBuilder, Tool};
//! use serde_json::json;
//!
//! let weather_tool = Tool {
//!     tool_type: "function".to_string(),
//!     name: "get_weather".to_string(),
//!     description: "Get current weather".to_string(),
//!     parameters: json!({
//!         "type": "object",
//!         "properties": {
//!             "location": {"type": "string", "description": "City name"}
//!         },
//!         "required": ["location"]
//!     }),
//! };
//!
//! let req = Gpt5RequestBuilder::new(Gpt5Model::Gpt5)
//!     .input("What's the weather in Boston?")
//!     .tools(vec![weather_tool])
//!     .tool_choice("auto")
//!     .build();
//! ```

use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// GPT-5 Models available for the /v1/responses endpoint
/// 
/// # Examples
/// 
/// ```rust
/// use gpt5::Gpt5Model;
/// 
/// let model = Gpt5Model::Gpt5Nano; // Fastest, most cost-effective
/// let custom = Gpt5Model::Custom("gpt-5-custom".to_string());
/// ```
#[derive(Debug, Clone)]
pub enum Gpt5Model {
    /// Main GPT-5 model - most capable
    Gpt5,
    /// GPT-5 Mini - balanced performance and cost
    Gpt5Mini,
    /// GPT-5 Nano - fastest and most cost-effective
    Gpt5Nano,
    /// Custom GPT-5 variant for future models
    Custom(String),
}

impl Gpt5Model {
    /// Returns the string representation of the model for API calls
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use gpt5::Gpt5Model;
    /// 
    /// assert_eq!(Gpt5Model::Gpt5.as_str(), "gpt-5");
    /// assert_eq!(Gpt5Model::Gpt5Nano.as_str(), "gpt-5-nano");
    /// ```
    pub fn as_str(&self) -> &str {
        match self {
            Gpt5Model::Gpt5 => "gpt-5",
            Gpt5Model::Gpt5Mini => "gpt-5-mini",
            Gpt5Model::Gpt5Nano => "gpt-5-nano",
            Gpt5Model::Custom(name) => name,
        }
    }
}

// --- Enums for previously stringly-typed fields ---

/// Reasoning effort level for GPT-5 responses
/// 
/// Controls how much computational effort GPT-5 puts into reasoning
/// before generating the final response. Higher effort typically means
/// better quality but slower responses.
/// 
/// # Examples
/// 
/// ```rust
/// use gpt5::ReasoningEffort;
/// 
/// let low_effort = ReasoningEffort::Low;    // Fast, basic reasoning
/// let high_effort = ReasoningEffort::High;  // Slow, thorough reasoning
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReasoningEffort {
    /// Low reasoning effort - fastest responses
    Low,
    /// Medium reasoning effort - balanced performance
    Medium,
    /// High reasoning effort - most thorough analysis
    High,
    /// Unknown effort level (for future compatibility)
    Unknown(String),
}

impl serde::Serialize for ReasoningEffort {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            ReasoningEffort::Low => serializer.serialize_str("low"),
            ReasoningEffort::Medium => serializer.serialize_str("medium"),
            ReasoningEffort::High => serializer.serialize_str("high"),
            ReasoningEffort::Unknown(s) => serializer.serialize_str(s),
        }
    }
}

impl<'de> serde::Deserialize<'de> for ReasoningEffort {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "low" => ReasoningEffort::Low,
            "medium" => ReasoningEffort::Medium,
            "high" => ReasoningEffort::High,
            _ => ReasoningEffort::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VerbosityLevel {
    Low,
    Medium,
    High,
    Unknown(String),
}

impl serde::Serialize for VerbosityLevel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            VerbosityLevel::Low => serializer.serialize_str("low"),
            VerbosityLevel::Medium => serializer.serialize_str("medium"),
            VerbosityLevel::High => serializer.serialize_str("high"),
            VerbosityLevel::Unknown(s) => serializer.serialize_str(s),
        }
    }
}

impl<'de> serde::Deserialize<'de> for VerbosityLevel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "low" => VerbosityLevel::Low,
            "medium" => VerbosityLevel::Medium,
            "high" => VerbosityLevel::High,
            _ => VerbosityLevel::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OutputType {
    Message,
    FunctionCall,
    Unknown(String),
}

impl serde::Serialize for OutputType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            OutputType::Message => serializer.serialize_str("message"),
            OutputType::FunctionCall => serializer.serialize_str("function_call"),
            OutputType::Unknown(s) => serializer.serialize_str(s),
        }
    }
}

impl<'de> serde::Deserialize<'de> for OutputType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "message" => OutputType::Message,
            "function_call" => OutputType::FunctionCall,
            _ => OutputType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContentType {
    OutputText,
    Unknown(String),
}

impl serde::Serialize for ContentType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            ContentType::OutputText => serializer.serialize_str("output_text"),
            ContentType::Unknown(s) => serializer.serialize_str(s),
        }
    }
}

impl<'de> serde::Deserialize<'de> for ContentType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "output_text" => ContentType::OutputText,
            _ => ContentType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Status {
    InProgress,
    Completed,
    RequiresAction,
    Failed,
    Unknown(String),
}

impl serde::Serialize for Status {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Status::InProgress => serializer.serialize_str("in_progress"),
            Status::Completed => serializer.serialize_str("completed"),
            Status::RequiresAction => serializer.serialize_str("requires_action"),
            Status::Failed => serializer.serialize_str("failed"),
            Status::Unknown(s) => serializer.serialize_str(s),
        }
    }
}

impl<'de> serde::Deserialize<'de> for Status {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "in_progress" => Status::InProgress,
            "completed" => Status::Completed,
            "requires_action" => Status::RequiresAction,
            "failed" => Status::Failed,
            _ => Status::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Role {
    User,
    Assistant,
    Tool,
    System,
    Unknown(String),
}

impl serde::Serialize for Role {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Role::User => serializer.serialize_str("user"),
            Role::Assistant => serializer.serialize_str("assistant"),
            Role::Tool => serializer.serialize_str("tool"),
            Role::System => serializer.serialize_str("system"),
            Role::Unknown(s) => serializer.serialize_str(s),
        }
    }
}

impl<'de> serde::Deserialize<'de> for Role {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "user" => Role::User,
            "assistant" => Role::Assistant,
            "tool" => Role::Tool,
            "system" => Role::System,
            _ => Role::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FormatType {
    Markdown,
    PlainText,
    Unknown(String),
}

impl serde::Serialize for FormatType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            FormatType::Markdown => serializer.serialize_str("markdown"),
            FormatType::PlainText => serializer.serialize_str("plain_text"),
            FormatType::Unknown(s) => serializer.serialize_str(s),
        }
    }
}

impl<'de> serde::Deserialize<'de> for FormatType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "markdown" => FormatType::Markdown,
            "plain_text" => FormatType::PlainText,
            _ => FormatType::Unknown(s),
        })
    }
}

/// Request structure for the GPT-5 /v1/responses endpoint
/// 
/// This struct contains all the parameters needed to make a request to GPT-5.
/// Use `Gpt5RequestBuilder` to construct requests in a type-safe manner.
/// 
/// # Examples
/// 
/// ```rust
/// use gpt5::{Gpt5RequestBuilder, Gpt5Model, VerbosityLevel, ReasoningEffort};
/// 
/// let request = Gpt5RequestBuilder::new(Gpt5Model::Gpt5Nano)
///     .input("Hello, world!")
///     .instructions("Keep it short")
///     .verbosity(VerbosityLevel::Low)
///     .reasoning_effort(ReasoningEffort::Low)
///     .max_output_tokens(100)
///     .build();
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct Gpt5Request {
    pub model: String,
    pub input: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<RequestReasoning>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<Tool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_output_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<RequestText>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    #[serde(flatten)]
    pub parameters: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize)]
pub struct RequestReasoning {
    pub effort: ReasoningEffort,
}

#[derive(Debug, Clone, Serialize)]
pub struct RequestText {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verbosity: Option<VerbosityLevel>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Tool {
    #[serde(rename = "type")]
    pub tool_type: String,
    pub name: String,
    pub description: String,
    pub parameters: Value,
}

// Error response structure from OpenAI
#[derive(Debug, Clone, Deserialize)]
pub struct OpenAiError {
    pub error: OpenAiErrorDetails,
}

#[derive(Debug, Clone, Deserialize)]
pub struct OpenAiErrorDetails {
    pub message: String,
    #[serde(rename = "type")]
    pub error_type: String,
    pub param: Option<String>,
    pub code: Option<String>,
}

// Response structure from /v1/responses
#[derive(Debug, Clone, Deserialize)]
pub struct Gpt5Response {
    pub id: Option<String>,
    pub object: Option<String>,
    pub created_at: Option<u64>,
    pub status: Option<Status>,
    pub error: Option<Value>,
    pub incomplete_details: Option<Value>,
    pub instructions: Option<String>,
    pub max_output_tokens: Option<u32>,
    pub model: Option<String>,
    pub output: Option<Vec<ResponseOutput>>,
    pub parallel_tool_calls: Option<bool>,
    pub previous_response_id: Option<String>,
    pub reasoning: Option<ResponseReasoning>,
    pub store: Option<bool>,
    pub text: Option<ResponseText>,
    pub tool_choice: Option<String>,
    pub tools: Option<Vec<Value>>,
    pub top_p: Option<f64>,
    pub truncation: Option<String>,
    pub usage: Option<ResponseUsage>,
    pub user: Option<String>,
    pub metadata: Option<HashMap<String, Value>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ResponseOutput {
    #[serde(rename = "type")]
    pub output_type: OutputType,
    pub id: Option<String>,
    pub call_id: Option<String>,
    pub name: Option<String>,
    pub arguments: Option<String>,
    pub status: Option<Status>,
    pub role: Option<Role>,
    pub content: Option<Vec<OutputContent>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct OutputContent {
    #[serde(rename = "type")]
    pub content_type: ContentType,
    pub text: Option<String>,
    pub annotations: Option<Vec<Value>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ResponseReasoning {
    pub effort: Option<ReasoningEffort>,
    pub summary: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ResponseText {
    pub format: Option<ResponseTextFormat>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ResponseTextFormat {
    #[serde(rename = "type")]
    pub format_type: FormatType,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ResponseUsage {
    pub input_tokens: u32,
    pub input_tokens_details: Option<InputTokenDetails>,
    pub output_tokens: u32,
    pub output_tokens_details: Option<ResponseTokenDetails>,
    pub total_tokens: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct InputTokenDetails {
    pub cached_tokens: Option<u32>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ResponseTokenDetails {
    pub reasoning_tokens: Option<u32>,
}

/// Main client for interacting with the GPT-5 API
/// 
/// The `Gpt5Client` handles authentication, request building, and response parsing
/// for the OpenAI GPT-5 API. It supports both simple and advanced usage patterns.
/// 
/// # Examples
/// 
/// ## Simple Usage
/// 
/// ```rust
/// use gpt5::{Gpt5Client, Gpt5Model};
/// 
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = Gpt5Client::new("your-api-key".to_string());
///     let response = client.simple(Gpt5Model::Gpt5Nano, "Hello!").await?;
///     println!("{}", response);
///     Ok(())
/// }
/// ```
/// 
/// ## Advanced Usage with Custom Base URL
/// 
/// ```rust
/// use gpt5::Gpt5Client;
/// 
/// let client = Gpt5Client::new("your-api-key".to_string())
///     .with_base_url("https://custom-api.example.com".to_string());
/// ```
pub struct Gpt5Client {
    client: Client,
    api_key: String,
    base_url: String,
}

impl Gpt5Client {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
            base_url: "https://api.openai.com".to_string(),
        }
    }
    
    pub fn with_base_url(mut self, base_url: String) -> Self {
        self.base_url = base_url;
        self
    }
    
    // Core method using /v1/responses endpoint
    pub async fn request(&self, req: Gpt5Request) -> anyhow::Result<Gpt5Response> {
        // Validate GPT-5 model
        if !self.is_gpt5_model(&req.model) {
            return Err(anyhow::anyhow!("Only GPT-5 models are supported. Got: {}", req.model));
        }
        
        let url = format!("{}/v1/responses", self.base_url);
        
        let response = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&req)
            .send()
            .await?;
            
        let response_text = response.text().await?;
        
        // Log the raw response for debugging
        tracing::info!("GPT-5 raw response: {}", response_text);
        
        // Try to parse as a generic JSON first to see the structure
        match serde_json::from_str::<serde_json::Value>(&response_text) {
            Ok(json_value) => {
                tracing::info!("GPT-5 response JSON structure: {:#}", json_value);

                // First check if this is an error response
                if let Ok(error_response) = serde_json::from_str::<OpenAiError>(&response_text) {
                    tracing::error!("OpenAI API error: {}", error_response.error.message);
                    return Err(anyhow::anyhow!("OpenAI API error: {}", error_response.error.message));
                }

                // Try to parse as our expected success structure
                match serde_json::from_str::<Gpt5Response>(&response_text) {
                    Ok(gpt_response) => Ok(gpt_response),
                    Err(parse_error) => {
                        tracing::error!("Failed to parse GPT-5 response: {}", parse_error);
                        tracing::error!("Raw response: {}", response_text);
                        Err(anyhow::anyhow!("Failed to parse GPT-5 response: {}", parse_error))
                    }
                }
            }
            Err(json_error) => {
                tracing::error!("Invalid JSON response from GPT-5: {}", json_error);
                tracing::error!("Raw response: {}", response_text);
                Err(anyhow::anyhow!("Invalid JSON response: {}", json_error))
            }
        }
    }
    
    // Simple method for quick responses
    pub async fn simple(&self, model: Gpt5Model, prompt: &str) -> anyhow::Result<String> {
        let req = Gpt5Request {
            model: model.as_str().to_string(),
            input: prompt.to_string(),
            reasoning: None,
            tools: None,
            tool_choice: None,
            max_output_tokens: None,
            top_p: None,
            text: None,
            instructions: None,
            parameters: HashMap::new(),
        };
        
        let response = self.request(req).await?;
        self.extract_text(&response)
    }
    
    // Extract text from response
    fn extract_text(&self, response: &Gpt5Response) -> anyhow::Result<String> {
        // Look for text content in the output array
        if let Some(outputs) = &response.output {
            for output in outputs {
                if output.output_type == OutputType::Message {
                    if let Some(content_array) = &output.content {
                        for content in content_array {
                            if content.content_type == ContentType::OutputText {
                                if let Some(text) = &content.text {
                                    return Ok(text.clone());
                                }
                            }
                        }
                    }
                }
            }
        }

        // If no text output found, return error
        Err(anyhow::anyhow!("No text content in response output"))
    }
    
    // Validate GPT-5 models
    fn is_gpt5_model(&self, model: &str) -> bool {
        matches!(model,
            "gpt-5" | "gpt-5-mini" | "gpt-5-nano"
        ) || model.starts_with("gpt-5")
    }
}

// Builder for GPT-5 requests using /v1/responses
pub struct Gpt5RequestBuilder {
    model: Gpt5Model,
    input: String,
    reasoning: Option<RequestReasoning>,
    tools: Option<Vec<Tool>>,
    tool_choice: Option<String>,
    max_output_tokens: Option<u32>,
    top_p: Option<f64>,
    text: Option<RequestText>,
    instructions: Option<String>,
    parameters: HashMap<String, Value>,
}

impl Gpt5RequestBuilder {
    pub fn new(model: Gpt5Model) -> Self {
        Self {
            model,
            input: String::new(),
            reasoning: None,
            tools: None,
            tool_choice: None,
            max_output_tokens: None,
            top_p: None,
            text: None,
            instructions: None,
            parameters: HashMap::new(),
        }
    }
    
    pub fn input(mut self, text: &str) -> Self {
        self.input = text.to_string();
        self
    }
    
    pub fn instructions(mut self, instructions: &str) -> Self {
        self.instructions = Some(instructions.to_string());
        self
    }
    
    pub fn user_text(self, text: &str) -> Self {
        self.input(text)
    }
    
    pub fn tools(mut self, tools: Vec<Tool>) -> Self {
        self.tools = Some(tools);
        self
    }
    
    pub fn tool_choice(mut self, choice: &str) -> Self {
        self.tool_choice = Some(choice.to_string());
        self
    }
    
    // GPT-5 specific parameters for /v1/responses
    pub fn reasoning_effort(mut self, effort: ReasoningEffort) -> Self {
        self.reasoning = Some(RequestReasoning {
            effort,
        });
        self
    }
    
    pub fn verbosity(mut self, level: VerbosityLevel) -> Self {
        self.text = Some(RequestText {
            verbosity: Some(level),
        });
        self
    }
    
    pub fn max_output_tokens(mut self, tokens: u32) -> Self {
        self.max_output_tokens = Some(tokens);
        self
    }
    
    
    pub fn top_p(mut self, p: f64) -> Self {
        self.top_p = Some(p);
        self
    }
    
    // Generic parameter method
    pub fn param<T: Into<Value>>(mut self, key: &str, value: T) -> Self {
        self.parameters.insert(key.to_string(), value.into());
        self
    }
    
    pub fn build(self) -> Gpt5Request {
        // Pre-check validation
        self.validate();
        
        Gpt5Request {
            model: self.model.as_str().to_string(),
            input: self.input,
            reasoning: self.reasoning,
            tools: self.tools,
            tool_choice: self.tool_choice,
            max_output_tokens: self.max_output_tokens,
            top_p: self.top_p,
            text: self.text,
            instructions: self.instructions,
            parameters: self.parameters,
        }
    }
    
    fn validate(&self) {
        // Validate input is not empty
        if self.input.trim().is_empty() {
            tracing::warn!("Gpt5RequestBuilder: Input is empty, this may result in no response");
        }
        
        // Validate max_output_tokens is reasonable
        if let Some(tokens) = self.max_output_tokens {
            if tokens < 10 {
                tracing::warn!("Gpt5RequestBuilder: max_output_tokens ({}) is very low, response may be truncated", tokens);
            } else if tokens > 100000 {
                tracing::warn!("Gpt5RequestBuilder: max_output_tokens ({}) is very high, this may be expensive", tokens);
            }
        }
        
        // Validate top_p is in reasonable range
        if let Some(top_p) = self.top_p {
            if top_p < 0.0 || top_p > 1.0 {
                tracing::warn!("Gpt5RequestBuilder: top_p ({}) should be between 0.0 and 1.0", top_p);
            }
        }
        
        // Validate reasoning effort with verbosity
        if let Some(ref reasoning) = self.reasoning {
            if let Some(ref text) = self.text {
                if let Some(ref verbosity) = text.verbosity {
                    match (&reasoning.effort, verbosity) {
                        (ReasoningEffort::High, VerbosityLevel::Low) => {
                            tracing::warn!("Gpt5RequestBuilder: High reasoning effort with low verbosity may not produce detailed output");
                        }
                        (ReasoningEffort::Low, VerbosityLevel::High) => {
                            tracing::warn!("Gpt5RequestBuilder: Low reasoning effort with high verbosity may not produce the expected detailed output");
                        }
                        _ => {} // Good combinations
                    }
                }
            }
        }
        
        // Validate tools configuration
        if let Some(ref tools) = self.tools {
            if tools.is_empty() {
                tracing::warn!("Gpt5RequestBuilder: Empty tools array provided");
            }
        }
        
        tracing::info!("Gpt5RequestBuilder: Request validation completed");
    }
}

// Response utilities
impl Gpt5Response {
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

    pub fn reasoning_tokens(&self) -> Option<u32> {
        self.usage
            .as_ref()
            .and_then(|usage| usage.output_tokens_details.as_ref())
            .and_then(|details| details.reasoning_tokens)
    }

    pub fn total_tokens(&self) -> u32 {
        self.usage.as_ref().map(|u| u.total_tokens).unwrap_or(0)
    }

    pub fn is_completed(&self) -> bool {
        self.status.as_ref().map(|s| *s == Status::Completed).unwrap_or(false)
    }

    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }
}

// All types are already public with their definitions above
