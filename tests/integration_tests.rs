//! Integration tests for the GPT-5 Rust client library
//!
//! These tests verify the complete functionality of the library including
//! serialization, deserialization, and API interactions.

use gpt5::{
    ContentType, FormatType, Gpt5Client, Gpt5Model, Gpt5RequestBuilder, OutputType,
    ReasoningEffort, Role, Status, Tool, VerbosityLevel,
};
use serde_json::json;

/// Test Gpt5Model enum functionality
#[test]
fn test_gpt5_model_serialization() {
    assert_eq!(Gpt5Model::Gpt5.as_str(), "gpt-5");
    assert_eq!(Gpt5Model::Gpt5Mini.as_str(), "gpt-5-mini");
    assert_eq!(Gpt5Model::Gpt5Nano.as_str(), "gpt-5-nano");

    let custom = Gpt5Model::Custom("gpt-5-custom".to_string());
    assert_eq!(custom.as_str(), "gpt-5-custom");
}

/// Test ReasoningEffort enum serialization and deserialization
#[test]
fn test_reasoning_effort_serialization() {
    let low = ReasoningEffort::Low;
    let serialized = serde_json::to_string(&low).unwrap();
    assert_eq!(serialized, "\"low\"");

    let deserialized: ReasoningEffort = serde_json::from_str(&serialized).unwrap();
    assert_eq!(deserialized, ReasoningEffort::Low);

    // Test unknown value
    let unknown = ReasoningEffort::Unknown("custom".to_string());
    let serialized = serde_json::to_string(&unknown).unwrap();
    assert_eq!(serialized, "\"custom\"");
}

/// Test VerbosityLevel enum serialization and deserialization
#[test]
fn test_verbosity_level_serialization() {
    let low = VerbosityLevel::Low;
    let serialized = serde_json::to_string(&low).unwrap();
    assert_eq!(serialized, "\"low\"");

    let deserialized: VerbosityLevel = serde_json::from_str(&serialized).unwrap();
    assert_eq!(deserialized, VerbosityLevel::Low);
}

/// Test OutputType enum serialization and deserialization
#[test]
fn test_output_type_serialization() {
    let message = OutputType::Message;
    let serialized = serde_json::to_string(&message).unwrap();
    assert_eq!(serialized, "\"message\"");

    let deserialized: OutputType = serde_json::from_str(&serialized).unwrap();
    assert_eq!(deserialized, OutputType::Message);

    let function_call = OutputType::FunctionCall;
    let serialized = serde_json::to_string(&function_call).unwrap();
    assert_eq!(serialized, "\"function_call\"");
}

/// Test ContentType enum serialization and deserialization
#[test]
fn test_content_type_serialization() {
    let output_text = ContentType::OutputText;
    let serialized = serde_json::to_string(&output_text).unwrap();
    assert_eq!(serialized, "\"output_text\"");

    let deserialized: ContentType = serde_json::from_str(&serialized).unwrap();
    assert_eq!(deserialized, ContentType::OutputText);
}

/// Test Status enum serialization and deserialization
#[test]
fn test_status_serialization() {
    let completed = Status::Completed;
    let serialized = serde_json::to_string(&completed).unwrap();
    assert_eq!(serialized, "\"completed\"");

    let deserialized: Status = serde_json::from_str(&serialized).unwrap();
    assert_eq!(deserialized, Status::Completed);

    let in_progress = Status::InProgress;
    let serialized = serde_json::to_string(&in_progress).unwrap();
    assert_eq!(serialized, "\"in_progress\"");
}

/// Test Role enum serialization and deserialization
#[test]
fn test_role_serialization() {
    let user = Role::User;
    let serialized = serde_json::to_string(&user).unwrap();
    assert_eq!(serialized, "\"user\"");

    let deserialized: Role = serde_json::from_str(&serialized).unwrap();
    assert_eq!(deserialized, Role::User);

    let assistant = Role::Assistant;
    let serialized = serde_json::to_string(&assistant).unwrap();
    assert_eq!(serialized, "\"assistant\"");
}

/// Test FormatType enum serialization and deserialization
#[test]
fn test_format_type_serialization() {
    let markdown = FormatType::Markdown;
    let serialized = serde_json::to_string(&markdown).unwrap();
    assert_eq!(serialized, "\"markdown\"");

    let deserialized: FormatType = serde_json::from_str(&serialized).unwrap();
    assert_eq!(deserialized, FormatType::Markdown);
}

/// Test Gpt5Client creation
#[test]
fn test_gpt5_client_creation() {
    let client = Gpt5Client::new("test-api-key".to_string());
    // Client should be created successfully
    assert!(!client.api_key.is_empty());
}

/// Test Gpt5Client with custom base URL
#[test]
fn test_gpt5_client_with_base_url() {
    let client = Gpt5Client::new("test-api-key".to_string())
        .with_base_url("https://custom-api.example.com".to_string());

    // The base_url should be updated
    assert_eq!(client.base_url, "https://custom-api.example.com");
}

/// Test Gpt5RequestBuilder basic functionality
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

/// Test Gpt5RequestBuilder with all parameters
#[test]
fn test_gpt5_request_builder_complete() {
    let weather_tool = Tool {
        tool_type: "function".to_string(),
        name: "get_weather".to_string(),
        description: "Get current weather".to_string(),
        parameters: json!({
            "type": "object",
            "properties": {
                "location": {"type": "string"}
            }
        }),
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

/// Test Gpt5RequestBuilder validation
#[test]
fn test_gpt5_request_builder_validation() {
    // Test with empty input (should trigger warning in validation)
    let request = Gpt5RequestBuilder::new(Gpt5Model::Gpt5Nano)
        .input("")
        .max_output_tokens(5) // Very low token count
        .build();

    assert_eq!(request.input, "");
    assert_eq!(request.max_output_tokens, Some(5));
}

/// Test Tool struct creation and serialization
#[test]
fn test_tool_creation() {
    let tool = Tool {
        tool_type: "function".to_string(),
        name: "test_function".to_string(),
        description: "A test function".to_string(),
        parameters: json!({
            "type": "object",
            "properties": {
                "param1": {"type": "string"}
            }
        }),
    };

    assert_eq!(tool.tool_type, "function");
    assert_eq!(tool.name, "test_function");
    assert_eq!(tool.description, "A test function");
}

/// Test Gpt5Request serialization
#[test]
fn test_gpt5_request_serialization() {
    let request = Gpt5RequestBuilder::new(Gpt5Model::Gpt5Nano)
        .input("Test input")
        .max_output_tokens(100)
        .build();

    let serialized = serde_json::to_string(&request).unwrap();
    let deserialized: gpt5::Gpt5Request = serde_json::from_str(&serialized).unwrap();

    assert_eq!(deserialized.model, request.model);
    assert_eq!(deserialized.input, request.input);
    assert_eq!(deserialized.max_output_tokens, request.max_output_tokens);
}

/// Test Gpt5Response deserialization with sample data
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

    let response: gpt5::Gpt5Response = serde_json::from_value(sample_response).unwrap();

    assert_eq!(response.id, Some("resp_123".to_string()));
    assert_eq!(response.object, Some("response".to_string()));
    assert_eq!(response.status, Some(Status::Completed));
    assert_eq!(response.model, Some("gpt-5-nano".to_string()));
    assert!(response.output.is_some());
    assert!(response.usage.is_some());
}

/// Test Gpt5Response text extraction
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

    let response: gpt5::Gpt5Response = serde_json::from_value(sample_response).unwrap();
    let text = response.text();

    assert_eq!(text, Some("Hello, world!".to_string()));
}

/// Test Gpt5Response function calls extraction
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

    let response: gpt5::Gpt5Response = serde_json::from_value(sample_response).unwrap();
    let function_calls = response.function_calls();

    assert_eq!(function_calls.len(), 1);
    assert_eq!(function_calls[0].name, Some("get_weather".to_string()));
    assert_eq!(
        function_calls[0].arguments,
        Some("{\"location\": \"Boston\"}".to_string())
    );
}

/// Test Gpt5Response completion status
#[test]
fn test_gpt5_response_completion_status() {
    let completed_response = json!({
        "status": "completed"
    });

    let response: gpt5::Gpt5Response = serde_json::from_value(completed_response).unwrap();
    assert!(response.is_completed());

    let incomplete_response = json!({
        "status": "incomplete"
    });

    let response: gpt5::Gpt5Response = serde_json::from_value(incomplete_response).unwrap();
    assert!(!response.is_completed());
}

/// Test Gpt5Response token usage
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

    let response: gpt5::Gpt5Response = serde_json::from_value(sample_response).unwrap();

    assert_eq!(response.total_tokens(), 15);
    assert_eq!(response.reasoning_tokens(), Some(3));
}

/// Test error response deserialization
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

    let error: gpt5::OpenAiError = serde_json::from_value(error_response).unwrap();

    assert_eq!(error.error.message, "Invalid API key");
    assert_eq!(error.error.error_type, "invalid_request_error");
    assert_eq!(error.error.param, Some("api_key".to_string()));
    assert_eq!(error.error.code, Some("invalid_api_key".to_string()));
}

/// Test builder method chaining
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

/// Test enum equality and partial equality
#[test]
fn test_enum_equality() {
    assert_eq!(ReasoningEffort::Low, ReasoningEffort::Low);
    assert_ne!(ReasoningEffort::Low, ReasoningEffort::High);

    assert_eq!(VerbosityLevel::Medium, VerbosityLevel::Medium);
    assert_ne!(VerbosityLevel::Low, VerbosityLevel::High);

    assert_eq!(Status::Completed, Status::Completed);
    assert_ne!(Status::Completed, Status::InProgress);
}

/// Test unknown enum values handling
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

/// Test complex tool definition
#[test]
fn test_complex_tool_definition() {
    let complex_tool = Tool {
        tool_type: "function".to_string(),
        name: "analyze_data".to_string(),
        description: "Analyze complex data with multiple parameters".to_string(),
        parameters: json!({
            "type": "object",
            "properties": {
                "data": {
                    "type": "array",
                    "items": {"type": "number"},
                    "description": "Array of numbers to analyze"
                },
                "method": {
                    "type": "string",
                    "enum": ["mean", "median", "mode"],
                    "description": "Analysis method to use"
                },
                "options": {
                    "type": "object",
                    "properties": {
                        "include_stats": {"type": "boolean"},
                        "confidence_level": {"type": "number", "minimum": 0, "maximum": 1}
                    }
                }
            },
            "required": ["data", "method"]
        }),
    };

    assert_eq!(complex_tool.name, "analyze_data");
    assert!(complex_tool.parameters.is_object());
}

/// Test request with multiple tools
#[test]
fn test_multiple_tools() {
    let tool1 = Tool {
        tool_type: "function".to_string(),
        name: "tool1".to_string(),
        description: "First tool".to_string(),
        parameters: json!({}),
    };

    let tool2 = Tool {
        tool_type: "function".to_string(),
        name: "tool2".to_string(),
        description: "Second tool".to_string(),
        parameters: json!({}),
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
