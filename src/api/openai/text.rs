use crate::{
    api::base::text::TextGenerationProvider,
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
}

impl crate::api::base::text::TextCompletionProvider for OpenAITextAPI {
    async fn complete(&self, beginning: &str) -> Res<String> {
        let parameters = openai_dive::v1::resources::chat::ChatCompletionParameters {
            model: openai_dive::v1::models::Gpt4Engine::Gpt4O.to_string(),
            messages: vec![
                openai_dive::v1::resources::chat::ChatMessage {
                    role: openai_dive::v1::resources::chat::Role::System,
                    content: openai_dive::v1::resources::chat::ChatMessageContent::Text(
                        self.context.to_string(),
                    ),
                    ..Default::default()
                },
                openai_dive::v1::resources::chat::ChatMessage {
                    role: openai_dive::v1::resources::chat::Role::User,
                    content: openai_dive::v1::resources::chat::ChatMessageContent::Text(
                        beginning.to_string(),
                    ),
                    ..Default::default()
                },
            ],
            max_tokens: Some(12),
            ..Default::default()
        };

        for message in &parameters.messages {
            log::debug!("Sending message: {:?}", message);
        }

        let res = self.client.chat().create(parameters).await.unwrap();

        log::debug!("Received response: {:?}", res);

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

impl TextGenerationProvider for OpenAITextAPI {
    fn generate(&self, instruction: &str) -> impl std::future::Future<Output = Res<String>> + Send {
        async move {
            let parameters = openai_dive::v1::resources::chat::ChatCompletionParameters {
                model: openai_dive::v1::models::Gpt4Engine::Gpt4O.to_string(),
                messages: vec![
                    openai_dive::v1::resources::chat::ChatMessage {
                        role: openai_dive::v1::resources::chat::Role::System,
                        content: openai_dive::v1::resources::chat::ChatMessageContent::Text(
                            self.context.to_string(),
                        ),
                        ..Default::default()
                    },
                    openai_dive::v1::resources::chat::ChatMessage {
                        role: openai_dive::v1::resources::chat::Role::User,
                        content: openai_dive::v1::resources::chat::ChatMessageContent::Text(
                            instruction.to_string(),
                        ),
                        ..Default::default()
                    },
                ],
                max_tokens: Some(512),
                ..Default::default()
            };

            for message in &parameters.messages {
                log::debug!("Sending message: {:?}", message);
            }

            let res = self.client.chat().create(parameters).await.unwrap();

            log::debug!("Received response: {:?}", res);

            match &res.choices[0].message.content {
                openai_dive::v1::resources::chat::ChatMessageContent::Text(res) => Ok(res.clone()),
                _ => Err(SkarError::NoCompletionResponse),
            }
        }
    }
}
