//! Quick start example - minimal code to get GPT-5 working
//! 
//! This is the absolute minimum code needed to use GPT-5
//! Run with: cargo run --example quick_start

use gpt5::{Gpt5Client, Gpt5Model};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Create client
    let client = Gpt5Client::new("your-api-key-here".to_string());
    
    // 2. Ask a question
    let response = client
        .simple(Gpt5Model::Gpt5Nano, "Hello, world!")
        .await?;
    
    // 3. Print response
    println!("{}", response);
    
    Ok(())
}
