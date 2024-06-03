use clap::{Args, Subcommand};
use skar::core::error::Res;

#[derive(Subcommand, Debug)]
pub enum SkarShellCommand {
    Complete(SkarShellCompleteArgs),
    Explain(SkarShellExplainArgs),
    Generate(SkarShellGenerateArgs),
}

#[derive(Args, Debug)]
pub struct SkarShellCompleteArgs {
    pub beginning: String,
}

#[derive(Args, Debug)]
pub struct SkarShellExplainArgs {
    pub command: String,
}

#[derive(Args, Debug)]
pub struct SkarShellGenerateArgs {
    pub description: String,
}

pub async fn complete(args: SkarShellCompleteArgs) -> Res<()> {
    let (beginning, completion) = skar::command::shell::complete(args.beginning.as_str()).await?;
    println!(
        "{}{}",
        beginning,
        colored::Colorize::bright_blue(completion.as_str())
    );
    Ok(())
}

pub async fn explain(args: SkarShellExplainArgs) -> Res<()> {
    let explanation = skar::command::shell::explain(args.command.as_str()).await?;
    println!("{}", explanation);
    Ok(())
}

pub async fn generate(args: SkarShellGenerateArgs) -> Res<()> {
    let generation = skar::command::shell::generate(args.description.as_str()).await?;
    println!("{}", generation);
    Ok(())
}
