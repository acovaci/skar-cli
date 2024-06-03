use crate::{
    api::base::text::{TextCompletionProvider, TextGenerationProvider},
    core::error::Res,
};

pub async fn complete(beginning: &str) -> Res<(String, String)> {
    let context = crate::core::context::ModelContext::ShellCompletionContext;
    let api = crate::api::openai::text::OpenAITextAPI::new(context)?;

    let beginning = beginning.trim();
    let completion = api.complete(beginning).await?;

    Ok((beginning.to_string(), completion))
}

pub async fn explain(beginning: &str) -> Res<String> {
    let context = crate::core::context::ModelContext::ShellExplainContext;
    let api = crate::api::openai::text::OpenAITextAPI::new(context)?;

    let beginning = beginning.trim();
    let explanation = api.generate(beginning).await?;

    Ok(explanation)
}

pub async fn generate(description: &str) -> Res<String> {
    let context = crate::core::context::ModelContext::ShellGenerateContext;
    let api = crate::api::openai::text::OpenAITextAPI::new(context)?;

    let description = description.trim();
    let generation = api
        .generate(description)
        .await?
        .split('\n')
        .next()
        .unwrap()
        .to_string();

    if generation.contains("[NULL]") {
        return Err(crate::core::error::SkarError::NoGenerationResponse);
    }

    Ok(generation)
}
