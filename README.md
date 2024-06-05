# Skar - A command line LLM assistant

Skar is a command line assistant that helps you with working with shell commands right from your
terminal. It is a simple tool that can be used to complete commands, explain commands, and even
generate commands for you.

## Installation

Download the latest release from the [releases page](https://github.com/acovaci/skar/releases) and
extract the archive to where you want to install Skar. You can also clone the repository and build
the project yourself.

On Linux and MacOS, once you have extracted the archive, run the following command to initialize
Skar:

```bash
$ skar init
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

4. **Chat with the AI**: Use `?-` to open a chat interface with the AI agent. You can ask it
   questions, and it will try to answer them to the best of its ability.

![carbon-6](https://github.com/acovaci/skar-cli/assets/6562353/49d90ced-0360-4b4c-8543-d5115016216c)

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
