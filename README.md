# Shell Command Extension for Zed

This Zed extension adds a `/sh` slash command that allows you to run shell commands directly from Zed and see their output.

## Usage

In the Assistant panel, use the `/sh` slash command followed by the shell command you want to run:

```
/sh ls -la
```

The output of the command will be displayed in the Assistant panel.

## Features

- Run shell commands directly from Zed
- See both stdout and stderr output
- Commands run in the current project directory if available
- Command completions for common shell commands

## Compatibility

Currently supports:
- macOS
- Linux

Windows support is planned for future releases.

## Installation

### Option 1: Install from Extension Registry (once published)
1. Open Zed
2. Go to Settings > Extensions
3. Search for "Shell Command"
4. Click "Install"

### Option 2: Install from Source
1. Clone this repository
2. Build using `cargo build --release`
3. Install the extension in Zed by going to Settings > Extensions > Install From Path

## Security Note

This extension runs commands directly on your system. Be careful with what commands you run, especially if you're working with untrusted content.
