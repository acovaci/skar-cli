use clap::{Parser, Subcommand};
use skar::core::error::Res;

mod shell;

#[derive(Parser, Debug)]
struct Args {
    #[clap(subcommand)]
    command: SkarCommand,
}

#[derive(Subcommand, Debug)]
enum SkarCommand {
    #[clap(subcommand)]
    Shell(shell::SkarShellCommand),
}

pub async fn run() -> Res<()> {
    let args = Args::parse();

    match args.command {
        SkarCommand::Shell(shell::SkarShellCommand::Complete(args)) => shell::complete(args).await,
        SkarCommand::Shell(shell::SkarShellCommand::Explain(args)) => shell::explain(args).await,
        SkarCommand::Shell(shell::SkarShellCommand::Generate(args)) => shell::generate(args).await,
    }
}
