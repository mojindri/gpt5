//! Basic usage example for the GPT-5 Rust client
//! 
//! This example shows the simplest way to use the GPT-5 client
//! Run with: cargo run --example basic_usage

use gpt5::{Gpt5Client, Gpt5Model};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the client with your API key
    let api_key = std::env::var("OPENAI_API_KEY")
        .expect("Please set OPENAI_API_KEY environment variable");
    
    let client = Gpt5Client::new(api_key);
    
    // Simple text generation
    println!("🤖 Asking GPT-5 Nano a simple question...");
    let response = client
        .simple(Gpt5Model::Gpt5Nano, "What is the capital of France?")
        .await?;
    
    println!("Response: {}", response);
    
    // Try different models
    println!("\n🤖 Asking GPT-5 Mini...");
    let response = client
        .simple(Gpt5Model::Gpt5Mini, "Explain quantum computing in simple terms")
        .await?;
    
    println!("Response: {}", response);
    
    // Try the main GPT-5 model
    println!("\n🤖 Asking GPT-5 (main model)...");
    let response = client
        .simple(Gpt5Model::Gpt5, "Write a short poem about coding")
        .await?;
    
    println!("Response: {}", response);
    
    Ok(())
}
