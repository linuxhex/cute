<p align="center">
  <img width="128" alt="Cute Terminal" src="https://raw.githubusercontent.com/linuxhex/cute/main/app/assets/cute.png" />
</p>

<h1 align="center">Cute</h1>

<p align="center">
  A modern terminal for developers
</p>

## About

Cute is a modern, GPU-accelerated terminal emulator built with Rust. It combines a fast, native terminal experience with powerful developer features including AI-assisted coding, intelligent autocompletion, and a rich Git integration.

## Features

### Terminal

- **GPU-accelerated rendering** — smooth scrolling and responsive UI powered by custom wgpu-based renderer
- **Split panes** — organize your workspace with horizontal and vertical splits
- **Tabs** — manage multiple terminal sessions with a modern tab interface
- **Rich theming** — customize colors, fonts, and appearance to your preference
- **Ligature support** — beautiful font rendering with programming ligatures

### AI Agent

- **Built-in coding agent** — interact with AI directly from your terminal
- **Multi-agent support** — bring your own CLI agent (Claude Code, Qoder, Trae, and more)
- **Context-aware** — AI understands your terminal output, file system, and git context
- **Agent mode** — full-featured AI conversation with file editing, command execution, and more

### Code Review & Git

- **Diff viewer** — review file changes with syntax-highlighted diff views
- **Branch management** — visual branch selector with commit history graph
- **Commit history** — browse commit logs with file change details
- **Git operations** — stage, commit, push, and create PRs from the UI

### Autocomplete

- **Command suggestions** — intelligent, context-aware completions for shell commands
- **Path completion** — smart file and directory path suggestions
- **History search** — fuzzy search through your command history

### Editor

- **Built-in code editor** — edit files directly within the terminal
- **Syntax highlighting** — support for multiple programming languages
- **LSP integration** — language server protocol support for intelligent code editing

## Installation

### Download

Download the latest release from the [Releases](https://github.com/linuxhex/cute/releases) page.

### Build from Source

```bash
# Prerequisites
./script/bootstrap   # platform-specific setup (run once)

# Build and run
./script/run         # build and run Cute (recommended)

# Development checks
./script/presubmit   # fmt, clippy, and tests
```

**Important:**
- `./script/run` is the **recommended way** to run the app — it builds a proper `.app` bundle and opens it
- Running `cargo run --bin cute` directly may not work correctly due to missing bundle resources

## Licensing

Cute's UI framework is licensed under the [MIT license](LICENSE-MIT).

The rest of the code in this repository is licensed under the [AGPL v3](LICENSE-AGPL).

## Open Source Dependencies

Cute is built on top of many great open source projects:

- [Tokio](https://github.com/tokio-rs/tokio)
- [NuShell](https://github.com/nushell/nushell)
- [Fig Completion Specs](https://github.com/withfig/autocomplete)
- [Alacritty](https://github.com/alacritty/alacritty)
- [Hyper HTTP library](https://github.com/hyperium/hyper)
- [FontKit](https://github.com/servo/font-kit)
- [Core-foundation](https://github.com/servo/core-foundation-rs)
- [Smol](https://github.com/smol-rs/smol)