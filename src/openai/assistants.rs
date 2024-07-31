use async_openai::{
    types::{
        AssistantEventStream, AssistantObject, CreateAssistantRequest, CreateMessageRequest,
        CreateRunRequest, CreateThreadRequest, DeleteAssistantResponse, DeleteThreadResponse,
        MessageObject, MessageRole, ThreadObject,
    },
    Client,
};

use crate::openai::types::{OpenAIClient, OpenAIResult};

#[derive(Clone, Debug)]
pub struct Assistant {
    client: OpenAIClient,
    assistant_object: AssistantObject,
}

#[derive(Clone, Debug)]
pub struct Thread<'client> {
    client: &'client OpenAIClient,
    assistant_id: String,
    thread_object: ThreadObject,
}

impl Assistant {
    pub async fn new(
        client: &OpenAIClient,
        model: impl Into<String>,
        instructions: impl Into<String>,
    ) -> OpenAIResult<Self> {
        let assistant_object = client
            .assistants()
            .create(CreateAssistantRequest {
                model: model.into(),
                instructions: Some(instructions.into()),
                ..Default::default()
            })
            .await?;
        Ok(Self {
            client: client.clone(),
            assistant_object,
        })
    }

    pub async fn from_id(id: impl AsRef<str>) -> OpenAIResult<Self> {
        let client = Client::new();
        let assistant_object = client.assistants().retrieve(id.as_ref()).await?;
        Ok(Self {
            client,
            assistant_object,
        })
    }

    pub async fn create_thread(&self) -> OpenAIResult<Thread> {
        Thread::new(&self.client, self.id()).await
    }

    pub async fn delete(self) -> OpenAIResult<DeleteAssistantResponse> {
        self.client.assistants().delete(self.id()).await
    }

    pub fn id(&self) -> &str {
        &self.assistant_object.id
    }
}

impl<'client> Thread<'client> {
    pub async fn new(
        client: &'client OpenAIClient,
        assistant_id: impl Into<String>,
    ) -> OpenAIResult<Self> {
        Ok(Self {
            client,
            assistant_id: assistant_id.into(),
            thread_object: client
                .threads()
                .create(CreateThreadRequest::default())
                .await?,
        })
    }

    pub async fn from_id(
        client: &'client OpenAIClient,
        assistant_id: impl Into<String>,
        thread_id: impl AsRef<str>,
    ) -> OpenAIResult<Self> {
        Ok(Self {
            client,
            assistant_id: assistant_id.into(),
            thread_object: client.threads().retrieve(thread_id.as_ref()).await?,
        })
    }

    pub async fn run_stream(&self, prompt: impl AsRef<str>) -> OpenAIResult<AssistantEventStream> {
        self.create_message(prompt.as_ref()).await?;
        self.client
            .threads()
            .runs(self.id())
            .create_stream(CreateRunRequest {
                assistant_id: self.assistant_id.clone(),
                ..Default::default()
            })
            .await
    }

    pub fn id(&self) -> &str {
        &self.thread_object.id
    }

    async fn create_message(&self, prompt: &str) -> OpenAIResult<MessageObject> {
        self.client
            .threads()
            .messages(&self.thread_object.id)
            .create(CreateMessageRequest {
                role: MessageRole::User,
                content: prompt.into(),
                ..Default::default()
            })
            .await
    }

    async fn delete(&self) -> OpenAIResult<DeleteThreadResponse> {
        self.client.threads().delete(self.id()).await
    }
}
