use zed_extension_api::{
    self as zed, process, Result, SlashCommand, SlashCommandOutput, SlashCommandOutputSection,
    Worktree,
};

struct ShellCommandExtension;

impl zed::Extension for ShellCommandExtension {
    fn new() -> Self {
        Self
    }

    fn run_slash_command(
        &self,
        command: SlashCommand,
        args: Vec<String>,
        worktree: Option<&Worktree>,
    ) -> Result<SlashCommandOutput, String> {
        match command.name.as_str() {
            "sh" => {
                if args.is_empty() {
                    return Err("Please provide a command to run".to_string());
                }

                let command_str = args.join(" ");
                let mut cmd;

                // TODO: Add Windows support
                // if cfg!(windows) {
                //     return Err("Windows support is not yet implemented".to_string());
                // }

                // Check if we have a worktree and use it for the command context
                if let Some(wt) = worktree {
                    let worktree_path = wt.root_path();
                    let cd_command = format!("cd \"$WORKTREE_ROOT\" && {}", command_str);
                    cmd = process::Command::new("sh")
                        .env("WORKTREE_ROOT", worktree_path)
                        .arg("-c")
                        .arg(cd_command);
                } else {
                    // No worktree context, run the command directly
                    cmd = process::Command::new("sh").arg("-c").arg(&command_str);
                }

                // Execute the command and get output
                let output = cmd.output()?;

                // Convert Vec<u8> to String for both stdout and stderr
                let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                let stderr = String::from_utf8_lossy(&output.stderr).to_string();

                // Format the command output
                let mut result = format!("$ {}\n", command_str);
                result.push_str("```\n");

                if output.status != Some(0) {
                    result.push_str(&format!(
                        "⚠ Command exited with status code {:?}\n\n",
                        output.status
                    ));
                }

                if !stdout.is_empty() {
                    result.push_str(&stdout);
                }

                if !stderr.is_empty() {
                    if !stdout.is_empty() && !stdout.ends_with('\n') {
                        result.push('\n');
                    }
                    result.push_str(&stderr);
                }

                if stdout.is_empty() && stderr.is_empty() {
                    result.push_str("(No output)");
                }

                result.push_str("```");

                Ok(SlashCommandOutput {
                    sections: vec![SlashCommandOutputSection {
                        range: (0..result.len()).into(),
                        label: format!("$ {}", command_str),
                    }],
                    text: result,
                })
            }
            _ => Err(format!("Unknown slash command: {}", command.name)),
        }
    }

    fn complete_slash_command_argument(
        &self,
        command: SlashCommand,
        args: Vec<String>,
    ) -> Result<Vec<zed::SlashCommandArgumentCompletion>, String> {
        // Provide some common command completions if this is the first argument
        if command.name == "sh" && args.len() <= 1 {
            let query = args.get(0).cloned().unwrap_or_default().to_lowercase();
            let completions = vec![
                ("ls", "List directory contents"),
                ("git status", "Show git working tree status"),
                ("git log", "Show commit logs"),
                ("git branch", "List branches"),
                ("pwd", "Print working directory"),
                ("cat", "Print file contents"),
                ("grep", "Search for patterns"),
                ("find", "Search for files"),
                ("ps", "Process status"),
                ("echo", "Display a message"),
            ];

            return Ok(completions
                .into_iter()
                .filter(|(cmd, _)| cmd.contains(&query))
                .map(|(cmd, description)| zed::SlashCommandArgumentCompletion {
                    label: format!("{} - {}", cmd, description),
                    new_text: cmd.to_string(),
                    run_command: false,
                })
                .collect());
        }

        Ok(vec![])
    }
}

zed::register_extension!(ShellCommandExtension);
