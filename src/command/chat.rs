use crate::{
    api::base::text::{Message, Role, TextGenerationProvider},
    core::error::Res,
};

pub struct ChatContext {
    agent: Box<dyn TextGenerationProvider>,
    history: Vec<Message>,
}

impl ChatContext {
    pub fn new(agent: Box<dyn TextGenerationProvider>) -> Self {
        Self {
            agent,
            history: Vec::new(),
        }
    }

    pub async fn chat(&mut self, input: &str) -> Res<String> {
        self.history.push(Message {
            role: Role::User,
            content: input.to_string(),
        });
        let response = self.agent.generate(self.history.clone()).await?;
        self.history.push(Message {
            role: Role::Agent,
            content: response.clone(),
        });
        log::debug!("Chat history: {:?}", self.history);
        return Ok(response);
    }
}
