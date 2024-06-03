# Skar - A command line LLM assistant

Skar is a command line assistant that helps you with working with shell commands right from your
terminal. It is a simple tool that can be used to complete commands, explain commands, and even
generate commands for you.

## Installation

You first need to install Rust on your system. You can do this by following the instructions on the
[Rust website](https://www.rust-lang.org/tools/install).

Once you have Rust installed, you can install Skar by running the following commands:

```bash
$ cargo build --release
$ scripts/install.sh
```

Finally, you will also need to set your OpenAI API key as an environment variable. There are
multiple ways to do this, but the most basic is to add the following line to your shell's
configuration file (e.g., `~/.bashrc` or `~/.zshrc`):

```bash
export OPEN_AI_KEY="your-api-key"
```

You can get your OpenAI API key by signing up for an account on the
[OpenAI website](https://platform.openai.com/signup). I recommend you create a new project and
generate an API for a service account within it. This should give you greater control over your
usage and billing.

## Usage

Skar can currently do three things for you:

1. **Complete commands**: Use `?` before a command to get their best guess at what you are trying to
   type. For example, if you type `? ls`, Skar will suggest `ls -l` as a possible completion.

2. **Explain commands**: Use `??` before a command to get a brief explanation of what the command
   does. For example, if you type `?? ls -la /dev`, Skar will tell you that this command lists all
   devices in the `/dev` directory.

3. **Generate commands**: Use `?!` with a short description of what you want to do, and Skar will
   generate a command for you. For example, if you type `?! list all files in a directory`, Skar
   will suggest `ls -la` as a possible command.

![actual command outputs from the tool](https://github.com/acovaci/skar-cli/assets/6562353/00f2bd29-b8b4-43e0-9791-84c891a9f6aa)

## Contributing

Contributions are more than welcome! If you have an idea for a new feature or a bug fix, please
start a discussion. If you need to report a bug, please open an issue.

I would especially welcome contributions in the following areas:

1. **New models**: I am currently using the GPT-4o model from OpenAI. I would especially want to
   integrate open source models into this project. If you are interested in this, please let me
   know.
2. **Testing**: I have not written any tests for this project yet. If you are interested in writing
   tests, please let me know.
3. **Packaging**: I would also want to provide apt, brew, winget, etc. packages.

## License

This project is licensed under the GNU Affero General Public License v3.0. You can find the full
license text in the [LICENSE](LICENSE) file.

