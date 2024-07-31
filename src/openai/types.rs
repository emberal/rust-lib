use async_openai::{config::OpenAIConfig, error::OpenAIError, Client};

pub type OpenAIClient = Client<OpenAIConfig>;
pub type OpenAIResult<T> = Result<T, OpenAIError>;
