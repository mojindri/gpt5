//! GPT-5 API client implementation

use crate::models::Gpt5Model;
use crate::requests::{Gpt5Request, Gpt5RequestBuilder};
use crate::responses::{Gpt5Response, OpenAiError};
use reqwest::Client;

/// Main client for interacting with the GPT-5 API
///
/// The `Gpt5Client` handles authentication, request building, and response parsing
/// for the OpenAI GPT-5 API. It supports both simple and advanced usage patterns.
///
/// # Examples
///
/// ## Simple Usage
///
/// ```rust
/// use gpt5::{Gpt5Client, Gpt5Model};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = Gpt5Client::new("your-api-key".to_string());
///     let response = client.simple(Gpt5Model::Gpt5Nano, "Hello!").await?;
///     println!("{}", response);
///     Ok(())
/// }
/// ```
///
/// ## Advanced Usage with Custom Base URL
///
/// ```rust
/// use gpt5::Gpt5Client;
///
/// let client = Gpt5Client::new("your-api-key".to_string())
///     .with_base_url("https://custom-api.example.com".to_string());
/// ```
pub struct Gpt5Client {
    pub client: Client,
    pub api_key: String,
    pub base_url: String,
}

impl Gpt5Client {
    /// Create a new GPT-5 client with the specified API key
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your OpenAI API key
    ///
    /// # Examples
    ///
    /// ```rust
    /// use gpt5::Gpt5Client;
    ///
    /// let client = Gpt5Client::new("sk-...".to_string());
    /// ```
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
            base_url: "https://api.openai.com".to_string(),
        }
    }

    /// Set a custom base URL for the API
    ///
    /// # Arguments
    ///
    /// * `base_url` - Custom base URL for the API
    ///
    /// # Examples
    ///
    /// ```rust
    /// use gpt5::Gpt5Client;
    ///
    /// let client = Gpt5Client::new("sk-...".to_string())
    ///     .with_base_url("https://custom-api.example.com".to_string());
    /// ```
    pub fn with_base_url(mut self, base_url: String) -> Self {
        self.base_url = base_url;
        self
    }

    /// Send a request to the GPT-5 API
    ///
    /// # Arguments
    ///
    /// * `req` - The GPT-5 request to send
    ///
    /// # Returns
    ///
    /// * `Result<Gpt5Response, anyhow::Error>` - The response or an error
    ///
    /// # Examples
    ///
    /// ```rust
    /// use gpt5::{Gpt5Client, Gpt5RequestBuilder, Gpt5Model};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Gpt5Client::new("sk-...".to_string());
    ///     let request = Gpt5RequestBuilder::new(Gpt5Model::Gpt5Nano)
    ///         .input("Hello!")
    ///         .build();
    ///     
    ///     let response = client.request(request).await?;
    ///     println!("{:?}", response);
    ///     Ok(())
    /// }
    /// ```
    pub async fn request(&self, req: Gpt5Request) -> anyhow::Result<Gpt5Response> {
        // Validate GPT-5 model
        if !self.is_gpt5_model(&req.model) {
            return Err(anyhow::anyhow!(
                "Only GPT-5 models are supported. Got: {}",
                req.model
            ));
        }

        let url = format!("{}/v1/responses", self.base_url);

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&req)
            .send()
            .await?;

        let response_text = response.text().await?;

        // Log the raw response for debugging
        tracing::info!("GPT-5 raw response: {}", response_text);

        // Try to parse as a generic JSON first to see the structure
        match serde_json::from_str::<serde_json::Value>(&response_text) {
            Ok(json_value) => {
                tracing::info!("GPT-5 response JSON structure: {:#}", json_value);

                // First check if this is an error response
                if let Ok(error_response) = serde_json::from_str::<OpenAiError>(&response_text) {
                    tracing::error!("OpenAI API error: {}", error_response.error.message);
                    return Err(anyhow::anyhow!(
                        "OpenAI API error: {}",
                        error_response.error.message
                    ));
                }

                // Try to parse as our expected success structure
                match serde_json::from_str::<Gpt5Response>(&response_text) {
                    Ok(gpt_response) => Ok(gpt_response),
                    Err(parse_error) => {
                        tracing::error!("Failed to parse GPT-5 response: {}", parse_error);
                        tracing::error!("Raw response: {}", response_text);
                        Err(anyhow::anyhow!(
                            "Failed to parse GPT-5 response: {}",
                            parse_error
                        ))
                    }
                }
            }
            Err(json_error) => {
                tracing::error!("Invalid JSON response from GPT-5: {}", json_error);
                tracing::error!("Raw response: {}", response_text);
                Err(anyhow::anyhow!("Invalid JSON response: {}", json_error))
            }
        }
    }

    /// Send a simple request and get text response
    ///
    /// # Arguments
    ///
    /// * `model` - The GPT-5 model to use
    /// * `prompt` - The input prompt
    ///
    /// # Returns
    ///
    /// * `Result<String, anyhow::Error>` - The text response or an error
    ///
    /// # Examples
    ///
    /// ```rust
    /// use gpt5::{Gpt5Client, Gpt5Model};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Gpt5Client::new("sk-...".to_string());
    ///     let response = client.simple(Gpt5Model::Gpt5Nano, "Hello!").await?;
    ///     println!("{}", response);
    ///     Ok(())
    /// }
    /// ```
    pub async fn simple(&self, model: Gpt5Model, prompt: &str) -> anyhow::Result<String> {
        let req = Gpt5RequestBuilder::new(model).input(prompt).build();

        let response = self.request(req).await?;
        self.extract_text(&response)
    }

    /// Extract text from a GPT-5 response
    ///
    /// # Arguments
    ///
    /// * `response` - The GPT-5 response
    ///
    /// # Returns
    ///
    /// * `Result<String, anyhow::Error>` - The extracted text or an error
    fn extract_text(&self, response: &Gpt5Response) -> anyhow::Result<String> {
        // Look for text content in the output array
        if let Some(outputs) = &response.output {
            for output in outputs {
                if output.output_type == crate::enums::OutputType::Message {
                    if let Some(content_array) = &output.content {
                        for content in content_array {
                            if content.content_type == crate::enums::ContentType::OutputText {
                                if let Some(text) = &content.text {
                                    return Ok(text.clone());
                                }
                            }
                        }
                    }
                }
            }
        }

        // If no text output found, return error
        Err(anyhow::anyhow!("No text content in response output"))
    }

    /// Validate if a model is a GPT-5 model
    ///
    /// # Arguments
    ///
    /// * `model` - The model string to validate
    ///
    /// # Returns
    ///
    /// * `bool` - True if it's a GPT-5 model, false otherwise
    fn is_gpt5_model(&self, model: &str) -> bool {
        matches!(model, "gpt-5" | "gpt-5-mini" | "gpt-5-nano") || model.starts_with("gpt-5")
    }
}
