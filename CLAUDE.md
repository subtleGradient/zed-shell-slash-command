# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Zed editor extension that adds a `/sh` slash command, allowing users to run shell commands directly from the Zed Assistant panel and see their output.

## Build and Development Commands

```bash
# Build the extension (ready to install in Zed)
make

# Enter Nix development shell (if not already in it)
make dev-shell

# Other useful commands
make clean     # Clean build artifacts
make test      # Run tests
make lint      # Check for linting issues
make fmt       # Format code
make check     # Run all checks (fmt, lint, test, build)
make watch     # Watch for changes and rebuild
```

### Direct Cargo Commands (if preferred)

```bash
# Build WASM extension manually
cargo build --release --target wasm32-wasi
```

## Architecture

### Core Components

1. **Extension Entry Point** (`src/lib.rs`):
   - Implements `zed::Extension` trait
   - Main handler: `run_slash_command()` executes shell commands via `process::Command`
   - Handles command execution in worktree context when available
   - Provides command argument completions

2. **Extension Configuration** (`extension.toml`):
   - Defines the `/sh` slash command
   - Configures process execution capability for `sh -c *`

### Key Implementation Details

- Commands run with `sh -c` on Unix-like systems (macOS/Linux)
- When a worktree is available, commands execute in the project root directory
- Output includes both stdout and stderr
- Exit codes are displayed when non-zero
- Command completions provide common shell commands as suggestions

### Extension API Usage

This extension uses the Zed Extension API v0.3.0:
- `SlashCommand` for handling the `/sh` command
- `process::Command` for executing shell commands
- `Worktree` for accessing project context
- `SlashCommandOutputSection` for formatting output with labels