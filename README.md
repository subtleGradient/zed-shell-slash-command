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

- macOS (tested)
- Linux (untested)

Windows support is not currently planned.

## Installation

Install it the normal way that zed extensions are installed.
