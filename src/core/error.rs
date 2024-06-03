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

    #[error("OpenAI API key not set")]
    OpenAIKeyNotSet(#[from] std::env::VarError),
}
