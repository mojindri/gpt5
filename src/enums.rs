//! Type-safe enums for GPT-5 API parameters


/// Reasoning effort level for GPT-5 responses
/// 
/// Controls how much computational effort GPT-5 puts into reasoning
/// before generating the final response. Higher effort typically means
/// better quality but slower responses.
/// 
/// # Examples
/// 
/// ```rust
/// use gpt5::ReasoningEffort;
/// 
/// let low_effort = ReasoningEffort::Low;    // Fast, basic reasoning
/// let high_effort = ReasoningEffort::High;  // Slow, thorough reasoning
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReasoningEffort {
    /// Low reasoning effort - fastest responses
    Low,
    /// Medium reasoning effort - balanced performance
    Medium,
    /// High reasoning effort - most thorough analysis
    High,
    /// Unknown effort level (for future compatibility)
    Unknown(String),
}

impl serde::Serialize for ReasoningEffort {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            ReasoningEffort::Low => serializer.serialize_str("low"),
            ReasoningEffort::Medium => serializer.serialize_str("medium"),
            ReasoningEffort::High => serializer.serialize_str("high"),
            ReasoningEffort::Unknown(s) => serializer.serialize_str(s),
        }
    }
}

impl<'de> serde::Deserialize<'de> for ReasoningEffort {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "low" => ReasoningEffort::Low,
            "medium" => ReasoningEffort::Medium,
            "high" => ReasoningEffort::High,
            _ => ReasoningEffort::Unknown(s),
        })
    }
}

/// Verbosity level for GPT-5 responses
/// 
/// Controls the detail level of responses from GPT-5.
/// Higher verbosity typically means more detailed explanations.
/// 
/// # Examples
/// 
/// ```rust
/// use gpt5::VerbosityLevel;
/// 
/// let low = VerbosityLevel::Low;      // Concise responses
/// let high = VerbosityLevel::High;    // Detailed responses
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VerbosityLevel {
    /// Low verbosity - concise responses
    Low,
    /// Medium verbosity - balanced detail
    Medium,
    /// High verbosity - detailed responses
    High,
    /// Unknown verbosity level (for future compatibility)
    Unknown(String),
}

impl serde::Serialize for VerbosityLevel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            VerbosityLevel::Low => serializer.serialize_str("low"),
            VerbosityLevel::Medium => serializer.serialize_str("medium"),
            VerbosityLevel::High => serializer.serialize_str("high"),
            VerbosityLevel::Unknown(s) => serializer.serialize_str(s),
        }
    }
}

impl<'de> serde::Deserialize<'de> for VerbosityLevel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "low" => VerbosityLevel::Low,
            "medium" => VerbosityLevel::Medium,
            "high" => VerbosityLevel::High,
            _ => VerbosityLevel::Unknown(s),
        })
    }
}

/// Output type for GPT-5 responses
/// 
/// Indicates the type of output content in the response.
/// 
/// # Examples
/// 
/// ```rust
/// use gpt5::OutputType;
/// 
/// let message = OutputType::Message;        // Text message
/// let function_call = OutputType::FunctionCall; // Function call
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OutputType {
    /// Message output - regular text content
    Message,
    /// Function call output - tool/function invocation
    FunctionCall,
    /// Unknown output type (for future compatibility)
    Unknown(String),
}

impl serde::Serialize for OutputType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            OutputType::Message => serializer.serialize_str("message"),
            OutputType::FunctionCall => serializer.serialize_str("function_call"),
            OutputType::Unknown(s) => serializer.serialize_str(s),
        }
    }
}

impl<'de> serde::Deserialize<'de> for OutputType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "message" => OutputType::Message,
            "function_call" => OutputType::FunctionCall,
            _ => OutputType::Unknown(s),
        })
    }
}

/// Content type for output content
/// 
/// Specifies the type of content within an output message.
/// 
/// # Examples
/// 
/// ```rust
/// use gpt5::ContentType;
/// 
/// let text = ContentType::OutputText; // Text content
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContentType {
    /// Output text content
    OutputText,
    /// Unknown content type (for future compatibility)
    Unknown(String),
}

impl serde::Serialize for ContentType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            ContentType::OutputText => serializer.serialize_str("output_text"),
            ContentType::Unknown(s) => serializer.serialize_str(s),
        }
    }
}

impl<'de> serde::Deserialize<'de> for ContentType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "output_text" => ContentType::OutputText,
            _ => ContentType::Unknown(s),
        })
    }
}

/// Response status for GPT-5 responses
/// 
/// Indicates the current status of a GPT-5 response.
/// 
/// # Examples
/// 
/// ```rust
/// use gpt5::Status;
/// 
/// let completed = Status::Completed;     // Response is complete
/// let in_progress = Status::InProgress;  // Response is still being generated
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Status {
    /// Response is in progress
    InProgress,
    /// Response is completed
    Completed,
    /// Response requires action (e.g., function call)
    RequiresAction,
    /// Response failed
    Failed,
    /// Unknown status (for future compatibility)
    Unknown(String),
}

impl serde::Serialize for Status {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Status::InProgress => serializer.serialize_str("in_progress"),
            Status::Completed => serializer.serialize_str("completed"),
            Status::RequiresAction => serializer.serialize_str("requires_action"),
            Status::Failed => serializer.serialize_str("failed"),
            Status::Unknown(s) => serializer.serialize_str(s),
        }
    }
}

impl<'de> serde::Deserialize<'de> for Status {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "in_progress" => Status::InProgress,
            "completed" => Status::Completed,
            "requires_action" => Status::RequiresAction,
            "failed" => Status::Failed,
            _ => Status::Unknown(s),
        })
    }
}

/// Role in the conversation
/// 
/// Specifies the role of the message sender in the conversation.
/// 
/// # Examples
/// 
/// ```rust
/// use gpt5::Role;
/// 
/// let user = Role::User;         // User message
/// let assistant = Role::Assistant; // Assistant message
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Role {
    /// User role
    User,
    /// Assistant role
    Assistant,
    /// Tool role
    Tool,
    /// System role
    System,
    /// Unknown role (for future compatibility)
    Unknown(String),
}

impl serde::Serialize for Role {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Role::User => serializer.serialize_str("user"),
            Role::Assistant => serializer.serialize_str("assistant"),
            Role::Tool => serializer.serialize_str("tool"),
            Role::System => serializer.serialize_str("system"),
            Role::Unknown(s) => serializer.serialize_str(s),
        }
    }
}

impl<'de> serde::Deserialize<'de> for Role {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "user" => Role::User,
            "assistant" => Role::Assistant,
            "tool" => Role::Tool,
            "system" => Role::System,
            _ => Role::Unknown(s),
        })
    }
}

/// Format type for text responses
/// 
/// Specifies the format of text content in responses.
/// 
/// # Examples
/// 
/// ```rust
/// use gpt5::FormatType;
/// 
/// let markdown = FormatType::Markdown;     // Markdown format
/// let plain_text = FormatType::PlainText; // Plain text format
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FormatType {
    /// Markdown format
    Markdown,
    /// Plain text format
    PlainText,
    /// Unknown format type (for future compatibility)
    Unknown(String),
}

impl serde::Serialize for FormatType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            FormatType::Markdown => serializer.serialize_str("markdown"),
            FormatType::PlainText => serializer.serialize_str("plain_text"),
            FormatType::Unknown(s) => serializer.serialize_str(s),
        }
    }
}

impl<'de> serde::Deserialize<'de> for FormatType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "markdown" => FormatType::Markdown,
            "plain_text" => FormatType::PlainText,
            _ => FormatType::Unknown(s),
        })
    }
}
