use crate::core::error::Res;

pub trait TextCompletionProvider {
    fn complete(&self, beginning: &str) -> impl std::future::Future<Output = Res<String>> + Send;
}

pub trait TextGenerationProvider {
    fn generate(&self, instruction: &str) -> impl std::future::Future<Output = Res<String>> + Send;
}
