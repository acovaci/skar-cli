use openai_dive::v1::resources::chat::{ChatCompletionParameters, ChatCompletionResponse};

use crate::{
    api::base::text::{Message, Role, TextGenerationProvider},
    core::error::{Res, SkarError},
};

pub struct OpenAITextAPI {
    client: openai_dive::v1::api::Client,
    context: crate::core::context::ModelContext,
}

impl OpenAITextAPI {
    pub fn new(context: crate::core::context::ModelContext) -> Res<Self> {
        let key = std::env::var("OPENAI_API_KEY")?;

        let client = openai_dive::v1::api::Client::new(key);

        Ok(Self { client, context })
    }

    pub fn generate_messages(
        &self,
        history: Vec<Message>,
    ) -> Vec<openai_dive::v1::resources::chat::ChatMessage> {
        let mut messages = Vec::new();
        let context = self.context.to_string();

        if let Some(context) = context {
            messages.push(openai_dive::v1::resources::chat::ChatMessage {
                role: openai_dive::v1::resources::chat::Role::System,
                content: openai_dive::v1::resources::chat::ChatMessageContent::Text(
                    context.to_string(),
                ),
                ..Default::default()
            });
        }

        for message in history {
            messages.push(openai_dive::v1::resources::chat::ChatMessage {
                role: match message.role {
                    Role::User => openai_dive::v1::resources::chat::Role::User,
                    Role::Agent => openai_dive::v1::resources::chat::Role::System,
                },
                content: openai_dive::v1::resources::chat::ChatMessageContent::Text(
                    message.content.clone(),
                ),
                ..Default::default()
            });
        }

        messages
    }

    async fn submit(&self, parameters: ChatCompletionParameters) -> Res<ChatCompletionResponse> {
        for message in &parameters.messages {
            log::debug!("Sending message: {:?}", message);
        }

        let res = self
            .client
            .chat()
            .create(parameters)
            .await
            .map_err(|e| SkarError::OpenAIError(e.to_string()))?;

        log::debug!("Received response: {:?}", res);

        Ok(res)
    }
}

#[async_trait::async_trait]
impl crate::api::base::text::TextCompletionProvider for OpenAITextAPI {
    async fn complete(&self, beginning: &str) -> Res<String> {
        let messages = self.generate_messages(vec![Message {
            role: Role::User,
            content: beginning.to_string(),
        }]);

        let parameters = openai_dive::v1::resources::chat::ChatCompletionParameters {
            model: openai_dive::v1::models::Gpt4Engine::Gpt4O.to_string(),
            messages,
            max_tokens: Some(12),
            ..Default::default()
        };

        let res = self.submit(parameters).await?;

        let res = match &res.choices[0].message.content {
            openai_dive::v1::resources::chat::ChatMessageContent::Text(res) => {
                Ok::<String, SkarError>(res.clone())
            }
            _ => return Err(SkarError::NoCompletionResponse),
        }?;

        let res = res.trim().split('\n').next().unwrap_or(&res).to_string();

        if res.contains("[NULL]") {
            Ok(String::new())
        } else {
            Ok(res.strip_prefix(beginning).unwrap_or(&res).to_string())
        }
    }
}

#[async_trait::async_trait]
impl TextGenerationProvider for OpenAITextAPI {
    async fn reply(&self, instruction: &str) -> Res<String> {
        let messages = self.generate_messages(vec![Message {
            role: Role::User,
            content: instruction.to_string(),
        }]);

        let parameters = openai_dive::v1::resources::chat::ChatCompletionParameters {
            model: openai_dive::v1::models::Gpt4Engine::Gpt4O.to_string(),
            messages,
            max_tokens: Some(512),
            ..Default::default()
        };

        let res = self.submit(parameters).await?;

        match &res.choices[0].message.content {
            openai_dive::v1::resources::chat::ChatMessageContent::Text(res) => Ok(res.clone()),
            _ => Err(SkarError::NoCompletionResponse),
        }
    }

    async fn generate(&self, context: Vec<Message>) -> Res<String> {
        let messages = self.generate_messages(context);

        let parameters = openai_dive::v1::resources::chat::ChatCompletionParameters {
            model: openai_dive::v1::models::Gpt4Engine::Gpt4O.to_string(),
            messages,
            max_tokens: Some(512),
            ..Default::default()
        };

        let res = self.submit(parameters).await?;

        match &res.choices[0].message.content {
            openai_dive::v1::resources::chat::ChatMessageContent::Text(res) => Ok(res.clone()),
            _ => Err(SkarError::NoCompletionResponse),
        }
    }
}
