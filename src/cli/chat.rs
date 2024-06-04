use std::io::Write;

use clap::Args;
use skar::{
    api::openai::text::OpenAITextAPI,
    command::chat::ChatContext,
    core::{context::ModelContext, error::Res},
};

const INPUT_PROMPT: &str = "\n?- ";
const OUTPUT_PROMPT: &str = "|- ";

#[derive(Args, Debug)]
pub struct SkarChatArgs {
    #[arg(trailing_var_arg = true, allow_hyphen_values = true, hide = true)]
    pub _args: Vec<String>,
}

pub struct ChatRunner {
    running: bool,
    initial_input: String,
}

impl ChatRunner {
    pub fn new(input: &str) -> Self {
        Self {
            running: false,
            initial_input: input.to_string(),
        }
    }

    pub async fn run(&mut self, ctx: &mut ChatContext) -> Res<Option<String>> {
        let input = if !self.running {
            log::debug!("Using initial input: {}", self.initial_input);
            self.running = true;
            self.initial_input.clone()
        } else {
            log::debug!("Reading input...");

            ChatRunner::print_input_prompt();
            std::io::stdout().flush()?;

            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;
            input.trim().to_string()
        };

        let input = match input.as_str() {
            "exit" => Ok(None),
            _ => Ok(Some(input)),
        };

        match input {
            Ok(Some(input)) => Ok(Some(ctx.chat(input.as_str()).await?)),
            Ok(None) => Ok(None),
            Err(e) => Err(e),
        }
    }

    fn print_input_prompt() {
        print!(
            "{}",
            colored::Colorize::bold(colored::Colorize::green(INPUT_PROMPT))
        );
    }

    fn print_output_prompt() {
        print!(
            "{}",
            colored::Colorize::bold(colored::Colorize::cyan(OUTPUT_PROMPT))
        );
    }
}

pub async fn run(args: SkarChatArgs) -> Res<()> {
    let input = args._args.join(" ");
    let mut runner = ChatRunner::new(&input);

    log::debug!("Initializing chat context...");

    let agent = OpenAITextAPI::new(ModelContext::Chat)?;
    let mut ctx = ChatContext::new(Box::new(agent));

    log::debug!("Chat context initialized.");

    while let Some(response) = runner.run(&mut ctx).await? {
        log::debug!("Ran chat loop");

        ChatRunner::print_output_prompt();
        println!("{}", response);
    }

    Ok(())
}
