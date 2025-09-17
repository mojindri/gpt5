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
    /// Main GPT-5 model - most capable
    Gpt5,
    /// GPT-5 Mini - balanced performance and cost
    Gpt5Mini,
    /// GPT-5 Nano - fastest and most cost-effective
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
            Gpt5Model::Gpt5 => "gpt-5",
            Gpt5Model::Gpt5Mini => "gpt-5-mini",
            Gpt5Model::Gpt5Nano => "gpt-5-nano",
            Gpt5Model::Custom(name) => name,
        }
    }
}
