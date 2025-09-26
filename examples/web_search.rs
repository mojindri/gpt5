//! Web search assistance example for the GPT-5 Rust client
//!
//! Demonstrates how to enable OpenAI's web search assistance and
//! optionally override the query or result limits.
//! Run with: cargo run --example web_search

use gpt5::{Gpt5Client, Gpt5Model, Gpt5RequestBuilder};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Pull the API key from the environment
    let api_key =
        std::env::var("OPENAI_API_KEY").expect("Please set OPENAI_API_KEY environment variable");

    // Configure a reqwest client with a shorter timeout to fail fast in demos.
    // You can customise this however you like (proxies, retries, etc.).
    let http_client = reqwest::Client::builder()
        .timeout(Duration::from_secs(30))
        .build()
        .expect("failed to build reqwest client");

    // Build the GPT-5 client with the custom HTTP client for extra control.
    let client = Gpt5Client::new(api_key).with_http_client(http_client);

    println!("🔎 Asking GPT-5 to perform a live web-assisted search...\n");

    let request = Gpt5RequestBuilder::new(Gpt5Model::Gpt5)
        .instructions("Summarise the findings")
        .input("What are the latest announcements from the Rust language team?")
        .web_search_enabled(true)
        .web_search_query("Rust programming language roadmap 2025")
        .web_search_max_results(5)
        .build();

    let response = client.request(request).await?;

    if let Some(text) = response.text() {
        println!("Response:\n{}", text);
    } else {
        println!(
            "The response did not include text output. Full payload: {:#?}",
            response
        );
    }

    Ok(())
}
