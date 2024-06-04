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
    #[arg(trailing_var_arg = true, allow_hyphen_values = true, hide = true)]
    pub _args: Vec<String>,
}

#[derive(Args, Debug)]
pub struct SkarShellExplainArgs {
    #[arg(trailing_var_arg = true, allow_hyphen_values = true, hide = true)]
    pub _args: Vec<String>,
}

#[derive(Args, Debug)]
pub struct SkarShellGenerateArgs {
    #[arg(trailing_var_arg = true, allow_hyphen_values = true, hide = true)]
    pub _args: Vec<String>,
}

pub async fn run(args: SkarShellCommand) -> Res<()> {
    match args {
        SkarShellCommand::Complete(args) => complete(args).await,
        SkarShellCommand::Explain(args) => explain(args).await,
        SkarShellCommand::Generate(args) => generate(args).await,
    }
}

pub async fn complete(args: SkarShellCompleteArgs) -> Res<()> {
    let (beginning, completion) =
        skar::command::shell::complete(args._args.join(" ").as_str()).await?;
    println!(
        "{}{}",
        beginning,
        colored::Colorize::bright_blue(completion.as_str())
    );
    Ok(())
}

pub async fn explain(args: SkarShellExplainArgs) -> Res<()> {
    let explanation = skar::command::shell::explain(args._args.join(" ").as_str()).await?;
    println!("{}", explanation);
    Ok(())
}

pub async fn generate(args: SkarShellGenerateArgs) -> Res<()> {
    let generation = skar::command::shell::generate(args._args.join(" ").as_str()).await?;
    println!("{}", generation);
    Ok(())
}
