use async_openai::types::{
    ChatCompletionRequestMessage, ChatCompletionRequestUserMessage,
    ChatCompletionRequestUserMessageContent, ChatCompletionResponseStream,
    CreateChatCompletionRequest,
};

use crate::openai::types::{OpenAIClient, OpenAIResult};

pub async fn chat(
    client: &OpenAIClient,
    model: impl Into<String>,
    prompt: impl Into<String>,
) -> OpenAIResult<String> {
    Ok(client
        .chat()
        .create(CreateChatCompletionRequest {
            model: model.into(),
            messages: vec![create_user_message(prompt)],
            ..Default::default()
        })
        .await?
        .choices[0]
        .message
        .content
        .clone()
        .unwrap_or_default())
}

pub async fn chat_stream(
    client: &OpenAIClient,
    model: impl Into<String>,
    prompt: impl Into<String>,
) -> OpenAIResult<ChatCompletionResponseStream> {
    client
        .chat()
        .create_stream(CreateChatCompletionRequest {
            model: model.into(),
            stream: Some(true),
            messages: vec![create_user_message(prompt)],
            ..Default::default()
        })
        .await
}

fn create_user_message(prompt: impl Into<String>) -> ChatCompletionRequestMessage {
    ChatCompletionRequestMessage::User(ChatCompletionRequestUserMessage {
        content: ChatCompletionRequestUserMessageContent::from(prompt.into()),
        name: None,
    })
}
