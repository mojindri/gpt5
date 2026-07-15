//! GPT-5 Models and related types

/// GPT-5 Models available for the /v1/responses endpoint
///
/// # Examples
///
/// ```rust
/// use gpt5::Gpt5Model;
///
/// let model = Gpt5Model::Gpt56; // Latest GPT-5.6 flagship alias
/// let custom = Gpt5Model::Custom("gpt-5-custom".to_string());
/// ```
#[derive(Debug, Clone)]
pub enum Gpt5Model {
    /// GPT-5.6 alias that routes to the flagship Sol model
    Gpt56,
    /// GPT-5.6 Sol - flagship model for complex reasoning and coding
    Gpt56Sol,
    /// GPT-5.6 Terra - balanced intelligence and cost
    Gpt56Terra,
    /// GPT-5.6 Luna - cost-sensitive, high-volume workloads
    Gpt56Luna,
    /// GPT-5.5 model for coding and professional work
    Gpt55,
    /// GPT-5.5 Pro - higher capability GPT-5.5 variant
    Gpt55Pro,
    /// GPT-5.4 model - affordable model for coding and professional work
    Gpt54,
    /// GPT-5.4 Pro - higher capability GPT-5.4 variant
    Gpt54Pro,
    /// GPT-5.4 Mini - balanced performance and cost
    Gpt54Mini,
    /// GPT-5.4 Nano - fastest and most cost-effective
    Gpt54Nano,
    /// GPT-5.3 Codex - agentic coding model
    Gpt53Codex,
    /// GPT-5.2 model - retained for compatibility
    Gpt52,
    /// GPT-5.2 Pro - retained for compatibility
    Gpt52Pro,
    /// GPT-5.1 model - retained for compatibility
    Gpt51,
    /// GPT-5 model - retained for compatibility
    Gpt5,
    /// GPT-5 Pro - retained for compatibility
    Gpt5Pro,
    /// GPT-5 Mini - retained for compatibility
    Gpt5Mini,
    /// GPT-5 Nano - retained for compatibility
    Gpt5Nano,
    /// Custom GPT-5 variant for future models
    Custom(String),
}

impl Gpt5Model {
    /// Returns the string representation of the model for API calls
    ///
    /// # Examples
    ///
    /// ```rust
    /// use gpt5::Gpt5Model;
    ///
    /// assert_eq!(Gpt5Model::Gpt5.as_str(), "gpt-5");
    /// assert_eq!(Gpt5Model::Gpt5Nano.as_str(), "gpt-5-nano");
    /// ```
    pub fn as_str(&self) -> &str {
        match self {
            Gpt5Model::Gpt56 => "gpt-5.6",
            Gpt5Model::Gpt56Sol => "gpt-5.6-sol",
            Gpt5Model::Gpt56Terra => "gpt-5.6-terra",
            Gpt5Model::Gpt56Luna => "gpt-5.6-luna",
            Gpt5Model::Gpt55 => "gpt-5.5",
            Gpt5Model::Gpt55Pro => "gpt-5.5-pro",
            Gpt5Model::Gpt54 => "gpt-5.4",
            Gpt5Model::Gpt54Pro => "gpt-5.4-pro",
            Gpt5Model::Gpt54Mini => "gpt-5.4-mini",
            Gpt5Model::Gpt54Nano => "gpt-5.4-nano",
            Gpt5Model::Gpt53Codex => "gpt-5.3-codex",
            Gpt5Model::Gpt52 => "gpt-5.2",
            Gpt5Model::Gpt52Pro => "gpt-5.2-pro",
            Gpt5Model::Gpt51 => "gpt-5.1",
            Gpt5Model::Gpt5 => "gpt-5",
            Gpt5Model::Gpt5Pro => "gpt-5-pro",
            Gpt5Model::Gpt5Mini => "gpt-5-mini",
            Gpt5Model::Gpt5Nano => "gpt-5-nano",
            Gpt5Model::Custom(name) => name,
        }
    }
}
