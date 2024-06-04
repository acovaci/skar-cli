pub type Res<T> = Result<T, SkarError>;

#[derive(thiserror::Error, Debug)]
pub enum SkarError {
    #[error("Request error: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("No completion response")]
    NoCompletionResponse,

    #[error("No explanation response")]
    NoExplanationResponse,

    #[error("No generation response")]
    NoGenerationResponse,

    #[error("Environment variable not set: {0}")]
    OpenAIKeyNotSet(#[from] std::env::VarError),

    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),

    #[error("Invalid shell configuration: {0} {1}")]
    InvalidShellConfig(std::path::PathBuf, String),

    #[error("OpenAI API error: {0}")]
    OpenAIError(String),

    #[error("Internal Skar error: {0}")]
    _Internal(String),
}
