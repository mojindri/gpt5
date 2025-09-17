//! Error handling example for GPT-5 client
//! 
//! This example demonstrates proper error handling patterns
//! Run with: cargo run --example error_handling

use gpt5::{Gpt5Client, Gpt5Model, Gpt5Error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Test with invalid API key
    println!("🔑 Testing with invalid API key...");
    let invalid_client = Gpt5Client::new("invalid-key".to_string());
    
    match invalid_client.simple(Gpt5Model::Gpt5Nano, "Hello").await {
        Ok(response) => println!("Unexpected success: {}", response),
        Err(e) => {
            println!("❌ Expected error: {}", e);
            if let Some(gpt5_error) = e.downcast_ref::<Gpt5Error>() {
                println!("   Error type: {:?}", gpt5_error);
            }
        }
    }
    
    // Test with valid API key (if available)
    if let Ok(api_key) = std::env::var("OPENAI_API_KEY") {
        println!("\n✅ Testing with valid API key...");
        let client = Gpt5Client::new(api_key);
        
        // Test empty input
        println!("📝 Testing empty input...");
        match client.simple(Gpt5Model::Gpt5Nano, "").await {
            Ok(response) => println!("Response: {}", response),
            Err(e) => println!("❌ Error with empty input: {}", e),
        }
        
        // Test very long input
        println!("\n📝 Testing very long input...");
        let long_input = "Hello ".repeat(10000); // Very long string
        match client.simple(Gpt5Model::Gpt5Nano, &long_input).await {
            Ok(response) => println!("Response length: {} chars", response.len()),
            Err(e) => println!("❌ Error with long input: {}", e),
        }
        
        // Test normal usage
        println!("\n✅ Testing normal usage...");
        match client.simple(Gpt5Model::Gpt5Nano, "Say hello in 3 different languages").await {
            Ok(response) => println!("✅ Success: {}", response),
            Err(e) => println!("❌ Unexpected error: {}", e),
        }
        
    } else {
        println!("⚠️  OPENAI_API_KEY not set, skipping valid key tests");
    }
    
    // Demonstrate error type checking
    println!("\n🔍 Error type checking example...");
    let client = Gpt5Client::new("test-key".to_string());
    
    match client.simple(Gpt5Model::Gpt5Nano, "test").await {
        Ok(_) => println!("Unexpected success"),
        Err(e) => {
            // Check if it's a network error
            if e.to_string().contains("401") || e.to_string().contains("Unauthorized") {
                println!("🔐 Authentication error detected");
            } else if e.to_string().contains("timeout") {
                println!("⏰ Timeout error detected");
            } else if e.to_string().contains("network") {
                println!("🌐 Network error detected");
            } else {
                println!("❓ Other error: {}", e);
            }
        }
    }
    
    Ok(())
}
