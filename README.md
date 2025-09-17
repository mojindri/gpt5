# GPT-5 Rust Client Library

[![Crates.io](https://img.shields.io/crates/v/gpt5.svg)](https://crates.io/crates/gpt5)
[![Documentation](https://docs.rs/gpt5/badge.svg)](https://docs.rs/gpt5)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A comprehensive Rust client library for OpenAI's GPT-5 API with full support for function calling, reasoning capabilities, and type-safe enums.

## Features

- 🚀 **Type-safe API** - All parameters use strongly-typed enums
- 🔧 **Function calling** - Full support for OpenAI's function calling system
- 🧠 **Reasoning capabilities** - Configurable reasoning effort levels
- 📝 **Verbosity control** - Fine-tune response detail levels
- ⚡ **Async/await** - Built on tokio for high performance
- 🛡️ **Error handling** - Comprehensive error types and validation
- 📊 **Response parsing** - Easy access to text, function calls, and metadata

## Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
gpt5 = "0.1.0"
tokio = { version = "1.0", features = ["rt-multi-thread", "macros"] }
```

### Basic Usage

```rust
use gpt5::{Gpt5Client, Gpt5Model};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Gpt5Client::new("your-api-key".to_string());
    
    let response = client
        .simple(Gpt5Model::Gpt5Nano, "Hello, world!")
        .await?;
    
    println!("Response: {}", response);
    Ok(())
}
```

### Advanced Usage with Function Calling

```rust
use gpt5::{Gpt5Client, Gpt5Model, Gpt5RequestBuilder, Tool, VerbosityLevel, ReasoningEffort};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Gpt5Client::new("your-api-key".to_string());
    
    // Define a weather tool
    let weather_tool = Tool {
        tool_type: "function".to_string(),
        name: "get_current_weather".to_string(),
        description: "Get the current weather in a given location".to_string(),
        parameters: json!({
            "type": "object",
            "properties": {
                "location": {
                    "type": "string",
                    "description": "The city and state, e.g. San Francisco, CA"
                },
                "unit": {
                    "type": "string",
                    "enum": ["celsius", "fahrenheit"]
                }
            },
            "required": ["location", "unit"]
        }),
    };
    
    // Build a request with tools
    let request = Gpt5RequestBuilder::new(Gpt5Model::Gpt5)
        .input("What's the weather like in Boston today?")
        .instructions("Use the weather tool to get current conditions")
        .tools(vec![weather_tool])
        .tool_choice("auto")
        .verbosity(VerbosityLevel::Medium)
        .reasoning_effort(ReasoningEffort::Medium)
        .max_output_tokens(500)
        .build();
    
    // Send the request
    let response = client.request(request).await?;
    
    // Check for function calls
    let function_calls = response.function_calls();
    if !function_calls.is_empty() {
        println!("Function calls made: {}", function_calls.len());
        for call in function_calls {
            println!("Function: {}", call.name.as_deref().unwrap_or("unknown"));
            println!("Arguments: {}", call.arguments.as_deref().unwrap_or("{}"));
        }
    }
    
    // Get text response
    if let Some(text) = response.text() {
        println!("Response: {}", text);
    }
    
    Ok(())
}
```

## API Reference

### Models

The library supports all GPT-5 models:

```rust
use gpt5::Gpt5Model;

let model = Gpt5Model::Gpt5;        // Main model - most capable
let mini = Gpt5Model::Gpt5Mini;     // Balanced performance and cost
let nano = Gpt5Model::Gpt5Nano;     // Fastest and most cost-effective
let custom = Gpt5Model::Custom("gpt-5-custom".to_string());
```

### Reasoning Effort

Control how much computational effort GPT-5 puts into reasoning:

```rust
use gpt5::ReasoningEffort;

let low = ReasoningEffort::Low;     // Fast, basic reasoning
let medium = ReasoningEffort::Medium; // Balanced performance
let high = ReasoningEffort::High;   // Thorough analysis
```

### Verbosity Levels

Control the detail level of responses:

```rust
use gpt5::VerbosityLevel;

let low = VerbosityLevel::Low;      // Concise responses
let medium = VerbosityLevel::Medium; // Balanced detail
let high = VerbosityLevel::High;    // Detailed responses
```

### Response Status

Check response completion and status:

```rust
let response = client.request(request).await?;

if response.is_completed() {
    println!("Response completed successfully");
    if let Some(text) = response.text() {
        println!("Text: {}", text);
    }
} else {
    println!("Response incomplete: {:?}", response.status);
}

// Get usage statistics
println!("Total tokens: {}", response.total_tokens());
if let Some(reasoning_tokens) = response.reasoning_tokens() {
    println!("Reasoning tokens: {}", reasoning_tokens);
}
```

## Error Handling

The library provides comprehensive error handling:

```rust
use gpt5::{Gpt5Client, Gpt5Model};

match client.simple(Gpt5Model::Gpt5Nano, "Hello").await {
    Ok(response) => println!("Success: {}", response),
    Err(e) => {
        match e.downcast_ref::<reqwest::Error>() {
            Some(req_err) => println!("Network error: {}", req_err),
            None => println!("Other error: {}", e),
        }
    }
}
```

## Validation

The library includes built-in validation for requests:

```rust
let request = Gpt5RequestBuilder::new(Gpt5Model::Gpt5Nano)
    .input("")  // Empty input will trigger a warning
    .max_output_tokens(5)  // Very low token count will trigger a warning
    .build();  // Validation runs automatically
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Changelog

### 0.1.0
- Initial release
- Full GPT-5 API support
- Function calling capabilities
- Type-safe enums for all parameters
- Comprehensive documentation