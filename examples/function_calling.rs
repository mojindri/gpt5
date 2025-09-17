//! Function calling example with GPT-5
//! 
//! This example shows how to use GPT-5 with function calling capabilities
//! Run with: cargo run --example function_calling

use gpt5::{Gpt5Client, Gpt5Model, Gpt5RequestBuilder, Tool, VerbosityLevel, ReasoningEffort};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("OPENAI_API_KEY")
        .expect("Please set OPENAI_API_KEY environment variable");
    
    let client = Gpt5Client::new(api_key);
    
    // Define a simple calculator tool
    let calculator_tool = Tool {
        tool_type: "function".to_string(),
        name: "calculate".to_string(),
        description: "Perform basic arithmetic calculations".to_string(),
        parameters: json!({
            "type": "object",
            "properties": {
                "expression": {
                    "type": "string",
                    "description": "Mathematical expression to evaluate (e.g., '2 + 2', '10 * 5')"
                }
            },
            "required": ["expression"]
        }),
    };
    
    // Define a weather tool (mock)
    let weather_tool = Tool {
        tool_type: "function".to_string(),
        name: "get_weather".to_string(),
        description: "Get current weather information for a city".to_string(),
        parameters: json!({
            "type": "object",
            "properties": {
                "city": {
                    "type": "string",
                    "description": "The city name to get weather for"
                }
            },
            "required": ["city"]
        }),
    };
    
    println!("🧮 Testing calculator function...");
    
    // Build a request with tools
    let request = Gpt5RequestBuilder::new(Gpt5Model::Gpt5Nano)
        .input("What is 15 * 8 + 42?")
        .instructions("Use the calculator tool to solve this math problem")
        .tools(vec![calculator_tool])
        .tool_choice("auto")
        .verbosity(VerbosityLevel::Medium)
        .reasoning_effort(ReasoningEffort::Low)
        .max_output_tokens(200)
        .build();
    
    // Send the request
    let response = client.request(request).await?;
    
    // Check for function calls
    let function_calls = response.function_calls();
    if !function_calls.is_empty() {
        println!("🔧 Function calls made: {}", function_calls.len());
        for call in function_calls {
            println!("  Function: {}", call.name.as_deref().unwrap_or("unknown"));
            println!("  Arguments: {}", call.arguments.as_deref().unwrap_or("{}"));
        }
    }
    
    // Get text response
    if let Some(text) = response.text() {
        println!("🤖 Response: {}", text);
    }
    
    println!("\n🌤️ Testing weather function...");
    
    // Test weather tool
    let weather_request = Gpt5RequestBuilder::new(Gpt5Model::Gpt5Nano)
        .input("What's the weather like in Tokyo?")
        .instructions("Use the weather tool to get current conditions")
        .tools(vec![weather_tool])
        .tool_choice("auto")
        .verbosity(VerbosityLevel::Low)
        .reasoning_effort(ReasoningEffort::Low)
        .max_output_tokens(150)
        .build();
    
    let weather_response = client.request(weather_request).await?;
    
    let weather_calls = weather_response.function_calls();
    if !weather_calls.is_empty() {
        println!("🔧 Weather function calls made: {}", weather_calls.len());
        for call in weather_calls {
            println!("  Function: {}", call.name.as_deref().unwrap_or("unknown"));
            println!("  Arguments: {}", call.arguments.as_deref().unwrap_or("{}"));
        }
    }
    
    if let Some(text) = weather_response.text() {
        println!("🤖 Response: {}", text);
    }
    
    Ok(())
}
