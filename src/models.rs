//! GPT-5 Models and related types

/// GPT-5 Models available for the /v1/responses endpoint
///
/// # Examples
///
/// ```rust
/// use gpt5::Gpt5Model;
///
/// let model = Gpt5Model::Gpt5Nano; // Fastest, most cost-effective
/// let custom = Gpt5Model::Custom("gpt-5-custom".to_string());
/// ```
#[derive(Debug, Clone)]
pub enum Gpt5Model {
    /// Latest GPT-5.4 model - most capable current GPT-5 family default
    Gpt54,
    /// GPT-5.4 Mini - balanced performance and cost
    Gpt54Mini,
    /// GPT-5.4 Nano - fastest and most cost-effective
    Gpt54Nano,
    /// Original GPT-5 model - retained for compatibility
    Gpt5,
    /// Original GPT-5 Mini - retained for compatibility
    Gpt5Mini,
    /// Original GPT-5 Nano - retained for compatibility
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
            Gpt5Model::Gpt54 => "gpt-5.4",
            Gpt5Model::Gpt54Mini => "gpt-5.4-mini",
            Gpt5Model::Gpt54Nano => "gpt-5.4-nano",
            Gpt5Model::Gpt5 => "gpt-5",
            Gpt5Model::Gpt5Mini => "gpt-5-mini",
            Gpt5Model::Gpt5Nano => "gpt-5-nano",
            Gpt5Model::Custom(name) => name,
        }
    }
}
