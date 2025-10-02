//! Web search assistance example for the GPT-5 Rust client
//!
//! Demonstrates how to enable OpenAI's web search assistance and
//! optionally override the query or result limits.
//! Run with: cargo run --example web_search

use gpt5::{Gpt5Client, Gpt5Model, Gpt5RequestBuilder, OutputType, Status};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Pull the API key from the environment
    let api_key =
        std::env::var("OPENAI_API_KEY").expect("Please set OPENAI_API_KEY environment variable");

    // Configure a reqwest client with a shorter timeout to fail fast in demos.
    // You can customise this however you like (proxies, retries, etc.).
    let http_client = reqwest::Client::builder()
        .timeout(Duration::from_secs(90))
        .build()
        .expect("failed to build reqwest client");

    // Build the GPT-5 client with the custom HTTP client for extra control.
    let client = Gpt5Client::new(api_key).with_http_client(http_client);

    println!("🔎 Asking GPT-5 to perform a live web-assisted search...\n");

    let request = Gpt5RequestBuilder::new(Gpt5Model::Gpt5)
        .input("Summarise the newest Rust release notes")
        .web_search_enabled(true)
        .web_search_query("latest Rust release notes")
        .web_search_max_results(3)
        .build();

    let search_config = request.web_search_config.clone();
    let response = client.request(request).await?;

    match response.status {
        Some(Status::RequiresAction) => {
            if let Some(config) = search_config {
                println!(
                    "Suggested query: {} (max results: {:?})",
                    config.query.unwrap_or_default(),
                    config.max_results
                );
            }
            println!(
                "The model requested a tool call. You should run the search and submit tool_outputs."
            );
        }
        Some(Status::Completed) => {
            if let Some(text) = response.text() {
                println!("Response:\n{}", text);
            }
        }
        _ => {
            println!("Model status: {:?}", response.status);
        }
    }

    if let Some(outputs) = &response.output {
        for output in outputs {
            if let OutputType::Unknown(ref kind) = output.output_type {
                if kind == "tool_call" {
                    println!(
                        "Tool call requested: {} with args {}",
                        output.name.as_deref().unwrap_or("web_search"),
                        output.arguments.as_deref().unwrap_or("{}")
                    );
                }
            }
        }
    }

    if response.output.is_none() {
        println!(
            "No output yet; inspect the full payload for details: {:#?}",
            response
        );
    }

    Ok(())
}
