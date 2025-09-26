# GPT-5 Examples

This directory contains simple, practical examples showing how to use the GPT-5 Rust client library.

## Prerequisites

1. Set your OpenAI API key as an environment variable:
   ```bash
   export OPENAI_API_KEY="your-api-key-here"
   ```

2. Make sure you have the required dependencies in your `Cargo.toml`:
   ```toml
   [dependencies]
   gpt5 = "0.1.0"
   tokio = { version = "1.0", features = ["rt-multi-thread", "macros"] }
   serde_json = "1.0"
   ```

## Examples

### 1. Basic Usage (`basic_usage.rs`)
The simplest way to get started with GPT-5. Shows how to:
- Initialize the client
- Make simple requests
- Use different models (Nano, Mini, Main)

```bash
cargo run --example basic_usage
```

### 2. Simple Chat (`simple_chat.rs`)
A basic interactive chat loop. Perfect for:
- Testing the API
- Building simple chatbots
- Understanding basic request/response flow

```bash
cargo run --example simple_chat
```

### 3. Function Calling (`function_calling.rs`)
Demonstrates advanced features like:
- Defining custom tools/functions
- Function calling capabilities
- Tool choice configuration
- Verbosity and reasoning controls

```bash
cargo run --example function_calling
```

### 4. Error Handling (`error_handling.rs`)
Shows proper error handling patterns:
- Invalid API keys
- Network errors
- Input validation
- Error type checking

```bash
cargo run --example error_handling
```

### 5. Web Search Assistance (`web_search.rs`)
Learn how to:
- Enable OpenAI's web search assistance
- Override the search query for better relevance
- Limit the maximum number of search results inspected
- Combine instructions and inputs for richer answers

```bash
cargo run --example web_search
```

## Running Examples

All examples can be run with:

```bash
# From the gpt5 directory
cargo run --example <example_name>

# Or from the project root
cargo run --example <example_name> --manifest-path gpt5/Cargo.toml
```

## Tips

- Start with `basic_usage.rs` to understand the fundamentals
- Use `simple_chat.rs` for interactive testing
- Check `error_handling.rs` for production-ready error handling
- Explore `function_calling.rs` for advanced features

## Need Help?

- Check the main [README](../README.md) for detailed documentation
- Look at the [integration tests](../tests/integration_tests.rs) for more complex examples
- Review the source code in the `src/` directory for implementation details
