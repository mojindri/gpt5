//! Request structures and builders for GPT-5 API

use crate::enums::{ReasoningEffort, VerbosityLevel};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

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
#[derive(Debug, Clone, Serialize, Deserialize)]
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

/// Reasoning configuration for requests
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestReasoning {
    pub effort: ReasoningEffort,
}

/// Text configuration for requests
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestText {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verbosity: Option<VerbosityLevel>,
}

/// Tool definition for function calling
///
/// Defines a tool that GPT-5 can call during its response generation.
///
/// # Examples
///
/// ```rust
/// use gpt5::Tool;
/// use serde_json::json;
///
/// let weather_tool = Tool {
///     tool_type: "function".to_string(),
///     name: "get_weather".to_string(),
///     description: "Get current weather".to_string(),
///     parameters: json!({
///         "type": "object",
///         "properties": {
///             "location": {"type": "string"}
///         }
///     }),
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tool {
    #[serde(rename = "type")]
    pub tool_type: String,
    pub name: String,
    pub description: String,
    pub parameters: Value,
}

/// Builder for GPT-5 requests using /v1/responses
///
/// Provides a fluent interface for building GPT-5 requests with validation.
///
/// # Examples
///
/// ```rust
/// use gpt5::{Gpt5RequestBuilder, Gpt5Model, VerbosityLevel, ReasoningEffort};
///
/// let request = Gpt5RequestBuilder::new(Gpt5Model::Gpt5)
///     .input("What's the weather?")
///     .instructions("Use the weather tool")
///     .reasoning_effort(ReasoningEffort::Medium)
///     .verbosity(VerbosityLevel::High)
///     .max_output_tokens(1000)
///     .build();
/// ```
pub struct Gpt5RequestBuilder {
    model: crate::models::Gpt5Model,
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
    /// Create a new request builder with the specified model
    ///
    /// # Arguments
    ///
    /// * `model` - The GPT-5 model to use for the request
    ///
    /// # Examples
    ///
    /// ```rust
    /// use gpt5::{Gpt5RequestBuilder, Gpt5Model};
    ///
    /// let builder = Gpt5RequestBuilder::new(Gpt5Model::Gpt5Nano);
    /// ```
    pub fn new(model: crate::models::Gpt5Model) -> Self {
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

    /// Set the input text for the request
    ///
    /// # Arguments
    ///
    /// * `text` - The input text to send to GPT-5
    ///
    /// # Examples
    ///
    /// ```rust
    /// use gpt5::{Gpt5RequestBuilder, Gpt5Model};
    ///
    /// let request = Gpt5RequestBuilder::new(Gpt5Model::Gpt5Nano)
    ///     .input("Hello, world!")
    ///     .build();
    /// ```
    pub fn input(mut self, text: &str) -> Self {
        self.input = text.to_string();
        self
    }

    /// Set the instructions for the request (alias for input)
    ///
    /// # Arguments
    ///
    /// * `text` - The instructions to send to GPT-5
    ///
    /// # Examples
    ///
    /// ```rust
    /// use gpt5::{Gpt5RequestBuilder, Gpt5Model};
    ///
    /// let request = Gpt5RequestBuilder::new(Gpt5Model::Gpt5Nano)
    ///     .user_text("Hello, world!")
    ///     .build();
    /// ```
    pub fn user_text(self, text: &str) -> Self {
        self.input(text)
    }

    /// Set the system instructions for the request
    ///
    /// # Arguments
    ///
    /// * `instructions` - The system instructions to guide GPT-5's behavior
    ///
    /// # Examples
    ///
    /// ```rust
    /// use gpt5::{Gpt5RequestBuilder, Gpt5Model};
    ///
    /// let request = Gpt5RequestBuilder::new(Gpt5Model::Gpt5Nano)
    ///     .instructions("Be helpful and concise")
    ///     .input("What's the weather?")
    ///     .build();
    /// ```
    pub fn instructions(mut self, instructions: &str) -> Self {
        self.instructions = Some(instructions.to_string());
        self
    }

    /// Set the tools for function calling
    ///
    /// # Arguments
    ///
    /// * `tools` - Vector of tools that GPT-5 can call
    ///
    /// # Examples
    ///
    /// ```rust
    /// use gpt5::{Gpt5RequestBuilder, Gpt5Model, Tool};
    /// use serde_json::json;
    ///
    /// let weather_tool = Tool {
    ///     tool_type: "function".to_string(),
    ///     name: "get_weather".to_string(),
    ///     description: "Get weather".to_string(),
    ///     parameters: json!({}),
    /// };
    ///
    /// let request = Gpt5RequestBuilder::new(Gpt5Model::Gpt5)
    ///     .tools(vec![weather_tool])
    ///     .build();
    /// ```
    pub fn tools(mut self, tools: Vec<Tool>) -> Self {
        self.tools = Some(tools);
        self
    }

    /// Set the tool choice strategy
    ///
    /// # Arguments
    ///
    /// * `choice` - Tool choice strategy ("auto", "none", or specific tool name)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use gpt5::{Gpt5RequestBuilder, Gpt5Model};
    ///
    /// let request = Gpt5RequestBuilder::new(Gpt5Model::Gpt5)
    ///     .tool_choice("auto")
    ///     .build();
    /// ```
    pub fn tool_choice(mut self, choice: &str) -> Self {
        self.tool_choice = Some(choice.to_string());
        self
    }

    /// Set the reasoning effort level
    ///
    /// # Arguments
    ///
    /// * `effort` - The reasoning effort level
    ///
    /// # Examples
    ///
    /// ```rust
    /// use gpt5::{Gpt5RequestBuilder, Gpt5Model, ReasoningEffort};
    ///
    /// let request = Gpt5RequestBuilder::new(Gpt5Model::Gpt5)
    ///     .reasoning_effort(ReasoningEffort::High)
    ///     .build();
    /// ```
    pub fn reasoning_effort(mut self, effort: ReasoningEffort) -> Self {
        self.reasoning = Some(RequestReasoning { effort });
        self
    }

    /// Set the verbosity level
    ///
    /// # Arguments
    ///
    /// * `level` - The verbosity level for responses
    ///
    /// # Examples
    ///
    /// ```rust
    /// use gpt5::{Gpt5RequestBuilder, Gpt5Model, VerbosityLevel};
    ///
    /// let request = Gpt5RequestBuilder::new(Gpt5Model::Gpt5)
    ///     .verbosity(VerbosityLevel::High)
    ///     .build();
    /// ```
    pub fn verbosity(mut self, level: VerbosityLevel) -> Self {
        self.text = Some(RequestText {
            verbosity: Some(level),
        });
        self
    }

    /// Set the maximum output tokens
    ///
    /// # Arguments
    ///
    /// * `tokens` - Maximum number of tokens to generate
    ///
    /// # Examples
    ///
    /// ```rust
    /// use gpt5::{Gpt5RequestBuilder, Gpt5Model};
    ///
    /// let request = Gpt5RequestBuilder::new(Gpt5Model::Gpt5)
    ///     .max_output_tokens(1000)
    ///     .build();
    /// ```
    pub fn max_output_tokens(mut self, tokens: u32) -> Self {
        self.max_output_tokens = Some(tokens);
        self
    }

    /// Set the top_p parameter
    ///
    /// # Arguments
    ///
    /// * `p` - Top-p sampling parameter (0.0 to 1.0)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use gpt5::{Gpt5RequestBuilder, Gpt5Model};
    ///
    /// let request = Gpt5RequestBuilder::new(Gpt5Model::Gpt5)
    ///     .top_p(0.9)
    ///     .build();
    /// ```
    pub fn top_p(mut self, p: f64) -> Self {
        self.top_p = Some(p);
        self
    }

    /// Add a custom parameter
    ///
    /// # Arguments
    ///
    /// * `key` - Parameter key
    /// * `value` - Parameter value (must implement Into<Value>)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use gpt5::{Gpt5RequestBuilder, Gpt5Model};
    ///
    /// let request = Gpt5RequestBuilder::new(Gpt5Model::Gpt5)
    ///     .param("custom_param", "custom_value")
    ///     .build();
    /// ```
    pub fn param<T: Into<Value>>(mut self, key: &str, value: T) -> Self {
        self.parameters.insert(key.to_string(), value.into());
        self
    }

    /// Build the request with validation
    ///
    /// Validates the request parameters and returns the built request.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use gpt5::{Gpt5RequestBuilder, Gpt5Model};
    ///
    /// let request = Gpt5RequestBuilder::new(Gpt5Model::Gpt5Nano)
    ///     .input("Hello!")
    ///     .build();
    /// ```
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

    /// Validate the request parameters
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
            if !(0.0..=1.0).contains(&top_p) {
                tracing::warn!(
                    "Gpt5RequestBuilder: top_p ({}) should be between 0.0 and 1.0",
                    top_p
                );
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
