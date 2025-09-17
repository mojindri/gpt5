//! Simple chat example with GPT-5
//!
//! This example demonstrates a basic chat loop
//! Run with: cargo run --example simple_chat

use gpt5::{Gpt5Client, Gpt5Model};
use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key =
        std::env::var("OPENAI_API_KEY").expect("Please set OPENAI_API_KEY environment variable");

    let client = Gpt5Client::new(api_key);

    println!("🤖 GPT-5 Chat Bot");
    println!("Type 'quit' or 'exit' to end the conversation\n");

    loop {
        print!("You: ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        if input == "quit" || input == "exit" {
            println!("Goodbye! 👋");
            break;
        }

        if input.is_empty() {
            continue;
        }

        println!("🤖 Thinking...");

        match client.simple(Gpt5Model::Gpt5Nano, input).await {
            Ok(response) => {
                println!("Bot: {}\n", response);
            }
            Err(e) => {
                println!("❌ Error: {}\n", e);
            }
        }
    }

    Ok(())
}
