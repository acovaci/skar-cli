use crate::core::error::Res;

#[derive(Clone, Debug)]
pub enum Role {
    User,
    Agent,
}

#[derive(Clone, Debug)]
pub struct Message {
    pub role: Role,
    pub content: String,
}

#[async_trait::async_trait]
pub trait TextCompletionProvider {
    async fn complete(&self, beginning: &str) -> Res<String>;
}

#[async_trait::async_trait]
pub trait TextGenerationProvider {
    async fn reply(&self, instruction: &str) -> Res<String>;
    async fn generate(&self, context: Vec<Message>) -> Res<String>;
}
