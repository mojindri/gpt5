# GPT-5 Rust Client Library

[![Crates.io](https://img.shields.io/crates/v/gpt5.svg)](https://crates.io/crates/gpt5)
[![Documentation](https://docs.rs/gpt5/badge.svg)](https://docs.rs/gpt5)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

> ⚠️ **IN ACTIVE IMPROVEMENT** ⚠️  
> This library is actively being improved and may have breaking changes.  
> Perfect for experimentation, learning, and development projects!
> 
> **Latest Release**: v0.1.1 - All tests passing, improved documentation, and better error handling!

A comprehensive Rust client library for OpenAI's GPT-5 API with full support for function calling, reasoning capabilities, and type-safe enums.

## Features

### 🚀 **Core Capabilities**
- **Type-safe API** - All parameters use strongly-typed enums for compile-time safety
- **Function calling** - Full support for OpenAI's function calling system with custom tools
- **Reasoning capabilities** - Configurable reasoning effort levels (Low, Medium, High)
- **Verbosity control** - Fine-tune response detail levels for different use cases
- **Multiple models** - Support for GPT-5, GPT-5 Mini, GPT-5 Nano, and custom models

### ⚡ **Performance & Developer Experience**
- **Async/await** - Built on tokio for high performance and concurrency
- **Error handling** - Comprehensive error types and validation with helpful messages
- **Response parsing** - Easy access to text, function calls, and metadata
- **Request builder** - Fluent API for building complex requests
- **Validation** - Built-in request validation with helpful warnings

### 📚 **Documentation & Examples**
- **Comprehensive examples** - 5 practical examples from basic to advanced
- **Interactive chat** - Ready-to-run chat loop example
- **Function calling demos** - Calculator and weather tool examples
- **Error handling patterns** - Production-ready error handling examples
- **Quick start guide** - Get running in minutes with minimal code

### 🔮 **Coming Soon**
- **Streaming responses** - Real-time response streaming for better UX
- **Retry mechanisms** - Automatic retry with exponential backoff
- **Rate limiting** - Built-in rate limiting and quota management
- **Response caching** - Optional response caching for cost optimization
- **WebSocket support** - Real-time bidirectional communication
- **More examples** - Advanced use cases and integration patterns
- **CLI tool** - Command-line interface for quick testing
- **Benchmarks** - Performance benchmarks and optimization guides

## Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
gpt5 = "0.1.1"
tokio = { version = "1.0", features = ["rt-multi-thread", "macros"] }
serde_json = "1.0"  # For function calling examples
```

### 🚀 Try the Examples

The fastest way to get started is with our examples:

```bash
# Clone and run examples
git clone <repository-url>
cd gpt5
cargo run --example quick_start
cargo run --example basic_usage
cargo run --example simple_chat
```

See the [examples/](examples/) directory for more detailed examples including function calling, error handling, and interactive chat.

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

## Examples

We provide comprehensive examples to help you get started quickly:

| Example | Description | Run Command |
|---------|-------------|-------------|
| [`quick_start.rs`](examples/quick_start.rs) | Minimal 3-line example | `cargo run --example quick_start` |
| [`basic_usage.rs`](examples/basic_usage.rs) | Different models demo | `cargo run --example basic_usage` |
| [`simple_chat.rs`](examples/simple_chat.rs) | Interactive chat loop | `cargo run --example simple_chat` |
| [`function_calling.rs`](examples/function_calling.rs) | Advanced function calling | `cargo run --example function_calling` |
| [`error_handling.rs`](examples/error_handling.rs) | Production error handling | `cargo run --example error_handling` |

### Prerequisites for Examples

Set your OpenAI API key:
```bash
export OPENAI_API_KEY="your-api-key-here"
```

## Contributing

🚀 **We're actively looking for contributors!** This is a fresh library with lots of room for improvement.

**Areas where we'd love help:**
- 🐛 Bug fixes and edge case handling
- 📚 Documentation improvements
- 🧪 More comprehensive tests
- ⚡ Performance optimizations
- 🔧 Additional features and examples
- 📖 Better error messages and validation

**How to contribute:**
1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a Pull Request

**Questions or ideas?** Open an issue and let's discuss! We're very responsive and would love to hear from you.

Contributions are welcome! Please feel free to submit a Pull Request.

## Changelog

### 0.1.1 (Latest)
- ✅ **All 94 tests passing** (28 unit + 26 integration + 40 doctests)
- 🔄 **Updated reqwest** dependency from 0.11 to 0.12
- 🛠️ **Fixed error handling** example with proper error types
- 📚 **Improved documentation** with realistic JSON examples
- 🧪 **Added no_run flags** to prevent real API calls in doctests
- ✨ **Better error messages** and handling patterns
- 🚀 **All examples compile** and run correctly

### 0.1.0
- Initial release
- Full GPT-5 API support
- Function calling capabilities
- Type-safe enums for all parameters
- Comprehensive documentation
- Complete examples directory with 5 practical examples