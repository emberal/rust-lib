use std::{
    pin::Pin,
    task::{Context, Poll},
};

use async_openai::types::ChatCompletionResponseStream;
use async_openai::{
    error::OpenAIError,
    types::{AssistantEventStream, AssistantStreamEvent, MessageDeltaContent, MessageDeltaObject},
};
use async_stream::try_stream;
use futures::{Stream, StreamExt};

use crate::openai::types::OpenAIResult;

pub struct TokenStream(Pin<Box<dyn Stream<Item = OpenAIResult<String>> + Send + 'static>>);

impl TokenStream {
    pub fn new(stream: impl Stream<Item = OpenAIResult<String>> + Send + 'static) -> Self {
        Self(Box::pin(stream))
    }
}

impl Stream for TokenStream {
    type Item = OpenAIResult<String>;
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.0.as_mut().poll_next(cx)
    }
}

impl From<AssistantEventStream> for TokenStream {
    fn from(mut value: AssistantEventStream) -> Self {
        Self::new(try_stream! {
            while let Some(event) = value.next().await {
                if let Ok(AssistantStreamEvent::ThreadMessageDelta(message)) = event {
                    if let Ok(text) = get_message(message) {
                        yield text;
                    };
                }
            }
        })
    }
}

impl From<ChatCompletionResponseStream> for TokenStream {
    fn from(mut value: ChatCompletionResponseStream) -> Self {
        Self::new(try_stream! {
            while let Some(event) = value.next().await {
                if let Ok(event) = event {
                    if let Some(text) = event.choices[0].delta.content.clone() {
                        yield text;
                    };
                }
            }
        })
    }
}

cfg_if::cfg_if! {
    if #[cfg(feature = "axum")] {
        use axum::response::sse::Event;

        pub struct EventStream<E>(Pin<Box<dyn Stream<Item = Result<Event, E>> + Send + 'static>>);

        impl<E> EventStream<E> {
            pub fn new(stream: impl Stream<Item = Result<Event, E>> + Send + 'static) -> Self {
                Self(Box::pin(stream))
            }
        }

        impl<E> Stream for EventStream<E> {
            type Item = Result<Event, E>;
            fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
                self.0.as_mut().poll_next(cx)
            }
        }

        impl<E> From<AssistantEventStream> for EventStream<E>
        where
            E: Send + 'static,
        {
            fn from(mut value: AssistantEventStream) -> Self {
                Self::new(try_stream! {
                    while let Some(event) = value.next().await {
                        if let Ok(AssistantStreamEvent::ThreadMessageDelta(message)) = event {
                            if let Ok(text) = get_message(message) {
                                yield Event::default().data(text);
                            };
                        }
                    }
                })
            }
        }

        impl<E> From<ChatCompletionResponseStream> for EventStream<E>
        where
            E: Send + 'static,
        {
            fn from(mut value: ChatCompletionResponseStream) -> Self {
                Self::new(try_stream! {
                    while let Some(event) = value.next().await {
                        if let Ok(event) = event {
                            if let Some(text) = event.choices[0].delta.content.clone() {
                                yield Event::default().data(text);
                            };
                        }
                    }
                })
            }
        }
    }
}

fn get_message(message: MessageDeltaObject) -> OpenAIResult<String> {
    let content = message
        .delta
        .content
        .and_then(|content| content.first().cloned())
        .ok_or(OpenAIError::StreamError("Expected content".into()))?;

    if let MessageDeltaContent::Text(content) = content {
        content
            .text
            .and_then(|text| text.value)
            .ok_or(OpenAIError::StreamError("Expected text message".into()))
    } else {
        Err(OpenAIError::StreamError("Expected text message".into()))
    }
}
