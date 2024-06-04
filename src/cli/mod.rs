use clap::{Parser, Subcommand};
use skar::core::error::Res;

mod chat;
mod init;
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
    Init,
    Chat(chat::SkarChatArgs),
}

pub async fn run() -> Res<()> {
    let args = Args::parse();

    match args.command {
        SkarCommand::Shell(command) => shell::run(command).await,
        SkarCommand::Init => init::run().await,
        SkarCommand::Chat(args) => chat::run(args).await,
    }
}
