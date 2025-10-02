# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

- Nothing yet.

## [0.2.3] - 2025-10-03

### Documentation
- Added field-level documentation for `Gpt5Client`, request payload structs, and response models to improve generated crate docs and contextual guidance

## [0.2.2] - 2025-10-02

### Changed
- Align `Gpt5RequestBuilder` web search support with the Responses API by emitting a bare `web_search` tool while retaining configuration metadata for host applications
- Simplified the web search example and documentation to use a lightweight release-notes query and a longer default HTTP timeout

### Fixed
- Prevent `400 Bad Request` errors caused by unsupported `tools[].name` and `tools[].description` parameters in web search requests

## [0.2.1] - 2025-09-26

### Changed
- Disabled `reqwest` default features to guarantee `rustls` TLS support and shrink the default dependency surface
- Refreshed dependency lockfile to pull in the latest compatible patch releases

### Documentation
- Updated README with the 0.2.1 installation instructions and latest release details

## [0.2.0] - 2025-09-26

### Added
- Optional web search assistance controls in `Gpt5RequestBuilder` including query overrides and result limits
- `Gpt5Client::with_http_client` for supplying a preconfigured `reqwest::Client`
- `web_search` example demonstrating live search configuration and custom HTTP clients

### Changed
- Hardened API calls with explicit HTTP status checks and richer error messages
- Default HTTP client now includes a 60s timeout with graceful fallback if the builder fails

## [0.1.1] - 2024-12-19

### Added
- Comprehensive test suite with 94 tests (28 unit + 26 integration + 40 doctests)
- Better error handling patterns in examples
- Realistic JSON examples in documentation

### Changed
- Updated reqwest dependency from 0.11 to 0.12
- Improved documentation with proper JSON structure
- Enhanced error messages and handling patterns

### Fixed
- Fixed error handling example with proper error types
- Added no_run flags to prevent real API calls in doctests
- Ensured all examples compile and run correctly
- Fixed all doctest compilation issues

### Technical
- All 94 tests now pass without failures
- Improved code coverage and reliability
- Better developer experience with working examples

## [0.1.0] - 2024-12-19

### Added
- Initial release of GPT-5 Rust client library
- Full GPT-5 API support with type-safe enums
- Function calling capabilities with custom tools
- Reasoning capabilities with configurable effort levels
- Verbosity control for response detail levels
- Multiple model support (GPT-5, GPT-5 Mini, GPT-5 Nano, Custom)
- Async/await support built on tokio
- Comprehensive error handling and validation
- Request builder with fluent API
- Response parsing for text, function calls, and metadata
- Complete examples directory with 5 practical examples:
  - `quick_start.rs` - Minimal 3-line example
  - `basic_usage.rs` - Different models demo
  - `simple_chat.rs` - Interactive chat loop
  - `function_calling.rs` - Advanced function calling
  - `error_handling.rs` - Production error handling
- Interactive chat example
- Function calling demos with calculator and weather tools
- Error handling patterns for production use
- Quick start guide for immediate usage

### Features
- Type-safe API with strongly-typed enums
- Function calling system with custom tools
- Reasoning effort levels (Low, Medium, High)
- Verbosity levels (Low, Medium, High)
- Multiple GPT-5 models support
- Async/await for high performance
- Comprehensive error types and validation
- Easy response parsing
- Request builder pattern
- Built-in validation with helpful warnings
