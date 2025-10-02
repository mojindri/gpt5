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
//! ```rust,no_run
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
//! ```rust,no_run
//! use gpt5::{Gpt5Client, Gpt5Model, Gpt5RequestBuilder, Tool};
//! use serde_json::json;
//!
//! let weather_tool = Tool {
//!     tool_type: "function".to_string(),
//!     name: Some("get_weather".to_string()),
//!     description: Some("Get current weather".to_string()),
//!     parameters: Some(json!({
//!         "type": "object",
//!         "properties": {
//!             "location": {"type": "string", "description": "City name"}
//!         },
//!         "required": ["location"]
//!     })),
//! };
//!
//! let req = Gpt5RequestBuilder::new(Gpt5Model::Gpt5)
//!     .input("What's the weather in Boston?")
//!     .tools(vec![weather_tool])
//!     .tool_choice("auto")
//!     .build();
//! ```

// Module declarations
mod client;
mod enums;
mod models;
mod requests;
mod responses;

// Re-export all public types for easy access
pub use crate::client::Gpt5Client;
pub use crate::enums::{
    ContentType, FormatType, OutputType, ReasoningEffort, Role, Status, VerbosityLevel,
};
pub use crate::models::Gpt5Model;
pub use crate::requests::{
    Gpt5Request, Gpt5RequestBuilder, RequestReasoning, RequestText, Tool, WebSearchConfig,
};
pub use crate::responses::{
    Gpt5Response, InputTokenDetails, OpenAiError, OpenAiErrorDetails, OutputContent,
    ResponseOutput, ResponseReasoning, ResponseText, ResponseTextFormat, ResponseTokenDetails,
    ResponseUsage,
};

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_gpt5_model_as_str() {
        assert_eq!(Gpt5Model::Gpt5.as_str(), "gpt-5");
        assert_eq!(Gpt5Model::Gpt5Mini.as_str(), "gpt-5-mini");
        assert_eq!(Gpt5Model::Gpt5Nano.as_str(), "gpt-5-nano");
        assert_eq!(Gpt5Model::Custom("custom".to_string()).as_str(), "custom");
    }

    #[test]
    fn test_reasoning_effort_serialization() {
        let low = ReasoningEffort::Low;
        let serialized = serde_json::to_string(&low).unwrap();
        assert_eq!(serialized, "\"low\"");

        let deserialized: ReasoningEffort = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, ReasoningEffort::Low);
    }

    #[test]
    fn test_verbosity_level_serialization() {
        let high = VerbosityLevel::High;
        let serialized = serde_json::to_string(&high).unwrap();
        assert_eq!(serialized, "\"high\"");

        let deserialized: VerbosityLevel = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, VerbosityLevel::High);
    }

    #[test]
    fn test_output_type_serialization() {
        let message = OutputType::Message;
        let serialized = serde_json::to_string(&message).unwrap();
        assert_eq!(serialized, "\"message\"");

        let deserialized: OutputType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, OutputType::Message);
    }

    #[test]
    fn test_content_type_serialization() {
        let output_text = ContentType::OutputText;
        let serialized = serde_json::to_string(&output_text).unwrap();
        assert_eq!(serialized, "\"output_text\"");

        let deserialized: ContentType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, ContentType::OutputText);
    }

    #[test]
    fn test_status_serialization() {
        let completed = Status::Completed;
        let serialized = serde_json::to_string(&completed).unwrap();
        assert_eq!(serialized, "\"completed\"");

        let deserialized: Status = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, Status::Completed);
    }

    #[test]
    fn test_role_serialization() {
        let user = Role::User;
        let serialized = serde_json::to_string(&user).unwrap();
        assert_eq!(serialized, "\"user\"");

        let deserialized: Role = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, Role::User);
    }

    #[test]
    fn test_format_type_serialization() {
        let markdown = FormatType::Markdown;
        let serialized = serde_json::to_string(&markdown).unwrap();
        assert_eq!(serialized, "\"markdown\"");

        let deserialized: FormatType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, FormatType::Markdown);
    }

    #[test]
    fn test_gpt5_client_creation() {
        let client = Gpt5Client::new("test-api-key".to_string());
        // Client should be created successfully
        assert!(!client.api_key.is_empty());
    }

    #[test]
    fn test_gpt5_client_with_base_url() {
        let client = Gpt5Client::new("test-api-key".to_string())
            .with_base_url("https://custom-api.example.com".to_string());

        // The base_url should be updated
        assert_eq!(client.base_url, "https://custom-api.example.com");
    }

    #[test]
    fn test_gpt5_request_builder_basic() {
        let request = Gpt5RequestBuilder::new(Gpt5Model::Gpt5Nano)
            .input("Hello, world!")
            .build();

        assert_eq!(request.model, "gpt-5-nano");
        assert_eq!(request.input, "Hello, world!");
        assert!(request.reasoning.is_none());
        assert!(request.tools.is_none());
        assert!(request.tool_choice.is_none());
        assert!(request.max_output_tokens.is_none());
        assert!(request.top_p.is_none());
        assert!(request.text.is_none());
        assert!(request.instructions.is_none());
    }

    #[test]
    fn test_gpt5_request_builder_complete() {
        let weather_tool = Tool {
            tool_type: "function".to_string(),
            name: Some("get_weather".to_string()),
            description: Some("Get current weather".to_string()),
            parameters: Some(json!({
                "type": "object",
                "properties": {
                    "location": {"type": "string"}
                }
            })),
        };

        let request = Gpt5RequestBuilder::new(Gpt5Model::Gpt5)
            .input("What's the weather?")
            .instructions("Use the weather tool")
            .reasoning_effort(ReasoningEffort::Medium)
            .verbosity(VerbosityLevel::High)
            .tools(vec![weather_tool])
            .tool_choice("auto")
            .max_output_tokens(1000)
            .top_p(0.9)
            .build();

        assert_eq!(request.model, "gpt-5");
        assert_eq!(request.input, "What's the weather?");
        assert_eq!(
            request.instructions,
            Some("Use the weather tool".to_string())
        );
        assert!(request.reasoning.is_some());
        assert!(request.tools.is_some());
        assert_eq!(request.tool_choice, Some("auto".to_string()));
        assert_eq!(request.max_output_tokens, Some(1000));
        assert_eq!(request.top_p, Some(0.9));
        assert!(request.text.is_some());
    }

    #[test]
    fn test_gpt5_request_serialization() {
        let request = Gpt5RequestBuilder::new(Gpt5Model::Gpt5Nano)
            .input("Test input")
            .max_output_tokens(100)
            .build();

        let serialized = serde_json::to_string(&request).unwrap();
        let deserialized: Gpt5Request = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized.model, request.model);
        assert_eq!(deserialized.input, request.input);
        assert_eq!(deserialized.max_output_tokens, request.max_output_tokens);
    }

    #[test]
    fn test_gpt5_response_deserialization() {
        let sample_response = json!({
            "id": "resp_123",
            "object": "response",
            "created_at": 1234567890,
            "status": "completed",
            "model": "gpt-5-nano",
            "output": [
                {
                    "type": "message",
                    "id": "msg_123",
                    "content": [
                        {
                            "type": "output_text",
                            "text": "Hello, world!"
                        }
                    ]
                }
            ],
            "usage": {
                "input_tokens": 10,
                "output_tokens": 5,
                "total_tokens": 15
            }
        });

        let response: Gpt5Response = serde_json::from_value(sample_response).unwrap();

        assert_eq!(response.id, Some("resp_123".to_string()));
        assert_eq!(response.object, Some("response".to_string()));
        assert_eq!(response.status, Some(Status::Completed));
        assert_eq!(response.model, Some("gpt-5-nano".to_string()));
        assert!(response.output.is_some());
        assert!(response.usage.is_some());
    }

    #[test]
    fn test_gpt5_response_text_extraction() {
        let sample_response = json!({
            "output": [
                {
                    "type": "message",
                    "content": [
                        {
                            "type": "output_text",
                            "text": "Hello, world!"
                        }
                    ]
                }
            ]
        });

        let response: Gpt5Response = serde_json::from_value(sample_response).unwrap();
        let text = response.text();

        assert_eq!(text, Some("Hello, world!".to_string()));
    }

    #[test]
    fn test_gpt5_response_function_calls() {
        let sample_response = json!({
            "output": [
                {
                    "type": "function_call",
                    "name": "get_weather",
                    "arguments": "{\"location\": \"Boston\"}"
                },
                {
                    "type": "message",
                    "content": [
                        {
                            "type": "output_text",
                            "text": "I'll check the weather for you."
                        }
                    ]
                }
            ]
        });

        let response: Gpt5Response = serde_json::from_value(sample_response).unwrap();
        let function_calls = response.function_calls();

        assert_eq!(function_calls.len(), 1);
        assert_eq!(function_calls[0].name, Some("get_weather".to_string()));
        assert_eq!(
            function_calls[0].arguments,
            Some("{\"location\": \"Boston\"}".to_string())
        );
    }

    #[test]
    fn test_gpt5_response_completion_status() {
        let completed_response = json!({
            "status": "completed"
        });

        let response: Gpt5Response = serde_json::from_value(completed_response).unwrap();
        assert!(response.is_completed());

        let incomplete_response = json!({
            "status": "incomplete"
        });

        let response: Gpt5Response = serde_json::from_value(incomplete_response).unwrap();
        assert!(!response.is_completed());
    }

    #[test]
    fn test_gpt5_response_token_usage() {
        let sample_response = json!({
            "usage": {
                "input_tokens": 10,
                "output_tokens": 5,
                "total_tokens": 15,
                "output_tokens_details": {
                    "reasoning_tokens": 3
                }
            }
        });

        let response: Gpt5Response = serde_json::from_value(sample_response).unwrap();

        assert_eq!(response.total_tokens(), 15);
        assert_eq!(response.reasoning_tokens(), Some(3));
    }

    #[test]
    fn test_error_response_deserialization() {
        let error_response = json!({
            "error": {
                "message": "Invalid API key",
                "type": "invalid_request_error",
                "param": "api_key",
                "code": "invalid_api_key"
            }
        });

        let error: OpenAiError = serde_json::from_value(error_response).unwrap();

        assert_eq!(error.error.message, "Invalid API key");
        assert_eq!(error.error.error_type, "invalid_request_error");
        assert_eq!(error.error.param, Some("api_key".to_string()));
        assert_eq!(error.error.code, Some("invalid_api_key".to_string()));
    }

    #[test]
    fn test_builder_method_chaining() {
        let request = Gpt5RequestBuilder::new(Gpt5Model::Gpt5)
            .input("Test")
            .instructions("Be helpful")
            .reasoning_effort(ReasoningEffort::High)
            .verbosity(VerbosityLevel::Medium)
            .max_output_tokens(500)
            .top_p(0.8)
            .param("custom_param", "custom_value")
            .build();

        assert_eq!(request.input, "Test");
        assert_eq!(request.instructions, Some("Be helpful".to_string()));
        assert!(request.reasoning.is_some());
        assert!(request.text.is_some());
        assert_eq!(request.max_output_tokens, Some(500));
        assert_eq!(request.top_p, Some(0.8));
        assert!(request.parameters.contains_key("custom_param"));
    }

    #[test]
    fn test_enum_equality() {
        assert_eq!(ReasoningEffort::Low, ReasoningEffort::Low);
        assert_ne!(ReasoningEffort::Low, ReasoningEffort::High);

        assert_eq!(VerbosityLevel::Medium, VerbosityLevel::Medium);
        assert_ne!(VerbosityLevel::Low, VerbosityLevel::High);

        assert_eq!(Status::Completed, Status::Completed);
        assert_ne!(Status::Completed, Status::InProgress);
    }

    #[test]
    fn test_unknown_enum_values() {
        let unknown_reasoning = ReasoningEffort::Unknown("custom_effort".to_string());
        let serialized = serde_json::to_string(&unknown_reasoning).unwrap();
        assert_eq!(serialized, "\"custom_effort\"");

        let deserialized: ReasoningEffort = serde_json::from_str("\"custom_effort\"").unwrap();
        assert_eq!(
            deserialized,
            ReasoningEffort::Unknown("custom_effort".to_string())
        );
    }

    #[test]
    fn test_tool_creation() {
        let tool = Tool {
            tool_type: "function".to_string(),
            name: Some("test_function".to_string()),
            description: Some("A test function".to_string()),
            parameters: Some(json!({
                "type": "object",
                "properties": {
                    "param1": {"type": "string"}
                }
            })),
        };

        assert_eq!(tool.tool_type, "function");
        assert_eq!(tool.name.as_deref(), Some("test_function"));
        assert_eq!(tool.description.as_deref(), Some("A test function"));
    }

    #[test]
    fn test_multiple_tools() {
        let tool1 = Tool {
            tool_type: "function".to_string(),
            name: Some("tool1".to_string()),
            description: Some("First tool".to_string()),
            parameters: Some(json!({})),
        };

        let tool2 = Tool {
            tool_type: "function".to_string(),
            name: Some("tool2".to_string()),
            description: Some("Second tool".to_string()),
            parameters: Some(json!({})),
        };

        let request = Gpt5RequestBuilder::new(Gpt5Model::Gpt5)
            .input("Use both tools")
            .tools(vec![tool1, tool2])
            .tool_choice("auto")
            .build();

        assert!(request.tools.is_some());
        assert_eq!(request.tools.unwrap().len(), 2);
        assert_eq!(request.tool_choice, Some("auto".to_string()));
    }

    #[test]
    fn test_all_text_extraction() {
        let sample_response = json!({
            "output": [
                {
                    "type": "message",
                    "content": [
                        {
                            "type": "output_text",
                            "text": "First message"
                        }
                    ]
                },
                {
                    "type": "message",
                    "content": [
                        {
                            "type": "output_text",
                            "text": "Second message"
                        }
                    ]
                }
            ]
        });

        let response: Gpt5Response = serde_json::from_value(sample_response).unwrap();
        let all_texts = response.all_text();

        assert_eq!(all_texts.len(), 2);
        assert_eq!(all_texts[0], "First message");
        assert_eq!(all_texts[1], "Second message");
    }

    #[test]
    fn test_has_error() {
        let error_response = json!({
            "error": {"message": "Test error"}
        });

        let response: Gpt5Response = serde_json::from_value(error_response).unwrap();
        assert!(response.has_error());

        let success_response = json!({
            "status": "completed"
        });

        let response: Gpt5Response = serde_json::from_value(success_response).unwrap();
        assert!(!response.has_error());
    }

    #[test]
    fn test_user_text_alias() {
        let request = Gpt5RequestBuilder::new(Gpt5Model::Gpt5Nano)
            .user_text("Hello from user_text method")
            .build();

        assert_eq!(request.input, "Hello from user_text method");
    }

    #[test]
    fn test_validation_warnings() {
        // Test with empty input - should not panic but log warning
        let request = Gpt5RequestBuilder::new(Gpt5Model::Gpt5Nano)
            .input("")
            .max_output_tokens(5) // Very low token count
            .build();

        assert_eq!(request.input, "");
        assert_eq!(request.max_output_tokens, Some(5));
    }

    #[test]
    fn test_web_search_configuration() {
        let request = Gpt5RequestBuilder::new(Gpt5Model::Gpt5Nano)
            .input("Search something")
            .web_search_enabled(true)
            .web_search_query("latest rust news")
            .web_search_max_results(5)
            .build();

        let tools = request.tools.expect("web_search tool should be present");
        let search_tool = tools
            .into_iter()
            .find(|tool| tool.tool_type == "web_search")
            .expect("expected a web_search tool");

        assert!(search_tool.name.is_none());
        assert!(search_tool.description.is_none());

        let config = request
            .web_search_config
            .expect("web_search metadata should be stored");
        assert_eq!(config.query.as_deref(), Some("latest rust news"));
        assert_eq!(config.max_results, Some(5));
    }

    #[test]
    fn test_web_search_disabled_is_omitted() {
        let request = Gpt5RequestBuilder::new(Gpt5Model::Gpt5Nano)
            .input("No search")
            .web_search_enabled(false)
            .build();

        assert!(request.tools.is_none());
        assert!(request.web_search_config.is_none());
    }

    #[test]
    fn test_client_with_custom_http_client() {
        let custom_client = reqwest::Client::builder()
            .build()
            .expect("should build client");
        let client = Gpt5Client::new("test-key".to_string()).with_http_client(custom_client);

        assert_eq!(client.api_key, "test-key");
    }
}
