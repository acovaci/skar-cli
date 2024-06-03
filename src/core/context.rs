pub enum ModelContext {
    ShellCompletionContext,
    ShellExplainContext,
    ShellGenerateContext,
}

impl ModelContext {
    pub fn to_string(&self) -> String {
        match self {
            ModelContext::ShellCompletionContext => {
                let instructions = vec![
                    "You are part of a command-line interface that provides auto-completion for shell",
                    "commands. You will get part of a command and you need to provide the full command,",
                    "including the input given by the user.",
                    "Because of this, any reply that contains text that is not a valid command will be",
                    "rejected. If you are unable to provide a valid command, you can include [NULL] in your",
                    "response to indicate that you do not have a valid completion. To be clear, any response",
                    "containing [NULL] will be treated as a null response, regardless of any other text in",
                    "the response. Any response that is not a valid command and does not contain [NULL] will",
                    "be rejected. You might receive requests containing popular command line programs, you",
                    "must provide completion for those as well, even though they're not built-in commands.",
                ]
                .join(" ");
                let examples = vec![
                    super::example::Example {
                        input: "ls -l /home/user/.c".to_string(),
                        output: "ls -l /home/user/.config".to_string(),
                    },
                    super::example::Example {
                        input: "dd of=\"usb_dump.bin\" if=\"/dev/s".to_string(),
                        output: "dd of=\"usb_dump.bin\" if=\"/dev/sda\"".to_string(),
                    },
                    super::example::Example {
                        input: "echo \"Hello, world!".to_string(),
                        output: "echo \"Hello, world!".to_string(),
                    },
                    super::example::Example {
                        input: "afhhi12412k".to_string(),
                        output: "[NULL]".to_string(),
                    },
                ]
                .into_iter()
                .map(|example| example.to_string())
                .collect::<Vec<String>>()
                .join("\n");
                format!("{}\n\nExamples:{}", instructions, examples)
            }
            ModelContext::ShellExplainContext => {
                let instructions = vec![
                "You are part of a command-line interface that provides explanations for shell",
                "commands. You will get a full command and you need to provide an explanation for",
                "it. The explanation should be a short description of what the command does, as",
                "well as common use cases for the command. Include practical examples of how the",
                "command is used, and any other relevant information that would help a user",
            ]
                .join(" ");
                let examples = vec![
                    super::example::Example {
                        input: "ls -l /home/user/.config".to_string(),
                        output: vec![
                            "The `ls` command is commonly used to list files and directories in a",
                            "directory. The `-l` flag is used to display the files and directories",
                            "in a long format, which includes additional information such as file",
                            "permissions, owner, group, size, and modification time.\n",
                            "\n",
                            "The `/home/user/.config` argument specifies the directory to list. In",
                            "this case, the command will list the files and directories in the",
                            "`/home/user/.config` directory. This directory is commonly used to",
                            "store configuration files for various applications.\n",
                            "\n",
                            "Command usage:\n",
                            "ls [OPTION] [FILE]\n",
                            "\n",
                            "Common options:\n",
                            "`-l`: Display files and directories in long format (one per line)\n",
                            "`-a`: Include hidden files and directories\n",
                            "`-h`: Display file sizes in human-readable format\n",
                            "`-t`: Sort files and directories by modification time\n",
                            "`-r`: Reverse the order of the sort\n",
                            "\n",
                            "Examples:\n",
                            "`ls -l /home/user/.config`: List files and directories in the\n",
                            "\t`/home/user/.config` directory in long format\n",
                            "`ls -a /usr/bin`: List all files and directories in the `/usr/bin`\n",
                            "\tdirectory, including hidden files\n",
                            "`ls -rlh /var/log`: List files and directories in the `/var/log`\n",
                            "\tdirectory in reverse order\n",
                        ]
                        .join(""),
                    },
                    super::example::Example {
                        input: "dd of=\"usb_dump.bin\" if=\"/dev/sda\"".to_string(),
                        output: vec![
                            "The `dd` command is commonly used to convert and copy files. The `of`",
                            "argument specifies the output file, while the `if` argument specifies",
                            "the input file. In this case, the command will copy the contents of",
                            "the device `/dev/sda` (commonly a storage device, for example a USB",
                            "drive) to the binary file `usb_dump.bin`.\n",
                            "\n",
                            "Command usage:\n",
                            "dd [OPTION] if=[FILE] of=[FILE]\n",
                            "\n",
                            "Common options:\n",
                            "`if=`: Specify the input file\n",
                            "`of=`: Specify the output file\n",
                            "`bs=`: Specify the block size for copying (how many bytes to",
                            "copy at a time). By default, `dd` uses a block size of 512 bytes.\n",
                            "`count=`: Specify the number of blocks to copy. By default, `dd`",
                            "copies until the end of the input file.\n",
                            "`status=progress`: Display the progress of the copy operation.\n",
                            "`skip=`: Skip a specified number of bytes before copying.\n",
                            "\n",
                            "Examples:\n",
                            "`dd if=/dev/sda of=usb_dump.bin`: Copy the contents of the device",
                            "`/dev/sda` to the file `usb_dump.bin`\n",
                            "`dd if=/dev/zero of=zeroes.bin bs=1M count=1`: Create a file",
                            "`zeroes.bin` filled with zeroes, with a size of 1 megabyte\n",
                            "`dd if=/dev/random of=random.bin bs=1M count=1`: Create a file",
                            "`random.bin` filled with random data, with a size of 1 megabyte\n",
                        ]
                        .join(""),
                    },
                    super::example::Example {
                        input: "echo \"Hello, world!\"".to_string(),
                        output: vec![
                            "The `echo` command is commonly used to display text on the terminal.",
                            "In this case, the command will display the text `Hello, world!` on",
                            "the terminal.\n",
                            "\n",
                            "By default, the `echo` command will display the text followed by a",
                            "newline character. If you want to display the text without a newline",
                            "character, you can use the `-n` option. Additionally, by default, the",
                            "text will be displayed on the standard output (usually the terminal),",
                            "howver, you can redirect the output to a file, another command, or",
                            "the standard error using redirection operators.\n",
                            "\n",
                            "Command usage:\n",
                            "echo [OPTION] [STRING]\n",
                            "\n",
                            "Common options:\n",
                            "`-n`: Do not output the trailing newline\n",
                            "\n",
                            "Examples:\n",
                            "`echo \"Hello, world!\"`: Display `Hello, world!` followed by a",
                            "\tnewline character\n",
                        ]
                        .join(""),
                    },
                    super::example::Example {
                        input: "afhhi12412k".to_string(),
                        output: vec![
                            "The command `afhhi12412k` is not a valid command, and no close",
                            "matches were found. Please check the command and try again.",
                        ]
                        .join(""),
                    },
                ]
                .into_iter()
                .map(|example| example.to_string())
                .collect::<Vec<String>>()
                .join("\n");
                format!(
                    "{}\n\nQuestion and Answer Examples:{}",
                    instructions, examples
                )
            }
            ModelContext::ShellGenerateContext => {
                let instructions = vec![
                    "You are part of a command-line interface that provides command generation for shell",
                    "commands. You will get a description of a command and you need to provide the full",
                    "command. The description will include the name of the command, as well as any",
                    "arguments or options that are required. You should generate a command that matches",
                    "the description as closely as possible.",
                    "If you are unable to generate a command based on the description, you can include",
                    "[NULL] in your response to indicate that you do not have a valid command. To be",
                    "clear, any response containing [NULL] will be treated as a null response, regardless",
                    "of any other text in the response. Any response that is not a valid command and does",
                    "not contain [NULL] will be rejected.",
                    "You might receive requests containing popular command line programs, you must provide",
                    "generation for those as well, even though they're not built-in commands.",
                ]
                .join(" ");
                let examples = vec![
                    super::example::Example {
                        input: "List all files and directories in the /home/user/.config directory in long format".to_string(),
                        output: "ls -l /home/user/.config".to_string(),
                    },
                    super::example::Example {
                        input: "Copy the contents of the device /dev/sda to the file usb_dump.bin".to_string(),
                        output: "dd if=/dev/sda of=usb_dump.bin".to_string(),
                    },
                    super::example::Example {
                        input: "Display Hello, world!".to_string(),
                        output: "echo \"Hello, world!\"".to_string(),
                    },
                    super::example::Example {
                        input: "Create a file filled with random data, with a size of 1 megabyte".to_string(),
                        output: "dd if=/dev/random of=random.bin bs=1M count=1".to_string(),
                    },
                    super::example::Example {
                        input: "List all files and directories in the /home/user/.config directory in long format".to_string(),
                        output: "ls -l /home/user/.config".to_string(),
                    },
                ].into_iter().map(|example| example.to_string()).collect::<Vec<String>>().join("\n");
                format!("{}\n\nExamples:{}", instructions, examples)
            }
        }
    }
}
